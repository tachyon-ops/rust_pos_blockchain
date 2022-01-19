mod blockchain;

pub use blockchain::Block;
pub use blockchain::BlockChain;
pub use blockchain::BlockData;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
