#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use crate::Pallet as RealisBridge;
    use crate::*;
    use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
    use frame_support::traits::Currency;
    use frame_system::RawOrigin as SystemOrigin;
    use pallet_nft as Nft;
    use primitive_types::U256;
    use realis_primitives::*;
    use sp_core::H160;
    use sp_runtime::traits::Saturating;

    const SEED: u32 = 1;
    const ED_MULTIPLIER: u32 = 10;
    const ED_MULTIPLIER_2: u32 = 5;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = Nft::NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        transfer_token_to_bsc {
            let caller = alice::<T>();
              let owner_origin = SystemOrigin::Signed(caller.clone());
              let recipient: T::AccountId = account("recipient", 1, SEED);

              let balance = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
              let _ = T::BridgeCurrency::make_free_balance_be(&recipient, balance);
              let transfer_amount = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            owner_origin,
            H160::from_slice("0x6D1eee1CFeEAb71A4d7Fcc73f0EF67A9CA2cD943".as_bytes()),
            transfer_amount
        )

        transfer_token_to_realis {
            let caller = alice::<T>();
              let owner_origin = SystemOrigin::Signed(caller.clone());
              let recipient: T::AccountId = account("recipient", 1, SEED);

              let balance = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
              let _ = T::BridgeCurrency::make_free_balance_be(&recipient, balance);
              let transfer_amount = T::BridgeCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
        }: _(
            owner_origin,
            H160::from_slice("0x6D1eee1CFeEAb71A4d7Fcc73f0EF67A9CA2cD943".as_bytes()),
            recipient,
            transfer_amount
        )

        transfer_nft_to_bsc {
              let caller = alice::<T>();
              let owner_origin = SystemOrigin::Signed(caller.clone());
              RealisBridge::<T>::transfer_nft_to_realis(
                  owner_origin.clone().into(),
                  H160::from_slice("0x6D1eee1CFeEAb71A4d7Fcc73f0EF67A9CA2cD943".as_bytes()),
                  caller.clone(),
                  U256([1, 0, 0, 0])
              )?;
          }: _(
              owner_origin.clone(),
              H160::from_slice("0x6D1eee1CFeEAb71A4d7Fcc73f0EF67A9CA2cD943".as_bytes()),
              U256([1, 0, 0, 0])
          )

        transfer_nft_to_realis {
              let caller = alice::<T>();
              let owner_origin = SystemOrigin::Signed(caller.clone());
          }: _(
                  owner_origin,
                  H160::from_slice("0x6D1eee1CFeEAb71A4d7Fcc73f0EF67A9CA2cD943".as_bytes()),
                  caller.clone(),
                  U256([1, 0, 0, 0])
          )
    }
}
