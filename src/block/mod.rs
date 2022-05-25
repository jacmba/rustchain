mod block {
  pub trait IBlock {
    fn mine(&self);
    fn validate(&self) -> bool;
  }

  #[derive(Debug)]
  pub struct Block {
    genesis: bool,
    payload: String,
    nounce: u64,
    hash: String,
    previousHash: String,
  }

  impl IBlock for Block {
    fn mine(&self) {
      // ToDo mining logic
    }

    fn validate(&self) -> bool {
      // ToDo mining logic
      true
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_test() {
    assert!(true);
  }
}