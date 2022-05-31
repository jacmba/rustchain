use crate::block::*;
use serde::Serialize;

pub trait IChain {
  fn get_chain(&self) -> &Vec<Block>;
  fn get_block(&self, id: u64) -> &Block;
  fn mine(&mut self, payload: &str) -> u64;
  fn validate(&self, id: u64) -> bool;
  fn size(&self) -> usize;
}

#[derive(Serialize)]
pub struct Chain {
  blocks: Vec<Block>,
}

impl IChain for Chain {
  fn get_chain(&self) -> &Vec<Block> {
    &self.blocks
  }

  fn get_block(&self, id: u64) -> &Block {
    let index = match id {
      0 => 0,
      _ => id - 1,
    } as usize;
    &self.blocks[index]
  }

  fn mine(&mut self, payload: &str) -> u64 {
    let index = self.blocks.len() + 1;
    let mut block = Block::new(
      index as u64,
      false,
      payload.to_string(),
      self.blocks[index - 2].get_hash(),
    );
    block.mine();
    self.blocks.push(block);
    index as u64
  }

  fn validate(&self, id: u64) -> bool {
    if id == 0 || id as usize > self.blocks.len() {
      return false;
    }
    let block = self.get_block(id);
    if id > 1 {
      let previous = self.get_block(id - 1);
      if block.get_previous_hash() != previous.get_hash() {
        return false;
      }
    } else {
      if !block.is_genesis()
        || block.get_payload() != "Genesis block".to_string()
        || block.get_previous_hash() != "".to_string()
      {
        return false;
      }
    }
    block.validate()
  }

  fn size(&self) -> usize {
    self.blocks.len()
  }
}

impl Chain {
  pub fn new() -> Chain {
    let mut chain = Chain { blocks: vec![] };
    let mut block = Block::new(1, true, "Genesis block".to_string(), "".to_string());
    block.mine();
    chain.blocks.push(block);
    chain
  }
}

#[cfg(test)]
mod tests {
  use crate::chain::*;

  #[test]
  fn test_chain_creation() {
    let chain = Chain::new();
    assert_eq!(chain.size(), 1);
    let genesis = chain.get_block(1);
    assert!(genesis.is_genesis());
    assert_eq!(genesis.get_payload(), "Genesis block".to_string());
    assert_eq!(genesis.get_previous_hash(), "".to_string());
  }

  #[test]
  fn test_mining_new_block() {
    let mut chain = Chain::new();
    let genesis_hash = chain.get_block(1).get_hash();
    let id = chain.mine("My new block");
    let block = chain.get_block(id);
    assert_eq!(chain.size(), 2);
    assert_eq!(id, 2);
    assert!(!block.is_genesis());
    assert_eq!(block.get_payload(), "My new block".to_string());
    assert_eq!(block.get_previous_hash(), genesis_hash);
  }

  #[test]
  fn test_block_validation() {
    let chain = Chain::new();
    assert!(chain.validate(1));
  }

  #[test]
  fn test_invalid_block_validation() {
    let chain = Chain::new();
    assert!(!chain.validate(5));
  }
}
