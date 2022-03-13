#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::NFTId;

pub trait NFTTrait {
	type AccountId: Clone;

	fn amount_owned(nft_id: NFTId, account: Self::AccountId) -> u128;
	fn transfer(nft_id: NFTId, from: Self::AccountId, to: Self::AccountId, amount: u128) -> u128;
}
