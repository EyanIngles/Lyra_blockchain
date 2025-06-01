mod block; // importing the modules created.
mod blockchain;
mod client;
mod network;
mod wallet;

use crate::wallet::UserWallet;
use blockchain::Blockchain;
use client::Path;
use network::P2PNode;
use std::env;
use std::fs;
use std::path;
use std::sync::{Arc, Mutex};
use tokio::test;

#[tokio::main]
async fn main() {
    // need to ping and ensure that the server is running, this will then route the data and maybe have the server send the signal
    // of the blockchain so that the data can be all sent and stored on that blockchain, this method, we are creating a new blockchain every
    // time we are calling 'cargo run' we need conditionals to stop this from happening.

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("❌ Error: Missing argument (args... or_use_this_method_to_publish_data...)");
        return;
    }
    // sort the command out function here.
    let command_arg: &str = &args[1];
    let command = client::sort_client_args_direction(command_arg);

    let path = "./blockchain.json";
    let blockchain = if !path::Path::new(path).exists() {
        let blockchain = new_blockchain().await;
        println!("blockchain does not exist, creating a new file now..");
        let data = serde_json::to_vec(&blockchain).expect("did not searlise");
        let _ = fs::write("./blockchain.json", data);
        blockchain
    } else {
        println!("\nblockchain found and fetching data now... \nplease wait.. \n");
        let file = fs::read(path).expect("unable to open blockchain file...");
        let blockchain: Blockchain = serde_json::from_slice(&file).expect("erorr...");
        blockchain
    };

    let blockchain_to_write = Arc::new(Mutex::new(blockchain));
    let p2p_node = P2PNode::new(blockchain_to_write.clone());

    // create a match to then point it the a function.
    match command {
        Path::NewBlock => create_new_block(blockchain_to_write.clone(), &args[2]),
        Path::GetBlock => get_block(blockchain_to_write.clone(), args[2].to_string()).await,
        Path::StartServer => start_server(&p2p_node, args[2].to_string()).await,
        Path::CreateWallet => create_wallet(args[2].clone()).await,
        _ => todo!(),
    };

    println!("command here::{:?}", command);
}
pub fn create_new_block(blockchain: Arc<Mutex<Blockchain>>, data: &str) {
    //let main_server = format!("127.0.0.1:8080"); // main sever..

    // Lock the blockchain (expect will unwrap or panic if poisoned)
    let mut blockchain_lock = blockchain.lock().expect("Could not lock the blockchain");
    blockchain_lock.add_block_to_chain(data.to_string());
    println!("\n Blockchain details: {:?}", blockchain_lock);

    let blockchain_copy = blockchain_lock.clone();
    drop(blockchain_lock);

    let data_update = serde_json::to_vec(&blockchain_copy).expect("did not searlise");
    let _ = fs::write("./blockchain.json", data_update);
    // TODO: create a peer to peer connection with a message of the block. then that message can be
    // added to each validator? could be more efficient.
    return;
}
async fn new_blockchain() -> Blockchain {
    let blockchain = Blockchain::new();
    return blockchain;
}
async fn start_server(p2p_node: &P2PNode, address: String) {
    if address == "default" || address == "" {
        p2p_node.start_server("127.0.0.1:8080").await;
        //TODO will want to ping to see if socket is clear and then run that socket address if clear.
    } else {
        p2p_node.start_server(&address).await; //TODO will want to ping to see if socket is clear and then run that socket address if clear.
    }
    // setting to 0, will basically need #TODO is to have 0 as a no so the value must changed otherwise revert.
}
async fn get_block(blockchain: Arc<Mutex<Blockchain>>, arg1: String) {
    let index: usize = arg1.parse().expect("Err: Arg is not a number");
    let blockchain_lock = blockchain.lock().expect("Could not lock the blockchain");
    let indexed_block = blockchain_lock.get_block_via_index(index);
    println!("blockchain Data: {:?}", indexed_block);
}

async fn create_wallet(name: String) {
    // create check to ensure there is a name and the name is not already in the data base to
    // ensure not doubles.
    let _new_wallet = UserWallet::generate_new_wallet(name);
}

#[test]
async fn test_creating_2_blocks() {
    let mut blockchain = Blockchain::new(); // creating the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); // creating a new block
    assert_eq!(blockchain.block[1].data, "new block created".to_string());
    Blockchain::add_block_to_chain(&mut blockchain, "new block created, 2".to_string()); // creating a new block again
    assert_eq!(blockchain.block[2].data, "new block created, 2".to_string()); //checking that the new block has been pushed.
    let block2hash = Blockchain::get_any_block_hash(&blockchain, 1); // block 2 is index 1.
    let block_previous_hash = Blockchain::get_index_block_previous_hash(&blockchain, 2); // this is block 3 but the index is 2.
    assert_eq!(
        block2hash, block_previous_hash,
        "first hash: {:?} second hash: {:?}",
        block2hash, block_previous_hash
    );
}
#[test]
async fn test_fn_get_last_block_index_and_hash() {
    let mut blockchain = Blockchain::new(); // creating the blockchain.
    Blockchain::add_block_to_chain(&mut blockchain, "new block created".to_string()); // creating a new block
    let (index, hash) = Blockchain::get_last_block_index_and_hash(&mut blockchain);
    let indexed_hash = Blockchain::get_any_block_hash(&mut blockchain, 1);
    assert_eq!(index, 1, "index: {:?}", index);
    assert_eq!(
        hash, indexed_hash,
        "expected hash: {:?}, hash got after function: {:?}",
        hash, indexed_hash
    );
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
    let (last_index, _last_hash) = Blockchain::get_last_block_index_and_hash(&blockchain);
    let last_hashh = Blockchain::get_last_block_hash(&blockchain);
    assert_ne!(
        index1, 2,
        "index: {:?} last_index: {:?}",
        index1, last_index
    );
    assert_ne!(
        hash1, last_hashh,
        "hash: {:?} last_hash: {:?}",
        hash1, last_hashh
    );
}
#[test]
pub async fn testing_connecting_to_peer_not_complete() {
    let blockchain = Blockchain::new();
    let _p2p_node = P2PNode::new(Arc::new(Mutex::new(blockchain.clone())));
    //P2PNode::start_server(&p2p_node, "127.0.0.1:8080").await; // starting default server.
    // The above goes on a forever loop so testing will need to kill the server after test.
}
