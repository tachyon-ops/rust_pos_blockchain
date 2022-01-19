mod blockchain;

pub use crate::blockchain::Block;
pub use crate::blockchain::BlockChain;
pub use crate::blockchain::BlockData;

#[cfg(test)]
mod tests {
    use super::{BlockChain, BlockData};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn blockchain_is_valid_chain() {
        let mut bc = BlockChain::new();
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        bc.add_block(BlockData::new());
        assert_eq!(BlockChain::is_valid_chain(bc.chain), true);
    }
}
