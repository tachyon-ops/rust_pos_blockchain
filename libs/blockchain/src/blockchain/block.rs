use std::fmt;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use super::data::BlockData;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub timestamp: i64, // https://stackoverflow.com/a/68198507/5954864
    pub last_hash: String,
    pub hash: String,
    pub data: BlockData,
    pub validator: Option<String>,
    pub signature: Option<String>,
}

// impl fmt::Display for Vec<Data> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!("")
//     }
// }

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: fix data
        let data = self.data.clone();
        let validator: String;
        match &self.validator {
            Some(val) => {
                validator = String::from(val);
            }
            _ => validator = String::from("no validator"),
        }
        let signature: String;

        match &self.signature {
            Some(val) => {
                signature = String::from(val);
            }
            _ => signature = String::from("no signature"),
        }

        write!(
            f,
            "Block - 
      |-Timestamp : {}
      |-Last Hash : {}
      |-Hash      : {}
      |-Data      : {:?}
      |-Validator : {}
      └-Signature : {}\n",
            self.timestamp, self.last_hash, self.hash, data, validator, signature
        )
    }
}

impl Block {
    pub fn genesis() -> Block {
        // return new this(`genesis time`, "----", "genesis-hash", []);
        Block {
            timestamp: chrono::Utc::now().timestamp(),
            last_hash: String::from("----"),
            hash: String::from("genesis-hash"),
            data: BlockData::new(),
            validator: None,
            signature: None,
        }
    }

    pub fn hash(timestamp: i64, last_hash: String, data: BlockData) -> String {
        let timestamp = timestamp;
        let last_hash = last_hash;
        // TODO: fix data
        // let data = "";
        println!("Hashing w/ rep of data: {:?}", data);
        let mut sha = Sha256::new();
        sha.input_str(format!("{}{}{:?}", timestamp, last_hash, data).as_str());
        sha.result_str()
    }

    pub fn create_block(mut last_block: Block, data: BlockData) -> Block {
        let timestamp = chrono::Utc::now().timestamp();
        let last_hash_mut = last_block.hash.as_mut_str();
        let last_hash = last_hash_mut.to_string();

        let hash = Block::hash(timestamp, last_hash.to_string(), data);
        Block {
            timestamp,
            last_hash,
            hash,
            data: BlockData::new(),
            validator: None,
            signature: None,
        }
    }
}
