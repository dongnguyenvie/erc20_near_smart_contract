// ref to the tutorial here: https://learn.figment.io/tutorials/write-nft-contracts-in-rust
#![deny(warnings)]

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The token ID type is also defined in the NEP
pub type TokenId = u64;
pub type AccountIdHash = Vec<u8>;

pub trait NEP4 {
    fn grant_access(&mut self, escrow_account_id: AccountId);

    fn revoke_acess(&mut self, escrow_account_id: AccountId);

    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, token_id: TokenId);

    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId);

    fn check_access(&mut self, account_id: AccountId) -> bool;

    fn get_token_owner(&mut self, token_id: TokenId) -> String;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleTokenBasic {
    pub token_to_account: UnorderedMap<TokenId, AccountId>,
    pub account_gives_access: UnorderedMap<AccountId, UnorderedSet<AccountIdHash>>,
    pub owner_id: AccountId,
    pub token_to_flarn: UnorderedMap<TokenId, Flarn>,
}

use near_sdk::serde::Serialize;
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
pub struct Flarn {
    pub dna: u64,
}

impl NonFungibleTokenBasic {
    pub fn new(owner_id: AccountId) -> Self {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid."
        );
        assert!(!env::state_exists(), "Already initialized");
        Self {
            token_to_account: UnorderedMap::new(b"token_to_account".to_vec()),
            account_gives_access: UnorderedMap::new(b"account_gives_access".to_vec()),
            owner_id: env::current_account_id(),
            token_to_flarn: UnorderedMap::new(b"token_to_flarn".to_vec()),
        }
    }
}
