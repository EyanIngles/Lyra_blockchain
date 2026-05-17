use crate::Arc;
use crate::Mutex;
use crate::block::Block;
pub use serde_derive::Deserialize;
use std::fs;
use std::path;

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Blockchain {
    pub chain_version: u8,
    pub chain_name: String,
    pub block: Vec<Block>,
}
// TODO: should we add a vector account with their names and wallets to show which are active on
// the network?

impl Blockchain {
    // Initialise the blockchain with genesis block
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
        let mut blockchain = Blockchain {
            chain_version: 1,
            chain_name: "Lyra".to_string(),
            block: vec![],
        };
        Self::push_block_onto_chain(&mut blockchain, genesis_block);
        blockchain
    }

    pub fn add_block_to_chain(&mut self, data: String) {
        let (last_index, previous_hash) = self.get_last_block_index_and_previous_hash();
        let new_index = last_index + 1;
        let block = Block::new(new_index, previous_hash, data);
        self.block.push(block);
    }

    //did we want to return something? TODO!
    pub async fn create_new_block(blockchain: Arc<Mutex<Blockchain>>, data: &str) {
        let mut blockchain_lock = blockchain.lock().expect("Could not lock the blockchain");
        blockchain_lock.add_block_to_chain(data.to_string());
        let blockchain_copy = blockchain_lock.clone();
        drop(blockchain_lock);

        let data_update = serde_json::to_vec(&blockchain_copy).expect("did not searlise");
        fs::write("./local_data/blockchain/blockchain.json", data_update)
            .expect("Err: Unable to write new blockchain block to blockchain.json.");
    }

    fn push_block_onto_chain(blockchain: &mut Blockchain, block: Block) {
        blockchain.block.push(block);
    }

    pub async fn get_blockchain() -> Blockchain {
        let path = "./local_data/blockchain/blockchain.json";
        if !path::Path::new(path).exists() {
            let blockchain = Self::new();
            println!("blockchain does not exist, creating a new file now..");
            let data = serde_json::to_vec(&blockchain).expect("did not searlise");
            fs::write("./local_data/blockchain/blockchain.json", data)
                .expect("Err: Unable to write new 'blockchain.json' file");
            blockchain
        } else {
            println!("\nblockchain found and fetching data now... \nplease wait.. \n");
            let file = fs::read(path).expect("unable to open blockchain file...");
            let blockchain: Blockchain = serde_json::from_slice(&file).expect("erorr...");
            blockchain
        }
    }

    pub fn get_last_block_index_and_previous_hash(&self) -> (usize, String) {
        let block = self.block.last();
        (block.unwrap().index, block.unwrap().hash.to_string())
    }

    pub fn get_block_via_index(&self, index: usize) -> Option<&Block> {
        self.block.get(index)
    }
}

#[allow(dead_code)]
impl Blockchain {
    //this is being used for testing.
    pub fn get_block_length(&self) -> usize {
        self.block.len()
    }

    pub fn get_last_block_hash(&self) -> String {
        let block = self.block.last();
        block.unwrap().hash.to_string()
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.block.last()
    }

    pub fn get_any_block_hash(&self, block_number: usize) -> String {
        let block = self.block.get(block_number);
        block.unwrap().hash.to_string()
    }

    pub fn get_any_block_index_and_hash(&self, block_number: usize) -> (usize, String) {
        let block = self.block.get(block_number);
        (block.unwrap().index, block.unwrap().hash.to_string())
    }

    pub fn get_last_block_index_and_hash(&self) -> (usize, String) {
        let block = self.block.last();
        (block.unwrap().index, block.unwrap().hash.to_string())
    }

    pub fn get_index_block_previous_hash(&self, index: usize) -> String {
        let block = self.block.get(index);
        block.unwrap().previous_hash.to_string()
    }
}

#[test]
fn test_new_blockchain() {
    let mut blockchain1 = Blockchain::new();
    Blockchain::add_block_to_chain(&mut blockchain1, "data".to_string());
    let mut blockchain2 = Blockchain::new();
    Blockchain::add_block_to_chain(&mut blockchain2, "data_2".to_string());

    assert_ne!(
        blockchain1, blockchain2,
        "blockchain 1: {:?} blockchain 2: {:?}",
        blockchain1, blockchain2
    )
}

#[test]
fn testing_function_is_attaching_previous_hash() {
    let mut blockchain = Blockchain::new();
    let data = "random data to push onto string.".to_string();
    let data2 = "random data to push at string.".to_string();
    let block = Blockchain::add_block_to_chain(&mut blockchain, data);
    let block2 = Blockchain::add_block_to_chain(&mut blockchain, data2);
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
