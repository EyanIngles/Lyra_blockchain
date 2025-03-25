mod block; // importing the modules created.
mod blockchain;
mod network;
mod wallet;

use blockchain::Blockchain;
use network::P2PNode;
use tokio::test;
use wallet::UserWallet;
use std::sync::{Arc, Mutex};
use std::env;

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    println!("args input here: {:?}", args);

    if args.len() < 2 {
        println!("❌ Error: Missing argument (server/client)");
        return;
    }

    let is_server = args[1] == "server";

    // ✅ Load the blockchain from file instead of creating a new one
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let p2p_node = P2PNode::new(blockchain.clone());
    let my_wallet = UserWallet::generate_new_wallet("Eyan".to_string());

    if is_server {
        p2p_node.start_server("127.0.0.1:8080").await;
    } else {
        let address = format!("127.0.0.1:8080");


        let mut blockchain_update = blockchain.lock().unwrap();

        let new_block_data = "new block created".to_string();
        blockchain_update.add_block_to_chain(new_block_data.clone());

        println!("✅ New block added! Latest block: {:?}", blockchain_update.get_last_block());

        let hash = blockchain_update.get_last_block_hash().clone();
        p2p_node.connect_to_peer(&address, my_wallet.name).await;
    }
}
#[test]
    async fn test_creating_2_blocks() {
    let mut blockchain = Blockchain::new(); // creating the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); // creating a new block
    assert_eq!(blockchain.chain[1].data, "new block created".to_string());
    Blockchain::add_block_to_chain(&mut blockchain, "new block created, 2".to_string()); // creating a new block again
    assert_eq!(blockchain.chain[2].data, "new block created, 2".to_string()); //checking that the new block has been pushed.
    let block2hash = Blockchain::get_any_block_hash(&blockchain, 1); // block 2 is index 1.
    let block_previous_hash = Blockchain::get_index_block_previous_hash(&blockchain, 2); // this is block 3 but the index is 2.
    assert_eq!(block2hash, block_previous_hash, "first hash: {:?} second hash: {:?}",block2hash, block_previous_hash );
}
#[test]
async fn test_fn_get_last_block_index_and_hash() {
    let mut blockchain = Blockchain::new(); // creating the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); // creating a new block
    let (index, hash) = Blockchain::get_last_block_index_and_hash(&mut blockchain);
    let indexed_hash = Blockchain::get_any_block_hash(&mut blockchain, 1);
    assert_eq!(index, 1 , "index: {:?}", index);
    assert_eq!(hash, indexed_hash, "expected hash: {:?}, hash got after function: {:?}", hash, indexed_hash);
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); // creating 2 new blocks
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); 
}
#[test]
async fn test_indexed_functions_work() {
    let mut blockchain = Blockchain::new(); // creating the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "index1".to_string()); // creating multiple blocks in the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "index2".to_string()); 
    Blockchain::add_block_to_chain(&mut blockchain, "index3".to_string()); 
    Blockchain::add_block_to_chain(&mut blockchain, "index4".to_string()); 
    Blockchain::add_block_to_chain(&mut blockchain, "index5".to_string()); 
    let (index1, hash1) = Blockchain::get_any_block_index_and_hash(&blockchain, 0);
    let (last_index, last_hash) = Blockchain::get_last_block_index_and_hash(&blockchain);
    let (index4, hash4) = Blockchain::get_any_block_index_and_hash(&blockchain, 4);
    let last_hashh = Blockchain::get_last_block_hash(&blockchain);
    assert_ne!(index1, 2,"index: {:?} last_index: {:?}", index1, last_index);
    //assert_ne!(hash1, last_hashh, "hash: {:?} last_hash: {:?}", hash1, last_hashh);
}


