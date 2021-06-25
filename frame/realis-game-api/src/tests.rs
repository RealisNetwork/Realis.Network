use crate::mock::*;
use pallet_nft as NFT;
use frame_support::assert_ok;
use primitive_types::U256;

#[test]
fn it_works_for_default_value() {
	new_test_ext(vec![1, 2]).execute_with(|| {
		assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types {
				tapes: 2
            }
        ));
	})
}