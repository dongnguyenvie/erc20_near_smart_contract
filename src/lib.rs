use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::log;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseResult,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct NolanToken {
    _balances: HashMap<AccountId, U128>, // balances
    _allowances: HashMap<AccountId, HashMap<AccountId, U128>>,
    _total_supply: U128,
    _name: String,
    _symbol: String,
    _owner_id: AccountId,
}

// #[ext_contract(ext_ft)]
trait ERC20 {
    fn total_supply(&self) -> U128;
    fn balance_of(account: String) -> U128;
    fn transfer(recipient: String, amount: U128);
    fn allowance(owner: String, spender: String) -> U128;
    fn approve(spender: String, amount: U128) -> bool;
    fn transfer_from(sender: String, recipient: String, amount: U128) -> bool;
}

#[near_bindgen]
impl NolanToken {
    #[init]
    pub fn new(name: String, symbol: String, total_supply: U128) -> Self {
        NolanToken {
            _owner_id: env::current_account_id(),
            _allowances: HashMap::new(),
            _balances: HashMap::new(),
            _name: name,
            _symbol: symbol,
            _total_supply: total_supply,
        }
    }

    pub fn total_supply(&self) -> U128 {
        self._total_supply
    }

    pub fn balance_of(&self, account: AccountId) -> U128 {
        match self._balances.get(&account) {
            Some(amount) => amount.clone(),
            None => U128(0),
        }
    }

    pub fn transfer(&mut self, recipient: AccountId, amount: U128) {
        let sender = env::signer_account_id();
        if let Some(balance_of_sender) = self._balances.get_mut(&sender) {
            if balance_of_sender.0 < amount.0 {
                env::log("out of amount".as_bytes());
                if let Some(x) = self._balances.get_mut(&recipient) {
                    // transfer
                    *balance_of_sender = U128(balance_of_sender.0 - amount.0);
                    *x = U128(x.0 + amount.0);
                } else {
                    // *balance_of_sender = U128(balance_of_sender.0 - amount.0);
                };
                // recipient
                // self._balances.insert(k, v)
            }
        }
        // match self._balances.get(&sender) {
        //     Some(balance) => {
        //         if balance.0 < amount.0 {
        //             env::log("out of amount".as_bytes());
        //             return
        //             if let Some(x) = self._balances.get_mut(&recipient) {
        //                 // transfer

        //                 *x = U128(x.0 + amount.0);
        //             } else {

        //             }
        //             // recipient
        //             // self._balances.insert(k, v)
        //         }
        //     },
        //     None => {
        //         env::log("out of amount".as_bytes());
        //         return
        //     },
        // }
    }
}
