use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  accounts: LookupMap<String, String>
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      accounts: LookupMap::new(b"r".to_vec())
    }
  }
}

#[near_bindgen]
impl Contract {

  pub fn set_account_details(&mut self, account: String) {
    let account_id = env::signer_account_id();
    self.accounts.insert(&account_id, &account);
  }

  pub fn get_account_details(&self, account_id: String) -> Option<String> {
    return self.accounts.get(&account_id);
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
    fn set_get_account_details() {
      let context = get_context(vec![], false);
      testing_env!(context);
      let mut contract = Contract::default();
      contract.set_account_details("hello".to_string());
      assert_eq!(
        "hello".to_string(),
        contract.get_account_details("bob_near".to_string()).unwrap()
      );
    }

    #[test]
    fn get_nonexistent_account_details() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Contract::default();
        assert_eq!(None, contract.get_account_details("francis.near".to_string()));
    }
}