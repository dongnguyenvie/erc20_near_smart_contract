use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

// 1. Main Struct

// 2. Default Implementation

// 3. Core Logic

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KeyValue {
    pairs: UnorderedMap<String, String>,
}

impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, k: String, v: String) {
        env::log(b"Create or updated");
        self.pairs.insert(&k, &v);
    }

    pub fn read(&self, k: String) -> Option<String> {
        env::log(b"Read");
        return self.pairs.get(&k);
    }

    pub fn read_keys<'a>(&'a self) -> Vec<String> {
        env::log(b"Read_keys");
        return self.pairs.keys().collect();
    }

    pub fn delete(&mut self, k: String) {
        env::log(b"Delete");
        self.pairs.remove(&k);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn create_read_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.read("first_key".to_string()).unwrap()
        );
    }

    #[test]
    fn read_nonexistent_pair() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = KeyValue::default();
        assert_eq!(None, contract.read("first_key".to_string()));
    }

    #[test]
    fn delete_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.read("first_key".to_string()).unwrap()
        );
        contract.delete("first_key".to_string());
        assert_eq!(None, contract.read("first_key".to_string()));
    }

    #[test]
    fn read_keys() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        contract.create_update("first_key1".to_string(), "hello".to_string());
        contract.create_update("first_key2".to_string(), "hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.read("first_key".to_string()).unwrap()
        );
        for key in contract.read_keys() {
            println!("read_keys:::::{}", key)
        }

        // assert_eq!(None, contract.read("first_key".to_string()));
    }
}
