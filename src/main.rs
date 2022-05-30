#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod block;
mod chain;

fn main() {
  api::initialize();
}
