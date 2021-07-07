#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::Pallet as Nft;
    use crate::*;
    use primitive_types::U256;
    use realis_primitives::*;
    use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
    use frame_system::RawOrigin as SystemOrigin;

    const SEED: u32 = 1;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = crate::NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        mint {
            let caller = alice::<T>();
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            Rarity::Common,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        )

        mint_basic {
            let caller = alice::<T>();
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            1
        )

        burn {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint(
            owner_origin,
            caller.clone(),
            U256([1, 0, 0, 0]),
            Rarity::Common,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            });
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        burn_basic {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            Nft::<T>::mint_basic(
                owner_origin,
                caller.clone(),
                U256([1, 0, 0, 0]),
                1
            );
        }: _(
            SystemOrigin::Signed(caller.clone()),
            U256([1, 0, 0, 0])
        )

        transfer {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                U256([1, 0, 0, 0]),
                Rarity::Common,
                Socket::Head,
                Params {
                    strength: 1,
                    agility: 1,
                    intelligence: 1
                }
            );
        }: _(
            SystemOrigin::Signed(caller.clone()),
            recipient,
            U256([1, 0, 0, 0])
        )

        transfer_basic {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            Nft::<T>::mint_basic(
                owner_origin,
                caller.clone(),
                U256([1, 0, 0, 0]),
                1
            );
        }: _(
            SystemOrigin::Signed(caller.clone()),
            recipient,
            U256([1, 0, 0, 0])
        )

    }

    impl_benchmark_test_suite!(
        PalletModule,
        crate::mock::new_test_ext(vec![1, 2, 3]),
        crate::mock::Test
    );
}
