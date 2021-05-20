// This file is part of Darwinia.
//
// Copyright (C) 2018-2021 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

// --- github ---
use substrate_fixed::{
	transcendental::{pow, sqrt},
	types::I64F64,
};
// --- substrate ---
use sp_arithmetic::helpers_128bit::multiply_by_rational;
use sp_core::U256;
use sp_runtime::Perbill;
// --- darwinia ---
use crate::*;

// Milliseconds per year for the Julian year (365.25 days).
pub const MILLISECONDS_PER_YEAR: TsInMs = (366 * 24 * 60 * 60) * 1000;

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `staker-payout = yearly_inflation(npos_token_staked / total_tokens) * total_tokens / era_per_year`
/// `maximum-payout = max_yearly_inflation * total_tokens / era_per_year`
///
/// `era_duration` is expressed in millisecond.
pub fn compute_total_payout<T: Config>(
	era_duration: TsInMs,
	living_time: TsInMs,
	total_left: CurBalance<T>,
	payout_fraction: Perbill,
) -> (CurBalance<T>, CurBalance<T>) {
	log::info!(
		target: "realis-staking",
		"era_duration: {}, living_time: {}, total_left: {:?}, payout_fraction: {:?}",
		era_duration,
		living_time,
		total_left,
		payout_fraction,
	);

	let inflation = {
		let maximum = {
			let total_left = total_left.saturated_into::<Balance>();

			multiply_by_rational(total_left, era_duration as _, MILLISECONDS_PER_YEAR as _)
				.unwrap_or(0)
		};
		let year = {
			let year = living_time / MILLISECONDS_PER_YEAR + 1;

			year as u32
		};

		compute_inflation(maximum, year).unwrap_or(0)
	};
	let payout = payout_fraction * inflation;

	(
		<CurBalance<T>>::saturated_from::<Balance>(payout),
		<CurBalance<T>>::saturated_from::<Balance>(inflation),
	)
}

/// Formula:
/// 	1 - (99 / 100) ^ sqrt(year)
pub fn compute_inflation(maximum: Balance, year: u32) -> Option<u128> {
	type F64 = I64F64;

	if let Ok(a) = sqrt::<F64, F64>(F64::from_num(year)) {
		let b: F64 = F64::from_num(99) / 100;

		if let Ok(c) = pow::<F64, F64>(b, a) {
			let d: F64 = F64::from_num(1) - c;
			let e: F64 = F64::from_num(maximum) * d;

			#[cfg(test)]
			{
				let a_f64 = (year as f64).sqrt();
				// eprintln!("{}\n{}", a, a_f64);
				let b_f64 = 0.99_f64;
				// eprintln!("{}\n{}", b, b_f64);
				let c_f64 = b_f64.powf(a_f64);
				// eprintln!("{}\n{}", c, c_f64);
				let d_f64 = 1.00_f64 - c_f64;
				// eprintln!("{}\n{}", d, d_f64);
				let e_f64 = maximum as f64 * d_f64;
				// eprintln!("{}\n{}", e, e_f64);

				sp_runtime::assert_eq_error_rate!(
					e.floor(),
					e_f64 as u128,
					if e_f64 == 0.00_f64 { 0 } else { 3 }
				);
			}

			return Some(e.floor().to_num());
		} else {
			log::error!(target: "realis-staking", "Compute Inflation Failed at Step 1");
		}
	} else {
		log::error!(target: "realis-staking", "Compute Inflation Failed at Step 0");
	}

	None
}
