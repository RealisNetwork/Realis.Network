use crate::{mock::*, Error};
use pallet_nft as NFT;
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext(vec![1, 2]).execute_with(|| {
		assert_ok!(NFT::mint_basic(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types {
				tapes: 2
            }
        ));
	})
}