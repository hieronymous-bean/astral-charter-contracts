// standard imports //
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,};

// imports specific to non-fungible-tokens //
// using these specifically because the course completion achievements are going to be in the form of NFTs //
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;

// boilerplate for setting up the allocator used in the wasm binary //
near_sdk::setup_alloc!();

// primary structure for the smart contract //
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AstralCharter {
	val: i8,
}

// implementation of the smart contract //
#[near_bindgen]
impl AstralCharter {
	
	// function for getting the users' completed courses //
	pub fn get_completed_courses(&self) -> i8 {
		return self.val;
	}

	// add newly-completed course to users' histories //
	pub fn complete_course(&mut self) {
		self.val += 1;
		let log_message = format!("Increased number to {}", self.val);
		env::log(log_message.as_bytes());
	}

}



// unit tests for the above methods //
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
      VMContext {
				current_account_id: "alice.testnet".to_string(),
				signer_account_id: "robert.testnet".to_string(),
				signer_account_pk: vec![0, 1, 2],
				predecessor_account_id: "jane.testnet".to_string(),
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
    fn increment() {
			let context = get_context(vec![], false);
			testing_env!(context);

      let mut contract = AstralCharter { val: 0 };
      contract.complete_course();
      println!("Value after increment: {}", contract.get_completed_courses());
      assert_eq!(1, contract.get_completed_courses());
    }
		
}