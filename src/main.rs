#![allow(unused_variables)]

extern crate sapper;
extern crate env_logger;
extern crate serde_json;
extern crate bson;
extern crate mongodb;
#[macro_use]
extern crate log;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use sapper::SapperApp;

mod persist;
use persist::Persist;
mod cart;
use cart::Cart;
// mod products;
// use product::Products;

pub fn main() {
    env_logger::init().unwrap();

    let mongo_client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone mongodb client.");

    let db = mongo_client.db("snap_cart").collection("cart");

    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .add_module(Box::new(Cart));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}