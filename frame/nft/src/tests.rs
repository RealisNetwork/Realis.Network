//use super::*;
use crate::{mock::*, Error, Params, Rarity, Socket, Types};
use frame_support::{assert_err, assert_ok};
use primitive_types::U256;

#[test]
fn mint_some_nft() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(Nft::mint_basic(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            self::Types { tape: 1 }
        ));
    });
}

/// Mint a new token to account than burn same token from same account
/// Mint - ok
/// Burn - ok
#[test]
fn mint_and_burn_same_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::burn(Origin::signed(1), U256([1, 0, 0, 0])));
    })
}

/// Mint a new token to account than try burn other (not existing token) from same account
/// Mint - ok
/// Burn - error - NonExistentToken
#[test]
fn mint_and_burn_different_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_err!(
            Nft::burn(Origin::signed(1), U256([2; 4])),
            Error::<Test>::NonExistentToken
        );
    })
}

/// Mint a new token to account than try burn same token from other account
/// Mint - ok
/// Burn - error - NotTokenOwner
#[test]
fn mint_token_and_burn_it_not_by_owner() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_err!(
            Nft::burn(Origin::signed(2), U256([1, 0, 0, 0])),
            Error::<Test>::NotTokenOwner
        );
    })
}

/// Mint token to account than transfer this token to another account
/// Mint - ok
/// Transfer - ok
#[test]
fn mint_token_and_transfer_it() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::transfer(Origin::signed(1), 2, U256([1, 0, 0, 0])));
    })
}

/// Mint token to account than transfer this token to none existing account
/// Mint - ok
/// Transfer - ok
#[test]
fn mint_token_and_transfer_it_to_non_exist_account() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::transfer(Origin::signed(1), 99, U256([1, 0, 0, 0])));
    })
}

/// Mint token than transfer it to other account
/// Than try transfer back not by owner
/// mint - ok
/// transfer - ok
/// transfer - error - NotTokenOwner
#[test]
fn mint_token_and_transfer_it_two_times() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::transfer(Origin::signed(1), 2, U256([1, 0, 0, 0])));
        assert_err!(
            Nft::transfer(Origin::signed(1), 1, U256([1, 0, 0, 0])),
            Error::<Test>::NotTokenOwner);
    })
}

/// Mint token than transfer it to other account
/// than burn from that account by new owner
/// main - ok
/// transfer - ok
/// burn - ok
#[test]
fn mint_token_and_transfer_it_then_burn() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::transfer(Origin::signed(1), 2, U256([1, 0, 0, 0])));
        assert_ok!(Nft::burn(Origin::signed(2), U256([1, 0, 0, 0])));
    })
}

/// Mint token than transfer it to other account
/// than burn from that account by old owner
/// main - ok
/// transfer - ok
/// burn - error - NotTokenOwner
#[test]
fn mint_token_and_transfer_it_then_burn_not_by_owner() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(Nft::mint(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            Rarity::Rare,
            Socket::Head,
            Params {
                strength: 1,
                agility: 1,
                intelligence: 1
            }
        ));
        assert_ok!(Nft::transfer(Origin::signed(1), 2, U256([1, 0, 0, 0])));
        assert_err!(
            Nft::burn(Origin::signed(1), U256([1, 0, 0, 0])),
            Error::<Test>::NotTokenOwner);
    })
}

#[test]
fn try_burn_not_exists_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_err!(
            Nft::burn(Origin::signed(1), U256([1, 0, 0, 0])),
            Error::<Test>::NonExistentToken
        );
    })
}

#[test]
fn try_transfer_not_exists_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_err!(
            Nft::transfer(Origin::signed(1), 2, U256([1, 0, 0, 0])),
            Error::<Test>::NonExistentToken
        );
    })
}

#[test]
fn try_burn_basic_not_exists_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_err!(
            Nft::burn_basic(Origin::signed(1), U256([1, 0, 0, 0])),
            Error::<Test>::NonExistentToken
        );
    })
}

#[test]
fn try_transfer_basic_not_exists_token() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_err!(
            Nft::transfer_basic(Origin::signed(1), 2, U256([1, 0, 0, 0])),
            Error::<Test>::NonExistentToken
        );
    })
}
