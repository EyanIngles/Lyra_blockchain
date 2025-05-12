mod block; // importing the modules created.
mod blockchain;
mod network;
mod wallet;
mod client;

use client::Path;
use crate::wallet::UserWallet;
use std::fs;
use std::path;
use blockchain::Blockchain;
use network::P2PNode;
use tokio::test;
// use wallet::UserWallet;
use std::sync::{Arc, Mutex};
use std::env;

#[tokio::main]
async fn main() { // need to ping and ensure that the server is running, this will then route the data and maybe have the server send the signal
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
        Path::StartServer => start_server(&p2p_node, args[2].to_string()).await,
        _ => todo!()
    };

    println!("command here::{:?}", command);


    let is_requesting_new_wallet = args[1] == "create_wallet";
    let wallet_name;

    // ✅ Load the blockchain from file instead of creating a new one
    // mental note, I think that this is just recreating a new blockchain eachtime so a new one is being started.

    // okay, so i need to have a json file that exists and ill have it so that if it isnt found, then create a new file with the file name.
    // then we will refer to the blockchain details and make sure they match otherwise return err.

    if is_requesting_new_wallet { // this generates a new wallet and attached a name to it. 
        wallet_name = args[2].clone();
        let my_wallet = UserWallet::generate_new_wallet(wallet_name.to_string()); // this does work.
        println!("new wallet:: {:?}", my_wallet);
        return
    }    
    
    if command == Path::GetWallet { // this is new block..... not getwallet, get wallet is for testing..
        let address = format!("127.0.0.1:8080");
        let blockchain_update = blockchain_to_write.lock().unwrap();
        //println!("blockchain_update variable: {:?}", blockchain_update);
        //println!("args[2]: {:?}", args[2]);
        if command == Path::GetBlock {
            
            assert!(args[2] != ""); // asserting that args[2] is not an empty string
            let block_number: usize = args[2].clone().parse().unwrap();
            let block = blockchain_update.get_block_via_index(block_number); 
            if block == None {
                println!("err: Unable to find block info - value has returned as 'none'");
                return
            }
            println!("Block details: {:?}", block);
            return 
        }; // else we are expecting it to be data that is wanting to be passed to a new block and added to the chain.

       // let block_data = args[1].clone();
       // let new_block_data = block_data.replace("_",  " ");
       // //println!("testing this section to replace _ with an empty space{:?}", new_block_data);
       // blockchain_update.add_block_to_chain(new_block_data.clone());
//
       // println!("✅ New block added! Latest block: {:?}", blockchain_update.get_last_block());
       let hash = blockchain_update.get_last_block_hash().clone();
       p2p_node.connect_to_peer(&address, hash).await; // sending the message off to the peer.

    }
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
    return
}
async fn new_blockchain() -> Blockchain {
    let blockchain = Blockchain::new();
    return blockchain
}
async fn start_server(p2p_node: &P2PNode, address: String){ // TODO:  will want to write another json file to keep track of what servers are
    // live and which are not live aswell as validators so that they are able to be pinged.
    if address == "default" || address == "" {
        p2p_node.start_server("127.0.0.1:8080").await;
        //TODO will want to ping to see if socket is clear and then run that socket address if clear.
    } else {
        p2p_node.start_server(&address).await; //TODO will want to ping to see if socket is clear and then run that socket address if clear.
    }
    // setting to 0, will basically need #TODO is to have 0 as a no so the value must changed otherwise revert.
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
    let (last_index, _last_hash) = Blockchain::get_last_block_index_and_hash(&blockchain);
    let last_hashh = Blockchain::get_last_block_hash(&blockchain);
    assert_ne!(index1, 2,"index: {:?} last_index: {:?}", index1, last_index);
    assert_ne!(hash1, last_hashh, "hash: {:?} last_hash: {:?}", hash1, last_hashh);
}
#[test]
pub async fn testing_connecting_to_peer_not_complete() { 
    let blockchain = Blockchain::new();
    let _p2p_node = P2PNode::new(Arc::new(Mutex::new(blockchain.clone())));
    //P2PNode::start_server(&p2p_node, "127.0.0.1:8080").await; // starting default server. 
    // The above goes on a forever loop so testing will need to kill the server after test. 

}
