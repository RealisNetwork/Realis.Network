#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use crate::Pallet as RealisBridge;
    use crate::*;
    use frame_benchmarking::{account, benchmarks};
    use frame_support::traits::Currency;
    use frame_system::RawOrigin as SystemOrigin;
    use pallet_nft::Pallet as Nft;
    use primitive_types::U256;
    use realis_primitives::*;
    use sp_core::H160;
    use sp_runtime::traits::Saturating;

    const SEED: u32 = 1;
    const ED_MULTIPLIER: u32 = 10;
    const ED_MULTIPLIER_2: u32 = 5;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = BridgeMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        transfer_token_to_bsc {
            let caller = alice::<T>();
              let owner_origin = SystemOrigin::Signed(caller.clone());
            let balance = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::BridgeCurrency::make_free_balance_be(&caller, balance);
            let transfer_amount = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            SystemOrigin::Signed(caller.clone()),
            H160::from_slice(b"0x6D1eee1CFeEAb71A4d"),
            transfer_amount
        )

        transfer_token_to_realis {
            let caller = alice::<T>();
            let owner_origin = SystemOrigin::Signed(caller.clone());
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let pallet_id = RealisBridge::<T>::account_id();
            let balance = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::BridgeCurrency::make_free_balance_be(&recipient, balance);
            T::BridgeCurrency::make_free_balance_be(&pallet_id, balance);
            T::BridgeCurrency::make_free_balance_be(&caller, balance);
            let transfer_amount = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            SystemOrigin::Signed(caller.clone()),
            H160::from_slice(b"0x6D1eee1CFeEAb71A4d"),
            recipient,
            transfer_amount
        )

        transfer_nft_to_bsc {
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
              H160::from_slice(b"0x6D1eee1CFeEAb71A4d"),
              U256([1, 0, 0, 0])
          )

        transfer_nft_to_realis {
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
            H160::from_slice(b"0x6D1eee1CFeEAb71A4d"),
            caller.clone(),
            U256([1, 0, 0, 0])
        )
    }
}
