use serde::Serialize;
use sha256::digest_bytes;

pub const DIFFICULTY: u8 = 4;

pub trait IBlock {
  fn mine(&mut self);
  fn validate(&self) -> bool;
  fn get_id(&self) -> u64;
  fn is_genesis(&self) -> bool;
  fn get_payload(&self) -> String;
  fn get_nounce(&self) -> u64;
  fn get_hash(&self) -> String;
  fn get_previous_hash(&self) -> String;
  fn to_string(&self) -> String;
  fn calculate_hash(&mut self);
}

#[derive(Debug, Serialize, Clone)]
pub struct Block {
  id: u64,
  genesis: bool,
  payload: String,
  nounce: u64,
  hash: String,
  previous_hash: String,
}

impl IBlock for Block {
  fn mine(&mut self) {
    let prefix = self.get_prefix();

    loop {
      self.calculate_hash();
      if self.hash.starts_with(&prefix) {
        break;
      } else {
        self.nounce += 1;
      }
    }
  }

  fn validate(&self) -> bool {
    let hash = self.compute_hash();

    if hash != self.hash {
      return false;
    }

    let prefix = self.get_prefix();
    if !self.hash.starts_with(&prefix) {
      return false;
    }

    true
  }

  fn get_id(&self) -> u64 {
    self.id
  }

  fn is_genesis(&self) -> bool {
    self.genesis
  }

  fn get_payload(&self) -> String {
    self.payload.clone()
  }

  fn get_nounce(&self) -> u64 {
    self.nounce
  }

  fn get_hash(&self) -> String {
    self.hash.clone()
  }

  fn get_previous_hash(&self) -> String {
    self.previous_hash.clone()
  }

  fn to_string(&self) -> String {
    format!(
      "id={}; genesis={}; payload={}; nounce={}; previous_hash={}",
      self.id, self.genesis, self.payload, self.nounce, self.previous_hash
    )
  }

  fn calculate_hash(&mut self) {
    self.hash = self.compute_hash();
  }
}

impl Block {
  pub fn new(id: u64, genesis: bool, payload: String, previous_hash: String) -> Block {
    Block {
      id: id,
      genesis: genesis,
      nounce: 0,
      payload: payload,
      hash: "".to_string(),
      previous_hash: previous_hash,
    }
  }

  fn compute_hash(&self) -> String {
    let input = self.to_string();
    return digest_bytes(&input.as_bytes());
  }

  fn get_prefix(&self) -> String {
    String::from_utf8(vec![b'0'; DIFFICULTY.into()]).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use crate::block::*;

  #[test]
  fn simple_test() {
    assert!(true);
  }

  #[test]
  fn test_object_created() {
    let block: Block = Block::new(123, true, "foo: bar".to_string(), "abc".to_string());
    assert_eq!(block.get_id(), 123);
    assert!(block.is_genesis());
    assert_eq!(block.get_payload(), "foo: bar".to_string());
    assert_eq!(block.get_previous_hash(), "abc".to_string());
    assert_eq!(block.get_hash(), "".to_string());
    assert_eq!(block.get_nounce(), 0);
    assert_eq!(
      block.to_string(),
      "id=123; genesis=true; payload=foo: bar; nounce=0; previous_hash=abc"
    );
  }

  #[test]
  fn test_block_hash() {
    let mut block = Block::new(
      123456,
      false,
      "lorem ipsum dolor sit amet".to_string(),
      "fddklfjdsfsalkdrto".to_string(),
    );
    block.calculate_hash();
    let expected = "edcb707a5684c389230adbe4076e98cdd6cb488f028d98ede479802c33be860d".to_string();
    assert_eq!(block.get_hash(), expected);
  }

  #[test]
  fn test_mine_block() {
    let mut block = Block::new(
      2,
      false,
      "This is a block to be mined".to_string(),
      "edcb707a5684c389230adbe4076e98cdd6cb488f028d98ede479802c33be860d".to_string(),
    );

    block.mine();
    assert_eq!(block.get_nounce(), 2184);
    assert_eq!(
      block.get_hash(),
      "00007b479bb492680dd1ab6aac50631c9d34c925f61841b1368b7946ef2ff247".to_string()
    );
  }

  #[test]
  fn test_validate_valid_block() {
    let mut block = Block::new(
      2,
      false,
      "This is a block to be validated".to_string(),
      "edcb707a5684c389230adbe4076e98cdd6cb488f028d98ede479802c33be860d".to_string(),
    );
    block.mine();
    let valid = block.validate();
    assert!(valid);
  }

  #[test]
  fn test_validate_invalid_block_hash_mismatch() {
    let block = Block::new(
      2,
      false,
      "This block does not match hash so is not valid".to_string(),
      "edcb707a5684c389230adbe4076e98cdd6cb488f028d98ede479802c33be860d".to_string(),
    );
    let valid = block.validate();
    assert!(!valid);
  }

  #[test]
  fn test_validate_invalid_block_does_not_meet_consensus() {
    let mut block = Block::new(
      2,
      false,
      "This block does not meet consensus so is not valid".to_string(),
      "edcb707a5684c389230adbe4076e98cdd6cb488f028d98ede479802c33be860d".to_string(),
    );
    block.calculate_hash();
    let valid = block.validate();
    assert!(!valid);
  }
}
