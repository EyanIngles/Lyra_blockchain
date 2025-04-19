use crate::block::Block;
pub use serde_derive::Deserialize;


#[derive(serde_derive::Serialize, Deserialize)]
#[derive(Debug, PartialEq, Clone)]
pub struct Blockchain {
    pub chain_version: u8,
    pub chain_name: String,
    pub block: Vec<Block>,
}

impl Blockchain {

    // Initialise the blockchain with genesis block
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
        let mut blockchain = Blockchain {
            chain_version: 1,
            chain_name: "Mizu".to_string(),
            block: vec![]
        };
        Self::push_block_onto_chain(&mut blockchain, genesis_block);
        blockchain
    }

  
    pub fn add_block_to_chain(&mut self, data: String) { // doesnt push new block onto the array, it over writes it the existing block.
        let (last_index, previous_hash) = self.get_last_block_index_and_previous_hash();
        let new_index = last_index +1;
        let block = Block::new(new_index, previous_hash, data);
        self.block.push(block.clone());
        println!("block info here: {:?}", block.clone());
    }
    fn push_block_onto_chain(blockchain: &mut Blockchain, block: Block) {
        blockchain.block.push(block);
    }

    pub fn get_last_block_index_and_previous_hash(&self) -> (usize, String) {
        let block = self.block.last().clone();
        let index = block.unwrap().index;
        let hash = block.unwrap().hash.to_string();
        return (index, hash)
    }
    #[warn(dead_code)]
    pub fn get_index_block_previous_hash(&self, index: usize) -> String {
        let block = self.block.get(index).clone();
        let hash = block.unwrap().previous_hash.to_string();
        return hash
    }
    pub fn get_block_via_index(&self, index: usize) -> Option<&Block> {
        let block = self.block.get(index).clone();
        return block
    }
    #[warn(dead_code)]
    pub fn get_last_block_index_and_hash(&self) -> (usize, String) {
        let block = self.block.last().clone();
        let index = block.unwrap().index;
        let hash = block.unwrap().hash.to_string();
        return (index, hash)
    }
    #[warn(dead_code)]
    pub fn get_any_block_index_and_hash(&self, block_number: usize) -> (usize, String) {
        let block = self.block.get(block_number).clone();
        let index = block.unwrap().index;
        let hash = block.unwrap().hash.to_string();
        return (index, hash)
    }
    #[warn(dead_code)]
    pub fn get_any_block_hash(&self, block_number: usize) -> String {
        let block = self.block.get(block_number).clone();
        let hash = block.unwrap().hash.to_string();
        return hash
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        let block = self.block.last().clone();
        return block
    }

    pub fn get_last_block_hash(&self) -> String {
        let block = self.block.last().clone();
        let hash = block.unwrap().hash.to_string();
        return hash
    }
    #[warn(dead_code)] //this is being used for testing.
    pub fn get_block_length(&self) -> usize {
        let block_length = self.block.len().clone();
        return block_length
    }

    
}

#[test]
fn test_new_blockchain() {
    let mut blockchain1 = Blockchain::new();
    Blockchain::add_block_to_chain(&mut blockchain1, "data".to_string());
    let mut blockchain2 = Blockchain::new();
    Blockchain::add_block_to_chain(&mut blockchain2, "data_2".to_string());
    
    assert_ne!(blockchain1, blockchain2, "blockchain 1: {:?} blockchain 2: {:?}",blockchain1, blockchain2)
}

#[test]
fn testing_function_is_attaching_previous_hash() {
    let mut blockchain = Blockchain::new();
    let data = "random data to push onto string.".to_string();
    let data2 = "random data to push at string.".to_string();
    let block = Blockchain::add_block_to_chain(&mut blockchain, data.clone());
    let block2 = Blockchain::add_block_to_chain(&mut blockchain, data2.clone());
    println!("block data here: {:?}", block);

    assert_eq!(block, block2, "block data {:?}", block);
}

#[test]
fn testing_blocks_are_being_added_to_chain() {
    let mut blockchain = Blockchain::new();
    let data = "random Data".to_string();
    let data2 = "random Data2".to_string();
    let _block1 = Blockchain::add_block_to_chain(&mut blockchain, data);
    let _block2 = Blockchain::add_block_to_chain(&mut blockchain, data2);
    let chain_length = Blockchain::get_block_length(&blockchain);
    assert_eq!(chain_length, 3);
}