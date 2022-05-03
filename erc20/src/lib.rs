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

    pub fn balance_of(self, account: AccountId) -> U128 {
        match self._balances.get(&account) {
            Some(amount) => amount.clone(),
            None => U128(0),
        }
    }

    pub fn transfer(&mut self, recipient: AccountId, amount: U128) {
        let sender = env::signer_account_id();
        self._balances.entry(sender.to_string()).or_insert(U128(0));
        self._balances
            .entry(recipient.to_string())
            .or_insert(U128(0));
        if let Some(balance_of_sender) = self._balances.get(&sender) {
            if balance_of_sender.0 < amount.0 {
                env::log("Out of amount".as_bytes());
                let mut temp_recipient = U128(0);
                let mut temp_sender = U128(0);
                if let Some(balance_of_recipient) = self._balances.get(&recipient) {
                    // transfer
                    temp_sender = U128(balance_of_sender.0 - amount.0);
                    temp_recipient = U128(balance_of_recipient.0 + amount.0);

                    self._balances.insert(sender, temp_sender);
                    self._balances.insert(recipient, temp_recipient);

                    env::log("Transfer successfully".as_bytes());
                } else {
                    env::log("Transfer faily".as_bytes());
                };
            }
        } else {
            env::log("U should deposit token first".as_bytes());
        }
    }

    pub fn mint_by_owner(&mut self, recipient: AccountId, amount: U128) {
        let sender = env::signer_account_id();
        match sender == self._owner_id {
            true => {
                self._balances
                    .entry(recipient.to_string())
                    .or_insert(U128(0));
                self._total_supply = U128(self._total_supply.0 + amount.0);
                self._balances.insert(recipient.to_string(), amount);
                env::log("Mint successfully".as_bytes());
            }
            false => {
                env::log("U are not owner".as_bytes());
            }
        }
    }

    pub fn transfer_from(&mut self, sender: AccountId, recipient: AccountId, amount: U128) {
        let spender = env::signer_account_id();
        let sender_clone = sender.clone();
        let price_allowed = self.allowance(sender, spender);
        if price_allowed.0 >= amount.0 {
            if let Some(balance_of_sender) = self._balances.get(&sender_clone) {
                if balance_of_sender.0 < amount.0 {
                    env::log("Out of amount".as_bytes());
                    let mut temp_recipient = U128(0);
                    let mut temp_sender = U128(0);
                    if let Some(balance_of_recipient) = self._balances.get(&recipient) {
                        // transfer
                        temp_sender = U128(balance_of_sender.0 - amount.0);
                        temp_recipient = U128(balance_of_recipient.0 + amount.0);

                        self._balances.insert(sender_clone, temp_sender);
                        self._balances.insert(recipient, temp_recipient);

                        env::log("Transfer successfully".as_bytes());
                    } else {
                        env::log("Transfer faily".as_bytes());
                    };
                }
            } else {
                env::log("U can't transfer money".as_bytes());
            }
        }
    }

    pub fn allowance(&self, owner: AccountId, spender: AccountId) -> U128 {
        match self._allowances.get(&owner) {
            Some(sender_allowances) => match sender_allowances.get(&spender) {
                Some(amount) => *amount,
                None => U128(0),
            },
            None => U128(0),
        }
    }

    pub fn approve(&mut self, spender: AccountId, amount: U128) {
        let sender = env::signer_account_id();
        self._allowances
            .entry(sender.to_string())
            .or_insert(HashMap::new());
        if let Some(sender_allowances) = self._allowances.get_mut(&sender) {
            match sender_allowances.insert(spender, amount) {
                Some(_) => {
                    println!("Approve successfully");
                }
                None => {
                    println!("Approve failly");
                }
            }
        }
    }
}
