#![cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::*;
    use frame_benchmarking::{account, benchmarks};
    use frame_support::traits::Currency;
    use frame_system::RawOrigin as SystemOrigin;
    use pallet::Pallet as NftDelegate;
    use pallet_nft::NftMasters;
    use pallet_nft::Pallet as Nft;
    use primitive_types::U256;
    use realis_primitives::*;

    const ED_MULTIPLIER: u128 = 1_000_000_000_000_000;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let balance = T::DelegateCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::DelegateCurrency::make_free_balance_be(&caller, balance);
            T::DelegateCurrency::make_free_balance_be(&buyer, balance);
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
            buyer,
            U256([1, 0, 0, 0]),
            10
        )

        sell_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
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
            10,
            100
        )

        buy_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let balance = T::DelegateCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::DelegateCurrency::make_free_balance_be(&caller, balance);
            T::DelegateCurrency::make_free_balance_be(&buyer, balance);
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
            SystemOrigin::Signed(buyer.clone()),
            U256([1, 0, 0, 0])
        )

        change_price_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let balance = T::DelegateCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::DelegateCurrency::make_free_balance_be(&caller, balance);
            T::DelegateCurrency::make_free_balance_be(&buyer, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin,
                U256([1, 0, 0, 0]),
                5,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0]),
            200
        )

        change_delegate_time_on_sale {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin,
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0]),
            200
        )

        remove_from_sell {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let balance = T::DelegateCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::DelegateCurrency::make_free_balance_be(&caller, balance);
            T::DelegateCurrency::make_free_balance_be(&buyer, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin,
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        remove_delegate {
            let caller = alice::<T>();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::DelegateCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::DelegateCurrency::make_free_balance_be(&caller, balance);
            T::DelegateCurrency::make_free_balance_be(&buyer, balance);
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin.clone(),
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
            NftDelegate::<T>::delegate (
                owner_origin,
                buyer,
                U256([1, 0, 0, 0]),
                1
            )?;
            CurrentBlock::<T>::mutate(|x| *x = T::BlockNumber::from(2_u32));
        }: _ (
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )
    }
}
