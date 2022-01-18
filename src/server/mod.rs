use lazy_static::lazy_static;

use rocket::http::Status;
use serde_json::json;

use crate::blockchain::{BlockChain, BlockData};
use crate::server::api::ApiResponse;

pub mod api;

fn _test() {
    let mut bc = BlockChain::new();
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    bc.add_block(BlockData::new());
    print!("{}", bc);
    BlockChain::is_valid_chain(bc.chain);
}

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
    // ApiResponse {
    //     json: json!({ "block": new_block }).into(),
    //     status: Status::Ok,
    // }
    // const block = blockchain.addBlock(req.body.data);
    // console.log(`New block added: ${block.toString()}`);
    // res.redirect('/blocks');
    let json = json!(*BC.lock().unwrap().chain).into();
    ApiResponse {
        json,
        status: Status::Ok,
    }
}

// #[post("/mine", data = "<block>")]
// fn mine(block: rocket::serde::json::Json<Block>) -> ApiResponse {
//     let new_block: Block = Block {
//         timestamp: block.timestamp,
//         last_hash: block.last_hash.clone(),
//         hash: block.hash.clone(),
//         data: block.data.clone(),
//         validator: block.validator.clone(),
//         signature: block.signature.clone(),
//     };

//     ApiResponse {
//         json: json!({ "block": new_block }).into(),
//         status: Status::Ok,
//     }
//     // const block = blockchain.addBlock(req.body.data);
//     // console.log(`New block added: ${block.toString()}`);

//     // res.redirect('/blocks');
// }

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        // Heroes
        blocks, blockchain, mine
    ]
}
