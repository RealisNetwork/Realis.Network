#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use crate::Pallet as RealisGameApi;
    use crate::*;
    use pallet_nft::Pallet as Nft;
    use pallet_nft_delegate::Pallet as NftDelegate;
    use primitive_types::U256;

    use realis_primitives::*;
    use frame_benchmarking::{account, benchmarks};
    use frame_support::traits::Currency;
    use frame_system::RawOrigin as SystemOrigin;
    use pallet_nft::NftMasters;

    use sp_runtime::traits::Saturating;

    const SEED: u32 = 1;
    const ED_MULTIPLIER: u32 = 10;

    const ED_MULTIPLIER_2: u32 = 5;

    // Get Alice AccountId
    fn alice<T: Config>() -> T::AccountId {
        let alice = NftMasters::<T>::get();
        alice.get(0).unwrap().clone()
    }

    benchmarks! {
        mint_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            1,
            b"qq".to_vec(),
            Rarity::Common,
            b"1".to_vec(),
            b"Qq".to_vec()
        )

        burn_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0])
        )

        transfer_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let recipient_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(recipient.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            T::ApiCurrency::make_free_balance_be(&recipient, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                recipient_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            recipient,
            U256([1, 0, 0, 0])
        )

        transfer_from_pallet {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let recipient_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let _ = T::ApiCurrency::make_free_balance_be(&caller, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                recipient_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            recipient,
            transfer_amount
        )

        transfer_to_pallet {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let recipient_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let _ = T::ApiCurrency::make_free_balance_be(&caller, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                recipient_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            transfer_amount
        )

        transfer_from_ptp {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let recipient_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&recipient, balance);
            let _ = T::ApiCurrency::make_free_balance_be(&caller, balance);
            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                recipient_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            recipient,
            transfer_amount
        )

        spend_in_game {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let recipient: T::AccountId = account("recipient", 1, SEED);
            let recipient_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            T::ApiCurrency::make_free_balance_be(&recipient, balance);

            let transfer_amount = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER_2 - 1).into());
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                recipient_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            recipient,
            transfer_amount
        )

        add_to_whitelist {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
        }: _(
            SystemOrigin::Signed(caller.clone())
        )

        remove_from_whitelist {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
             let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone())
        )

        add_to_validator_whitelist {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
             let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone()
        )

        remove_from_validator_whitelist {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
             let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER - 1).into());
            let _ = T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_validator_whitelist(
                owner_origin.clone(),
                caller.clone()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone()
        )

        sell_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            10
        )

        buy_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 0, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            RealisGameApi::<T>::sell_nft(
                owner_origin.clone(),
                caller.clone(),
                U256([1, 0, 0, 0]),
                10
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0])
        )

        change_price_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            RealisGameApi::<T>::sell_nft(
                owner_origin.clone(),
                caller.clone(),
                U256([1, 0, 0, 0]),
                10
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            10
        )

        remove_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0])
        )

        delegate_nft {
            let caller = alice::<T>();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(buyer.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            T::ApiCurrency::make_free_balance_be(&buyer, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                buyer_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            buyer.clone(),
            U256([1, 0, 0, 0]),
            10
        )

        sell_delegate_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let seller: T::AccountId = account("seller", 0, 1);
            let seller_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(seller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            T::ApiCurrency::make_free_balance_be(&seller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                seller_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            seller.clone(),
            U256([1, 0, 0, 0]),
            10,
            100
        )

        buy_delegate_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("seller", 0, 1);
            let buyer_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(buyer.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            T::ApiCurrency::make_free_balance_be(&buyer, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                buyer_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin.clone(),
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            buyer.clone(),
            U256([1, 0, 0, 0])
        )

        change_price_delegate_nft {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin.clone(),
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            10
        )

        change_delegate_nft_time_on_sale {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin.clone(),
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0]),
            200
        )

        remove_from_sell {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            NftDelegate::<T>::sell_delegate (
                owner_origin.clone(),
                U256([1, 0, 0, 0]),
                2,
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            caller.clone(),
            U256([1, 0, 0, 0])
        )

        remove_delegate {
            let caller = alice::<T>();
            let owner_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(caller.clone()).into();
            let buyer: T::AccountId = account("buyer", 0, 1);
            let balance = T::ApiCurrency::minimum_balance().saturating_mul((ED_MULTIPLIER * 10).into());
            let buyer_origin: <T as frame_system::Config>::Origin = SystemOrigin::Signed(buyer.clone()).into();
            T::ApiCurrency::make_free_balance_be(&buyer, balance);
            T::ApiCurrency::make_free_balance_be(&caller, balance);
            RealisGameApi::<T>::add_to_whitelist(
                owner_origin.clone(),
            )?;
            RealisGameApi::<T>::add_to_whitelist(
                buyer_origin.clone(),
            )?;
            Nft::<T>::mint(
                owner_origin.clone(),
                caller.clone(),
                b"QQ".to_vec(),
                U256([1, 1, 0, 0]),
                1,
                Rarity::Common,
                b"QQ".to_vec()
            )?;
            NftDelegate::<T>::delegate (
                owner_origin.clone(),
                buyer.clone(),
                U256([1, 0, 0, 0]),
                20
            )?;
        }: _(
            SystemOrigin::Signed(caller.clone()),
            buyer,
            U256([1, 0, 0, 0])
        )
    }
}
