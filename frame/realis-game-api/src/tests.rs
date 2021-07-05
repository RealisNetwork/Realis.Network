use crate::{mock::*, Config, Currency, Error};
use frame_support::{assert_err, assert_ok};
use pallet_nft as NFT;
use primitive_types::U256;
use realis_primitives::*;

fn alice<T: Config>() -> T::AccountId {
    let alice = NFT::NftMasters::<T>::get();
    alice.get(0).unwrap().clone()
}

#[test]
fn mint_some_type() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
    })
}

#[test]
fn mint_existent_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_err!(
            RealisGameApi::mint_basic_nft(
                Origin::signed(1),
                1,
                U256([1, 0, 0, 0]),
                1
            ),
            Error::<Test>::TokenExist
        );
    })
}

#[test]
fn burn_none_existent_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_err!(
            RealisGameApi::burn_basic_nft(Origin::signed(1), U256([1, 0, 0, 0])),
            NFT::Error::<Test>::NonExistentToken
        )
    })
}

#[test]
fn mint_and_burn() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_1_2_burn_1_2() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([2, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([2, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_1_2_burn_2_1() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([2, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([2, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_transfer_burn_by_owner() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(2),
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_transfer_burn_not_by_owner() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
        assert_err!(
            RealisGameApi::burn_basic_nft(Origin::signed(1), U256([1, 0, 0, 0])),
            NFT::Error::<Test>::NotTokenOwner
        );
    })
}

#[test]
fn mint_and_transfer() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_and_transfer_2_times() {
    new_test_ext(vec![1, 2, 3]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(2),
            3,
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_and_transfer_2_times_burn_by_owner() {
    new_test_ext(vec![1, 2, 3]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(2),
            3,
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(3),
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_and_transfer_2_times_burn_not_by_owner() {
    new_test_ext(vec![1, 2, 3]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            1
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(2),
            3,
            U256([1, 0, 0, 0])
        ));
        assert_err!(
            RealisGameApi::burn_basic_nft(Origin::signed(2), U256([1, 0, 0, 0])),
            NFT::Error::<Test>::NotTokenOwner
        );
    })
}

#[test]
fn transfer_from_pallet() {
    new_test_ext(vec![1]).execute_with(|| {
        let caller = alice::<Test>();
        let pallet_id = RealisGameApi::account_id();
        Balances::make_free_balance_be(&pallet_id, 100);
        let transfer_amount = 50;

        assert_ok!(RealisGameApi::transfer_from_pallet(
            Origin::signed(caller.clone()),
            caller.clone(),
            transfer_amount
        ));
    })
}

#[test]
fn transfer_to_pallet() {
    new_test_ext(vec![1]).execute_with(|| {
        let caller = alice::<Test>();
        Balances::make_free_balance_be(&caller, 100);
        let transfer_amount = 50;

        assert_ok!(RealisGameApi::transfer_to_pallet(
            Origin::signed(caller.clone()),
            caller.clone(),
            transfer_amount
        ));
    })
}

#[test]
fn transfer_from_ptp() {
    new_test_ext(vec![1, 2, 3]).execute_with(|| {
        let caller = alice::<Test>();
        Balances::make_free_balance_be(&caller, 100);
        let transfer_amount = 50;

        assert_ok!(RealisGameApi::transfer_from_ptp(
            Origin::signed(caller.clone()),
            caller.clone(),
            3,
            transfer_amount
        ));
    })
}

#[test]
fn spend_in_game() {
    new_test_ext(vec![1]).execute_with(|| {
        let caller = alice::<Test>();
        Balances::make_free_balance_be(&caller, 100);
        let transfer_amount = 50;

        assert_ok!(RealisGameApi::spend_in_game(
            Origin::signed(caller.clone()),
            caller.clone(),
            transfer_amount
        ));
    })
}
