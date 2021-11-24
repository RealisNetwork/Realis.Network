#![cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use pallet_nft::Pallet as Nft;
    use pallet::Pallet as Marketplace;
    use crate::*;
    use frame_benchmarking::benchmarks;
    use frame_system::RawOrigin as SystemOrigin;
    use primitive_types::U256;
    use realis_primitives::*;
    use pallet_nft::NftMasters;
    use frame_support::traits::Currency;

    const ED_MULTIPLIER: u128 = 1_000_000_000_000_000;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        sell_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::Currency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::Currency::make_free_balance_be(&caller, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0]),
            10
        )

        buy_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::Currency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::Currency::make_free_balance_be(&caller, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            Marketplace::<T>::sell_nft (
                owner_origin,
                U256([1, 0, 0, 0]),
                10
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        change_price_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::Currency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::Currency::make_free_balance_be(&caller, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            Marketplace::<T>::sell_nft (
                owner_origin,
                U256([1, 0, 0, 0]),
                10
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0]),
            5
        )

        remove_from_marketplace_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::Currency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::Currency::make_free_balance_be(&caller, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            Marketplace::<T>::sell_nft (
                owner_origin,
                U256([1, 0, 0, 0]),
                10
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )
    }
}