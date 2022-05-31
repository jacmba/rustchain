use crate::chain::*;
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;

lazy_static! {
  static ref BLOCKCHAIN: Mutex<Chain> = Mutex::new(Chain::new());
}

#[derive(Serialize)]
struct MineResponse {
  id: u64,
}

#[get("/blockchain")]
fn get_chain() -> String {
  let bc = BLOCKCHAIN.lock().unwrap();
  let chain = bc.get_chain();
  serde_json::to_string(&chain).unwrap()
}

#[get("/block/<id>")]
fn get_block(id: u64) -> String {
  let bc = BLOCKCHAIN.lock().unwrap();
  let block = bc.get_block(id);
  serde_json::to_string(&block).unwrap()
}

#[post("/mine", data = "<data>")]
fn post_mine(data: String) -> String {
  let mut bc = BLOCKCHAIN.lock().unwrap();
  let id = bc.mine(&data);
  let response = MineResponse { id: id };
  serde_json::to_string(&response).unwrap()
}

pub fn initialize() {
  rocket::ignite()
    .mount("/", routes![get_chain, get_block, post_mine])
    .launch();
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_test() {
    assert!(true);
  }
}
