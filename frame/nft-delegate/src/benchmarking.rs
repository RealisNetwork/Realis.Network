#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::Pallet as NftDelegate;
    use pallet_nft::Pallet as Nft;
    use crate::*;
    use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
    use frame_system::RawOrigin as SystemOrigin;
    use primitive_types::U256;
    use realis_primitives::*;
    use pallet_nft::NftMasters;

    const SEED: u32 = 1;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    fn bob<T: Config>() -> T::AccountId {
        let bob = NftMasters::<T>::get();
        bob.get(1).unwrap().clone()
    }

    benchmarks! {
        delegate {
            let caller = alice::<T>();
            let account_to = bob::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            account_to,
            U256([1, 0, 0, 0]),
            10
        )

        sell_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
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
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        change_price_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
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
            200
        )

        change_delegate_time_on_sale {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
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
            200
        )

        remove_from_sell {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        remove_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )
    }
}