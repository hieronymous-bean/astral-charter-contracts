use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Achievements {
  achievements: LookupMap<String, String>,
}

impl Default for Achievements {
  fn default() -> Self {
    Self {
      achievements: LookupMap::new(b"r".to_vec()),
    }
  }
}

#[near_bindgen]
impl Achievements {

}