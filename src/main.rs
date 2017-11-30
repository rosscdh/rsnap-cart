#![allow(unused_variables)]

extern crate sapper;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate log;

use sapper::SapperApp;

mod cart;
use cart::Cart;
// mod products;
// use product::Products;

pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .add_module(Box::new(Cart));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}