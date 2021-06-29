#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::Pallet as RealisGameApi;
    use crate::{*};
    use frame_benchmarking::{benchmarks, account};
    use frame_system::RawOrigin as SystemOrigin;
	use pallet_nft as Nft;
	use frame_support::traits::Get;
	use sp_runtime::traits::Saturating;

    const SEED: u32 = 1;

	const ED_MULTIPLIER: u32 = 10;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = Nft::NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    // fn addToken<T: Config>() -> Result<(), >

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
			let b in 1 .. 1000;
			RealisGameApi::<T>::mint_basic_nft(
				owner_origin,
				caller.clone(),
				Nft::U256([b.into(), 0, 0, 0]),
				Nft::Types { tape: 1 }
			);
		}: _(
			SystemOrigin::Signed(caller.clone()),
			Nft::U256([b.into(), 0, 0, 0])
		)

        transfer_basic_nft {
			let caller = alice::<T>();
			let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
			let recipient: T::AccountId = account("recipient", 1, SEED);
			let b in 1 .. 1000;
			RealisGameApi::<T>::mint_basic_nft(
				owner_origin,
				caller.clone(),
				Nft::U256([b.into(), 0, 0, 0]),
				Nft::Types { tape: 1 }
			);
		}: _(
			SystemOrigin::Signed(caller.clone()),
			recipient,
			Nft::U256([b.into(), 0, 0, 0])
		)

		transfer_from_pallet {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);
			let pallet_id = RealisGameApi::<T>::account_id();
			let b in 1 .. 1000;

			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();

			ReaslisGameApi::<T>::transfer_to_pallet()
		}: _(
			owner_origin,
			recipient,
			transfer_amount
		)

		transfer_to_pallet {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
			let recipient: T::AccountId = account("recipient", 1, SEED);
			let b in 1 .. 1000;
			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			recipient,
			transfer_amount
			// 10000 as Nft::Config::Balance
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
			let b in 1 .. 1000;
			let existential_deposit = T::ExistentialDeposit::get();
			let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
		}: _(
			owner_origin,
			recipient,
			transfer_amount
			// 10000 as Nft::Config::Balance
		)

		balance_pallet {
			let caller = alice::<T>();
			let owner_origin = SystemOrigin::Signed(caller.clone());
		}: _(
			owner_origin
		)
	}
}