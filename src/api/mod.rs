use crate::chain::*;
use lazy_static::lazy_static;

lazy_static! {
  static ref blockchain: Chain = {
    let mut c = Chain::new();
    c
  };
}

#[get("/blockchain")]
fn get_blockchain() -> String {
  blockchain.mine("asdf");
  let c = blockchain.get_chain();
  serde_json::to_string(&c).unwrap()
}

pub fn initialize() {
  let rok = rocket::ignite();
  rok.mount("/", routes![get_blockchain]).launch();
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_test() {
    assert!(true);
  }
}
