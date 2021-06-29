#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::Pallet as RealisGameApi;
    use crate::{*};
	use pallet_nft as Nft;


	use frame_benchmarking::{benchmarks, account};
    use frame_system::RawOrigin as SystemOrigin;
	use frame_support::traits::Get;
	use sp_runtime::traits::{Saturating};

	const SEED: u32 = 1;
	const ED_MULTIPLIER: u32 = 10;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = Nft::NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
		mint_basic_nft {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let b in 1 .. 1000;
		}: _(
			owner_origin,
			caller.clone(),
			Nft::U256([b.into(), 0, 0, 0]),
			Nft::Types { tape: 1 }
		)

		burn_basic_nft {
			let caller = alice::<T>();
			let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
			RealisGameApi::<T>::mint_basic_nft(
				owner_origin,
				caller.clone(),
				Nft::U256([1, 0, 0, 0]),
				Nft::Types { tape: 1 }
			)?;
		}: _(
			SystemOrigin::Signed(caller.clone()),
			Nft::U256([1, 0, 0, 0])
		)

        transfer_basic_nft {
			let caller = alice::<T>();
			let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
			let recipient: T::AccountId = account("recipient", 1, SEED);
			RealisGameApi::<T>::mint_basic_nft(
				owner_origin,
				caller.clone(),
				Nft::U256([1, 0, 0, 0]),
				Nft::Types { tape: 1 }
			)?;
		}: _(
			SystemOrigin::Signed(caller.clone()),
			recipient,
			Nft::U256([1, 0, 0, 0])
		)

		// TODO take out same 8 lines of code in next 3 benchmarks
		transfer_from_pallet {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);
			let pallet_id = RealisGameApi::<T>::account_id();

			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			recipient,
			transfer_amount
		)

		transfer_to_pallet {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);

			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			recipient,
			transfer_amount
		)

		transfer_from_ptp {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);

			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			caller,
			recipient,
			transfer_amount
		)

		spend_in_game {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);

			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			recipient,
			transfer_amount
		)
	//
	// 	balance_pallet {
	// 		let caller = alice::<T>();
	// 		let owner_origin = SystemOrigin::Signed(caller.clone());
	// 	}: _(
	// 		owner_origin
	// 	)
	}
}