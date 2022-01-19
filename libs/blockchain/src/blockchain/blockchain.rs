use std::fmt;

use super::{block::Block, data::BlockData};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chain: String = String::from("");
        let chain_iter = self.chain.iter();
        for block in chain_iter {
            chain = String::from(format!("{}   └- {}", chain, block));
        }
        write!(f, "BlockChain\n └- Chain {}\n", chain)
    }
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: BlockData) -> Block {
        let last_block = self.chain.last().unwrap().to_owned();
        let block = Block::create_block(last_block, data);
        self.chain.push(block.clone());
        block
    }

    fn block_hash(block: Block) -> String {
        Block::hash(block.timestamp, block.last_hash, block.data)
    }

    pub fn is_valid_chain(chain: Vec<Block>) -> bool {
        if chain[0].to_string() != Block::genesis().to_string() {
            return false;
        }

        for (pos, block) in chain.iter().enumerate() {
            if pos > 0 {
                let last_block = chain[pos - 1].clone();
                if block.last_hash != last_block.hash
                    || block.hash != BlockChain::block_hash(block.to_owned())
                {
                    return false;
                }
            }
            println!("Element at position {} is VALID: \n{}", pos, block);
        }
        return true;
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) {
        if new_chain.len() <= self.chain.len() {
            println!("Recieved chain is not longer than the current chain");
            return;
        } else if !BlockChain::is_valid_chain(new_chain.clone()) {
            println!("Recieved chain is invalid");
            return;
        }
        println!("Replacing the current chain with new chain");
        self.chain = new_chain.clone();
    }
}
