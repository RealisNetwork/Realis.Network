#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
    use crate::Pallet as Nft;
    use crate::*;
    use frame_benchmarking::{account, benchmarks};
    use frame_system::RawOrigin as SystemOrigin;
    use primitive_types::U256;
    use realis_primitives::*;

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
            b"QQ".to_vec(),
            U256([1, 1, 1, 0]),
            1,
            Rarity::Common,
            b"QQ".to_vec()
        )

        // burn {
        //     let caller = alice::<T>();
        //     let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
        //     Nft::<T>::mint(
        //     owner_origin,
        //     caller.clone(),
        //     U256([1, 0, 0, 0]),
        //     Rarity::Common,
        //     Socket::Head,
        //     Params {
        //         strength: 1,
        //         agility: 1,
        //         intelligence: 1
        //     });
        // }: _(
        //     SystemOrigin::Signed(caller.clone()),
        //     U256([1, 0, 0, 0])
        // )

        transfer {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            Nft::<T>::mint(
                owner_origin,
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 1, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
                )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            recipient,
            U256([1, 1, 1, 0])
        )
    }
}
