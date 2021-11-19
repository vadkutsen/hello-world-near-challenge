
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::LookupMap;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<String, String>,
}

impl Default for Welcome {
  fn default() -> Self {
    Self {
      records: LookupMap::new(b"a".to_vec()),
    }
  }
}

#[near_bindgen]
impl Welcome {

    pub fn set_name(&mut self, name: String) {
        let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Saving name '{}' for account '{}'", name, account_id,).as_bytes());

        self.records.insert(&account_id, &name);
    }

    pub fn get_name(&mut self, account_id: String) -> String {
        match self.records.get(&account_id) {
            Some(name) => name,
            None => "User".to_string(),
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
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
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_name() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_name("Bob".to_string());
        assert_eq!(
            "Bob".to_string(),
            contract.get_name("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_name() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let mut contract = Welcome::default();
        // this test did not call set_name so should return the default "Hello" greeting
        assert_eq!(
            "User".to_string(),
            contract.get_name("francis.near".to_string())
        );
    }
}
