// --- crates ---
use codec::{Decode, Encode};
// --- substrate ---
use frame_support::{
	ensure,
	traits::{Currency, Imbalance, OnUnbalanced},
	StorageDoubleMap, StorageMap,
};
use sp_runtime::{
	traits::{Saturating, Zero},
	DispatchResult, Perbill, RuntimeDebug,
};
use sp_std::{
	ops::{Add, AddAssign, Sub},
	prelude::*,
};
// --- darwinia ---
use crate::*;

/// The proportion of the slashing reward to be paid out on the first slashing detection.
/// This is f_1 in the paper.
const REWARD_F1: Perbill = Perbill::from_percent(50);

/// The index of a slashing span - unique to each stash.
pub type SpanIndex = u32;

pub(crate) type RT<T> = C<CurBalance<T>>;

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
pub struct C<U> {
	 r: U,
}
impl<U> Zero for C<U>
where
U: Zero,
{
	fn zero() -> Self {
		Self {
			r: Zero::zero(),
		}
	}

	fn set_zero(&mut self) {
		self.r = Zero::zero();
	}

	fn is_zero(&self) -> bool {
		self.r.is_zero()
	}
}
impl<U> Add for C<U>
where
U: Add<Output = U>,
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			r: self.r + rhs.r,
		}
	}
}
impl<U> AddAssign for C<U>
where
U: AddAssign,
{
	fn add_assign(&mut self, rhs: Self) {
		self.r += rhs.r;
	}
}
impl<U> Sub for C<U>
where
U: Sub<Output = U>,
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			r: self.r - rhs.r,
		}
	}
}
impl<U> Saturating for C<U>
where
U: Copy + Saturating,
{
	fn saturating_add(self, o: Self) -> Self {
		Self {
			r: self.r.saturating_add(o.r),
		}
	}

	fn saturating_sub(self, o: Self) -> Self {
		Self {
			r: self.r.saturating_sub(o.r),
		}
	}

	fn saturating_mul(self, o: Self) -> Self {
		Self {
			r: self.r.saturating_mul(o.r),
		}
	}

	fn saturating_pow(self, exp: usize) -> Self {
		Self {
			r: self.r.saturating_pow(exp),
		}
	}
}

// A range of start..end eras for a slashing span.
#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct SlashingSpan {
	pub(crate) index: SpanIndex,
	pub(crate) start: EraIndex,
	pub(crate) length: Option<EraIndex>, // the ongoing slashing span has indeterminate length.
}

impl SlashingSpan {
	fn contains_era(&self, era: EraIndex) -> bool {
		self.start <= era && self.length.map_or(true, |l| self.start + l > era)
	}
}

/// An encoding of all of a nominator's slashing spans.
#[derive(Encode, Decode, RuntimeDebug)]
pub struct SlashingSpans {
	// the index of the current slashing span of the nominator. different for
	// every stash, resets when the account hits free balance 0.
	span_index: SpanIndex,
	// the start era of the most recent (ongoing) slashing span.
	last_start: EraIndex,
	// the last era at which a non-zero slash occurred.
	last_nonzero_slash: EraIndex,
	// all prior slashing spans' start indices, in reverse order (most recent first)
	// encoded as offsets relative to the slashing span after it.
	prior: Vec<EraIndex>,
}

impl SlashingSpans {
	// creates a new record of slashing spans for a stash, starting at the beginning
	// of the bonding period, relative to now.
	pub(crate) fn new(window_start: EraIndex) -> Self {
		SlashingSpans {
			span_index: 0,
			last_start: window_start,
			// initialize to zero, as this structure is lazily created until
			// the first slash is applied. setting equal to `window_start` would
			// put a time limit on nominations.
			last_nonzero_slash: 0,
			prior: vec![],
		}
	}

	// update the slashing spans to reflect the start of a new span at the era after `now`
	// returns `true` if a new span was started, `false` otherwise. `false` indicates
	// that internal state is unchanged.
	pub(crate) fn end_span(&mut self, now: EraIndex) -> bool {
		let next_start = now + 1;
		if next_start <= self.last_start {
			return false;
		}

		let last_length = next_start - self.last_start;
		self.prior.insert(0, last_length);
		self.last_start = next_start;
		self.span_index += 1;
		true
	}

	// an iterator over all slashing spans in _reverse_ order - most recent first.
	pub(crate) fn iter(&'_ self) -> impl Iterator<Item = SlashingSpan> + '_ {
		let mut last_start = self.last_start;
		let mut index = self.span_index;
		let last = SlashingSpan {
			index,
			start: last_start,
			length: None,
		};
		let prior = self.prior.iter().cloned().map(move |length| {
			let start = last_start - length;
			last_start = start;
			index -= 1;

			SlashingSpan {
				index,
				start,
				length: Some(length),
			}
		});

		sp_std::iter::once(last).chain(prior)
	}

	/// Yields the era index where the most recent non-zero slash occurred.
	pub fn last_nonzero_slash(&self) -> EraIndex {
		self.last_nonzero_slash
	}

	// prune the slashing spans against a window, whose start era index is given.
	//
	// If this returns `Some`, then it includes a range start..end of all the span
	// indices which were pruned.
	fn prune(&mut self, window_start: EraIndex) -> Option<(SpanIndex, SpanIndex)> {
		let old_idx = self
			.iter()
			.skip(1) // skip ongoing span.
			.position(|span| {
				span.length
					.map_or(false, |len| span.start + len <= window_start)
			});

		let earliest_span_index = self.span_index - self.prior.len() as SpanIndex;
		let pruned = match old_idx {
			Some(o) => {
				self.prior.truncate(o);
				let new_earliest = self.span_index - self.prior.len() as SpanIndex;
				Some((earliest_span_index, new_earliest))
			}
			None => None,
		};

		// readjust the ongoing span, if it started before the beginning of the window.
		self.last_start = sp_std::cmp::max(self.last_start, window_start);
		pruned
	}
}

/// A slashing-span record for a particular stash.
#[derive(Encode, Decode, Default)]
pub(crate) struct SpanRecord<CurBalance> {
	slashed: C<CurBalance>,
	paid_out: C<CurBalance>,
}

impl<CurBalance> SpanRecord<CurBalance> {
	/// The value of stash balance slashed in this span.
	#[cfg(test)]
	pub(crate) fn amount_slashed(&self) -> &C<CurBalance> {
		&self.slashed
	}
}

/// Parameters for performing a slash.
#[derive(Clone)]
pub(crate) struct SlashParams<'a, T: 'a + Config> {
	/// The stash account being slashed.
	pub(crate) stash: &'a T::AccountId,
	/// The proportion of the slash.
	pub(crate) slash: Perbill,
	/// The exposure of the stash and all nominators.
	pub(crate) exposure: &'a Exposure<T::AccountId, CurBalance<T>>,
	/// The era where the offence occurred.
	pub(crate) slash_era: EraIndex,
	/// The first era in the current bonding period.
	pub(crate) window_start: EraIndex,
	/// The current era.
	pub(crate) now: EraIndex,
	/// The maximum percentage of a slash that ever gets paid out.
	/// This is f_inf in the paper.
	pub(crate) reward_proportion: Perbill,
}

/// Computes a slash of a validator and nominators. It returns an unapplied
/// record to be applied at some later point. Slashing metadata is updated in storage,
/// since unapplied records are only rarely intended to be dropped.
///
/// The pending slash record returned does not have initialized reporters. Those have
/// to be set at a higher level, if any.
pub(crate) fn compute_slash<T: Config>(
	params: SlashParams<T>,
) -> Option<UnappliedSlash<T::AccountId, CurBalance<T>>> {
	let SlashParams {
		stash,
		slash,
		exposure,
		slash_era,
		window_start,
		now,
		reward_proportion,
	} = params.clone();

	let mut reward_payout = Zero::zero();
	let mut val_slashed = Zero::zero();

	// is the slash amount here a maximum for the era?
	let own_slash = C {
		r: slash * exposure.own_cur_balance,
	};
	if (slash * exposure.total_power).is_zero() {
		// kick out the validator even if they won't be slashed,
		// as long as the misbehavior is from their most recent slashing span.
		kick_out_if_recent::<T>(params);
		return None;
	}

	let (prior_slash_p, _era_slash) =
		<Module<T> as Store>::ValidatorSlashInEra::get(&slash_era, stash)
			.unwrap_or((Perbill::zero(), Zero::zero()));

	// compare slash proportions rather than slash values to avoid issues due to rounding
	// error.
	if slash.deconstruct() > prior_slash_p.deconstruct() {
		<Module<T> as Store>::ValidatorSlashInEra::insert(&slash_era, stash, &(slash, own_slash));
	} else {
		// we slash based on the max in era - this new event is not the max,
		// so neither the validator or any nominators will need an update.
		//
		// this does lead to a divergence of our system from the paper, which
		// pays out some reward even if the latest report is not max-in-era.
		// we opt to avoid the nominator lookups and edits and leave more rewards
		// for more drastic misbehavior.
		return None;
	}

	// apply slash to validator.
	{
		let mut spans = fetch_spans::<T>(
			stash,
			window_start,
			&mut reward_payout,
			&mut val_slashed,
			reward_proportion,
		);

		let target_span = spans.compare_and_update_span_slash(slash_era, own_slash);

		if target_span == Some(spans.span_index()) {
			// misbehavior occurred within the current slashing span - take appropriate
			// actions.

			// chill the validator - it misbehaved in the current span and should
			// not continue in the next election. also end the slashing span.
			spans.end_span(now);
			<Module<T>>::chill_stash(stash);

			// make sure to disable validator till the end of this session
			if T::SessionInterface::disable_validator(stash).unwrap_or(false) {
				// force a new era, to select a new validator set
				<Module<T>>::ensure_new_era()
			}
		}
	}

	let mut nominators_slashed = vec![];
	reward_payout += slash_nominators::<T>(params, prior_slash_p, &mut nominators_slashed);

	Some(UnappliedSlash {
		validator: stash.clone(),
		own: val_slashed,
		others: nominators_slashed,
		reporters: vec![],
		payout: reward_payout,
	})
}

// doesn't apply any slash, but kicks out the validator if the misbehavior is from
// the most recent slashing span.
fn kick_out_if_recent<T: Config>(params: SlashParams<T>) {
	// these are not updated by era-span or end-span.
	let mut reward_payout = C::zero();
	let mut val_slashed = C::zero();
	let mut spans = fetch_spans::<T>(
		params.stash,
		params.window_start,
		&mut reward_payout,
		&mut val_slashed,
		params.reward_proportion,
	);

	if spans.era_span(params.slash_era).map(|s| s.index) == Some(spans.span_index()) {
		spans.end_span(params.now);
		<Module<T>>::chill_stash(params.stash);

		// make sure to disable validator till the end of this session
		if T::SessionInterface::disable_validator(params.stash).unwrap_or(false) {
			// force a new era, to select a new validator set
			<Module<T>>::ensure_new_era()
		}
	}
}

/// Slash nominators. Accepts general parameters and the prior slash percentage of the validator.
///
/// Returns the amount of reward to pay out.
fn slash_nominators<T: Config>(
	params: SlashParams<T>,
	prior_slash_p: Perbill,
	nominators_slashed: &mut Vec<(T::AccountId, RT<T>)>,
) -> RT<T> {
	let SlashParams {
		stash: _,
		slash,
		exposure,
		slash_era,
		window_start,
		now,
		reward_proportion,
	} = params;

	let mut reward_payout = Zero::zero();

	nominators_slashed.reserve(exposure.others.len());
	for nominator in &exposure.others {
		let stash = &nominator.who;
		let mut nom_slashed = Zero::zero();

		// the era slash of a nominator always grows, if the validator
		// had a new max slash for the era.
		let era_slash = {
			let own_slash_prior = C {
				r: prior_slash_p * nominator.cur_balance,
			};
			let own_slash_by_validator = C {
				r: slash * nominator.cur_balance,
			};
			let own_slash_difference = own_slash_by_validator.saturating_sub(own_slash_prior);

			let mut era_slash = <Module<T> as Store>::NominatorSlashInEra::get(&slash_era, stash)
				.unwrap_or_else(|| Zero::zero());

			era_slash += own_slash_difference;

			<Module<T> as Store>::NominatorSlashInEra::insert(&slash_era, stash, &era_slash);

			era_slash
		};

		// compare the era slash against other eras in the same span.
		{
			let mut spans = fetch_spans::<T>(
				stash,
				window_start,
				&mut reward_payout,
				&mut nom_slashed,
				reward_proportion,
			);

			let target_span = spans.compare_and_update_span_slash(slash_era, era_slash);

			if target_span == Some(spans.span_index()) {
				// End the span, but don't chill the nominator. its nomination
				// on this validator will be ignored in the future.
				spans.end_span(now);
			}
		}

		nominators_slashed.push((stash.clone(), nom_slashed));
	}

	reward_payout
}

// helper struct for managing a set of spans we are currently inspecting.
// writes alterations to disk on drop, but only if a slash has been carried out.
//
// NOTE: alterations to slashing metadata should not be done after this is dropped.
// dropping this struct applies any necessary slashes, which can lead to free balance
// being 0, and the account being garbage-collected -- a dead account should get no new
// metadata.
struct InspectingSpans<'a, T: Config + 'a> {
	dirty: bool,
	window_start: EraIndex,
	stash: &'a T::AccountId,
	spans: SlashingSpans,
	paid_out: &'a mut RT<T>,
	slash_of: &'a mut RT<T>,
	reward_proportion: Perbill,
}

// fetches the slashing spans record for a stash account, initializing it if necessary.
fn fetch_spans<'a, T: Config + 'a>(
	stash: &'a T::AccountId,
	window_start: EraIndex,
	paid_out: &'a mut RT<T>,
	slash_of: &'a mut RT<T>,
	reward_proportion: Perbill,
) -> InspectingSpans<'a, T> {
	let spans = <Module<T> as Store>::SlashingSpans::get(stash).unwrap_or_else(|| {
		let spans = SlashingSpans::new(window_start);
		<Module<T> as Store>::SlashingSpans::insert(stash, &spans);
		spans
	});

	InspectingSpans {
		dirty: false,
		window_start,
		stash,
		spans,
		slash_of,
		paid_out,
		reward_proportion,
	}
}

impl<'a, T: 'a + Config> InspectingSpans<'a, T> {
	fn span_index(&self) -> SpanIndex {
		self.spans.span_index
	}

	fn end_span(&mut self, now: EraIndex) {
		self.dirty = self.spans.end_span(now) || self.dirty;
	}

	// add some value to the slash of the staker.
	// invariant: the staker is being slashed for non-zero value here
	// although `amount` may be zero, as it is only a difference.
	fn add_slash(&mut self, amount: RT<T>, slash_era: EraIndex) {
		*self.slash_of += amount;
		self.spans.last_nonzero_slash = sp_std::cmp::max(self.spans.last_nonzero_slash, slash_era);
	}

	// find the span index of the given era, if covered.
	fn era_span(&self, era: EraIndex) -> Option<SlashingSpan> {
		self.spans.iter().find(|span| span.contains_era(era))
	}

	// compares the slash in an era to the overall current span slash.
	// if it's higher, applies the difference of the slashes and then updates the span on disk.
	//
	// returns the span index of the era where the slash occurred, if any.
	fn compare_and_update_span_slash(
		&mut self,
		slash_era: EraIndex,
		slash: RT<T>,
	) -> Option<SpanIndex> {
		let target_span = self.era_span(slash_era)?;
		let span_slash_key = (self.stash.clone(), target_span.index);
		let mut span_record = <Module<T> as Store>::SpanSlash::get(&span_slash_key);
		let mut changed = false;

		let reward = if span_record.slashed < slash {
			// new maximum span slash. apply the difference.
			let difference = slash - span_record.slashed;
			span_record.slashed = slash;

			// compute reward.
			let slash = C {
				r: self.reward_proportion * slash.r,
			};
			let slash = slash.saturating_sub(span_record.paid_out);
			let reward = C {
				r: REWARD_F1 * slash.r,
			};

			self.add_slash(difference, slash_era);
			changed = true;

			reward
		} else if span_record.slashed == slash {
			// compute reward. no slash difference to apply.
			let slash = C {
				r: self.reward_proportion * slash.r,
			};
			let slash = slash.saturating_sub(span_record.paid_out);
			C {
				r: REWARD_F1 * slash.r,
			}
		} else {
			Zero::zero()
		};

		if !reward.is_zero() {
			changed = true;
			span_record.paid_out += reward;
			*self.paid_out += reward;
		}

		if changed {
			self.dirty = true;
			<Module<T> as Store>::SpanSlash::insert(&span_slash_key, &span_record);
		}

		Some(target_span.index)
	}
}

impl<'a, T: 'a + Config> Drop for InspectingSpans<'a, T> {
	fn drop(&mut self) {
		// only update on disk if we slashed this account.
		if !self.dirty {
			return;
		}

		if let Some((start, end)) = self.spans.prune(self.window_start) {
			for span_index in start..end {
				<Module<T> as Store>::SpanSlash::remove(&(self.stash.clone(), span_index));
			}
		}

		<Module<T> as Store>::SlashingSpans::insert(self.stash, &self.spans);
	}
}

/// Clear slashing metadata for an obsolete era.
pub(crate) fn clear_era_metadata<T: Config>(obsolete_era: EraIndex) {
	<Module<T> as Store>::ValidatorSlashInEra::remove_prefix(&obsolete_era);
	<Module<T> as Store>::NominatorSlashInEra::remove_prefix(&obsolete_era);
}

/// Clear slashing metadata for a dead account.
pub(crate) fn clear_stash_metadata<T: Config>(
	stash: &T::AccountId,
	num_slashing_spans: u32,
) -> DispatchResult {
	let spans = match <Module<T> as Store>::SlashingSpans::get(stash) {
		None => return Ok(()),
		Some(s) => s,
	};

	ensure!(
		num_slashing_spans as usize >= spans.iter().count(),
		<Error<T>>::IncorrectSlashingSpans
	);

	<Module<T> as Store>::SlashingSpans::remove(stash);

	// kill slashing-span metadata for account.
	//
	// this can only happen while the account is staked _if_ they are completely slashed.
	// in that case, they may re-bond, but it would count again as span 0. Further ancient
	// slashes would slash into this new bond, since metadata has now been cleared.
	for span in spans.iter() {
		<Module<T> as Store>::SpanSlash::remove(&(stash.clone(), span.index));
	}

	Ok(())
}

// apply the slash to a stash account, deducting any missing funds from the reward
// payout, saturating at 0. this is mildly unfair but also an edge-case that
// can only occur when overlapping locked funds have been slashed.
pub fn do_slash<T: Config>(
	stash: &T::AccountId,
	value: RT<T>,
	reward_payout: &mut RT<T>,
	slashed_cur: &mut CurNegativeImbalance<T>,
) {
	let controller = match <Module<T>>::bonded(stash) {
		None => return, // defensive: should always exist.
		Some(c) => c,
	};
	let mut ledger = match <Module<T>>::ledger(&controller) {
		Some(ledger) => ledger,
		None => return, // nothing to do.
	};
	let slash_cur = ledger.slash(
		value.r,
		<frame_system::Pallet<T>>::block_number(),
		T::UnixTime::now().as_millis().saturated_into::<TsInMs>(),
	);
	let mut slashed = false;

	if !slash_cur.is_zero() {
		slashed = true;

		let (imbalance, missing) = T::CurCurrency::slash(stash, slash_cur);

		slashed_cur.subsume(imbalance);

		if !missing.is_zero() {
			// deduct overslash from the reward payout
			reward_payout.r = reward_payout.r.saturating_sub(missing);
		}
	}

	if slashed {
		<Module<T>>::update_ledger(&controller, &mut ledger);
		<Module<T>>::deposit_event(RawEvent::Slash(stash.clone(), value.r));
	}
}

/// Apply a previously-unapplied slash.
pub(crate) fn apply_slash<T: Config>(
	unapplied_slash: UnappliedSlash<T::AccountId, CurBalance<T>>,
) {
	let mut slashed_cur = <CurNegativeImbalance<T>>::zero();
	let mut reward_payout = unapplied_slash.payout;

	do_slash::<T>(
		&unapplied_slash.validator,
		unapplied_slash.own,
		&mut reward_payout,
		&mut slashed_cur,
	);

	for &(ref nominator, nominator_slash) in &unapplied_slash.others {
		do_slash::<T>(
			&nominator,
			nominator_slash,
			&mut reward_payout,
			&mut slashed_cur,
		);
	}

	pay_reporters::<T>(
		reward_payout,
		slashed_cur,
		&unapplied_slash.reporters,
	);
}

/// Apply a reward payout to some reporters, paying the rewards out of the slashed imbalance.
fn pay_reporters<T: Config>(
	reward_payout: RT<T>,
	slashed_cur: CurNegativeImbalance<T>,
	reporters: &[T::AccountId],
) {
	if reporters.is_empty() || reward_payout.is_zero() {
		T::CurSlash::on_unbalanced(slashed_cur);

		return;
	}

	// take rewards out of the slashed imbalance.
	let cur_reward_payout = reward_payout.r.min(slashed_cur.peek());
	let (mut cur_reward_payout, mut cur_slashed) = slashed_cur.split(cur_reward_payout);

	let cur_per_reporter = cur_reward_payout.peek() / (reporters.len() as u32).into();

	for reporter in reporters {
		if !cur_per_reporter.is_zero() {
			let (cur_reporter_reward, cur_rest) = cur_reward_payout.split(cur_per_reporter);
			cur_reward_payout = cur_rest;

			// this cancels out the reporter reward imbalance internally, leading
			// to no change in total issuance.
			T::CurCurrency::resolve_creating(reporter, cur_reporter_reward);
		}
	}

	// the rest goes to the on-slash imbalance handler (e.g. treasury)
	cur_slashed.subsume(cur_reward_payout); // remainder of reward division remains.
	T::CurSlash::on_unbalanced(cur_slashed);
}