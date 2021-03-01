use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    // traits::Get,
};
// use sp_std::vec::Vec;

pub trait Nft<AccountId> {

    type TokenId;

    fn mint(target_account: &AccountId, token_id: Self::TokenId) -> Result<Self::TokenId, DispatchError>;

    fn burn(token_id: Self::TokenId) -> Result<Self::TokenId, DispatchError>;

    fn transfer(dest_account: &AccountId, token_id: &Self::TokenId) -> DispatchResult;

    // fn breed_token(token_id: Self::TokenId, token_id2: Self::TokenId) -> DispatchResult;

    fn owner_of(token_id: &Self::TokenId) -> AccountId;
}