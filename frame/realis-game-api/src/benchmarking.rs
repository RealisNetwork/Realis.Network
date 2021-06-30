#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
	use crate::Pallet as RealisGameApi;
	use crate::{*};
	use pallet_nft as Nft;


	use frame_benchmarking::{benchmarks, account};
	use frame_system::RawOrigin as SystemOrigin;
	use frame_support::traits::{Get, Currency};
	use sp_runtime::traits::{Saturating, StaticLookup};

	const SEED: u32 = 1;
    const ED_MULTIPLIER: u32 = 10;
    const ED_MULTIPLIER_2: u32 = 5;

	// Get Alice AccountId
	fn alice<T: Config>() -> T::AccountId {
		let alice = Nft::NftMasters::<T>::get();
		alice.get(0).unwrap().clone()
	}

	benchmarks! {
        mint_basic_nft {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
        }: _(
            owner_origin,
            caller.clone(),
            Nft::U256([1, 0, 0, 0]),
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

        transfer_from_pallet {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
            let existential_deposit = T::ExistentialDeposit::get();
            let recipient: T::AccountId = account("recipient", 1, SEED);

            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            owner_origin,
            recipient,
            transfer_amount
        )

        transfer_to_pallet {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
            let existential_deposit = T::ExistentialDeposit::get();
            let recipient: T::AccountId = account("recipient", 1, SEED);

            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
            }: _(
                owner_origin,
                caller,
                transfer_amount
            )

        transfer_from_ptp {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
            let existential_deposit = T::ExistentialDeposit::get();
            let recipient: T::AccountId = account("recipient", 1, SEED);

            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            owner_origin,
            caller,
            recipient,
            transfer_amount
        )

        spend_in_game {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
            let existential_deposit = T::ExistentialDeposit::get();
            let recipient: T::AccountId = account("recipient", 1, SEED);

            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            owner_origin,
            recipient,
            transfer_amount
        )
  //
  //   balance_pallet {
  //     let caller = alice::<T>();
  //     let owner_origin = SystemOrigin::Signed(caller.clone());
  //   }: _(
  //     owner_origin
  //   )
  }
}