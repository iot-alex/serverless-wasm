extern crate httparse;
extern crate mio;
extern crate parity_wasm;
extern crate rouille;
extern crate slab;
extern crate toml;
extern crate wasmi;

#[macro_use]
extern crate serde_derive;

use std::env::args;

mod async;
mod config;
mod interpreter;
mod sync;

fn main() {
  let args: Vec<_> = args().collect();
  if args.len() != 2 {
    println!("Usage: {} <config_file>", args[0]);
    return;
  }

  if let Some(config) = config::load(&args[1]) {
    async::server(config);
  } else {
    println!("invalid configuration");
  }
}
