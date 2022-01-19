use lazy_static::lazy_static;

use rocket::http::Status;
use serde_json::json;

use crate::server::api::ApiResponse;
use pos_blockchain::{BlockChain, BlockData};

pub mod api;

use std::sync::Mutex;

lazy_static! {
    static ref BC: Mutex<BlockChain> = Mutex::new(BlockChain::new());
}

#[get("/blocks")]
fn blocks() -> ApiResponse {
    let json = json!(*BC.lock().unwrap().chain).into();
    ApiResponse {
        json,
        status: Status::Ok,
    }
}

#[get("/blockchain")]
fn blockchain() -> String {
    // BC.lock().unwrap().add_block(BlockData::new());
    format!("{}", *BC.lock().unwrap())
}

#[post("/mine", data = "<data>")]
fn mine(data: rocket::serde::json::Json<BlockData>) -> ApiResponse {
    println!("Block data: {:?}", data);
    let block = BC.lock().unwrap().add_block(BlockData::new());
    println!("New block added: {}", block);
    let json = json!(*BC.lock().unwrap().chain).into();
    ApiResponse {
        json,
        status: Status::Ok,
    }
}

#[get("/")]
fn index() -> String {
    format!("Hello World!")
}

pub fn get_index_routes() -> Vec<rocket::Route> {
    routes![index]
}

pub fn get_blockchain_routes() -> Vec<rocket::Route> {
    routes![
        // Heroes
        blocks, blockchain, mine
    ]
}
