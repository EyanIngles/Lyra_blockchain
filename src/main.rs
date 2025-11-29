mod block; // importing the modules created.
mod blockchain;
mod client;
mod network;
mod wallet;
mod transactions;
mod token;
mod lyst746F;
mod luid;

use crate::wallet::{UserWallet, WalletCache, Address};
use crate::token::TokenList;
use blockchain::Blockchain;
use client::Path;
use generic_array::GenericArray;
use hex::FromHex;
use k256::ecdsa::SigningKey;
use k256::SecretKey;
use network::Cluster;
use network::P2PNode;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::sync::{Arc, Mutex};
use tokio::test;
#[tokio::main]

async fn main() {
    // TODO: need to ping and ensure that the server is running, this will then route the data and maybe have the server send the signal of the blockchain so that the data can be all sent and stored on that blockchain, this method, we are creating a new blockchain every

    let args: Vec<String> = env::args().collect();
    

    /// testing here::
    let time_u64 = luid::LUID::new();
    println!("here is the function being called value -> {:?}", time_u64);

    if args.len() <= 2 {
        // TODO: rewrite this to point users towards command -help
        println!("Err: Missing or incorrect arguments; Run 'cargo run command -help' ");
        return;
    }
    // sort the command out function here.
    let command_arg: &str = &args[1];
    let command = client::sort_client_args_direction(command_arg);

    if &args[2] == "-help" {
        let command_help_path = "./command_list.json";

        let read_commands = fs::read_to_string(command_help_path)
            .expect("Err: Unable to read Command-help file path.");

        let command_help: HashMap<String, String> = serde_json::from_str(&read_commands)
            .expect("Err: Unable to read sliced file for commands -help");

        println!("{}", command_help[&args[1].to_string()]);
        return;
    }

    let path = "./local_data/blockchain/blockchain.json";
    let blockchain = if !path::Path::new(path).exists() {
        let blockchain = new_blockchain().await;
        println!("blockchain does not exist, creating a new file now..");
        let data = serde_json::to_vec(&blockchain).expect("did not searlise");
        let _ = fs::write("./local_data/blockchain/blockchain.json", data);
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
        Path::NewBlock => create_new_block(&p2p_node, blockchain_to_write.clone(), &args[2]).await,
        Path::GetBlock => get_block(blockchain_to_write.clone(), args[2].to_string()).await,
        Path::StartServer => start_server(&p2p_node, args[2].to_string()).await,
        Path::CreateWallet => create_wallet(args[2].clone()).await,
        Path::WalletLogin => wallet_login(args[2].clone(), args[3].clone()).await,
        Path::WalletLogout => wallet_logout(args[2].clone(), args[3].clone()).await,
        Path::ImportWallet => {
            import_wallet(args[2].clone(), args[3].clone(), args[4].clone()).await
        }
        _ => todo!(),
    };
}
async fn create_new_block(p2p_node: &P2PNode, blockchain: Arc<Mutex<Blockchain>>, data: &str) {
    let mut blockchain_lock = blockchain.lock().expect("Could not lock the blockchain");
    blockchain_lock.add_block_to_chain(data.to_string());
    let blockchain_copy = blockchain_lock.clone();
    drop(blockchain_lock);

    let data_update = serde_json::to_vec(&blockchain_copy).expect("did not searlise");
    let _ = fs::write("./local_data/blockchain/blockchain.json", data_update);
    // TODO: create a peer to peer connection with a message of the block. then that message can be
    // added to each validator? could be more efficient.
    let network_path = "./network.json";
    let addresses = fs::read(network_path).expect("Err: Unable to find file.");
    let address_cluster: Cluster =
        serde_json::from_slice(&addresses).expect("Err: Unable to retrieve data from file.");
    //

    for network in address_cluster.networks.clone() {
        println!("going through the networks now...");
        if network.is_active == true {
            P2PNode::connect_to_peer(&p2p_node, network.address.as_str(), &data).await;
        } else {
            println!("network is not active: {:?}", network);
        }
    }
    return;
}

async fn new_blockchain() -> Blockchain {
    let blockchain = Blockchain::new();
    return blockchain;
}

async fn start_server(p2p_node: &P2PNode, address: String) {
    if address == "default" || address == "" {
        if !path::Path::new("./network.json").exists() {
            p2p_node.creating_server("127.0.0.1:8080").await
        }
        p2p_node.start_server("127.0.0.1:8080").await;
        //TODO: will want to ping to see if socket is clear and then run that socket address if clear.
    } else {
        if !path::Path::new("./network.json").exists() {
            p2p_node.creating_server(&address).await
        }
        p2p_node.start_server(&address).await; //TODO will want to ping to see if socket is clear and then run that socket address if clear.
    }
}

async fn get_block(blockchain: Arc<Mutex<Blockchain>>, arg1: String) {
    let index: usize = arg1.parse().expect("Err: Arg is not a number");
    let blockchain_lock = blockchain.lock().expect("Could not lock the blockchain");
    let indexed_block = blockchain_lock.get_block_via_index(index);
    println!("blockchain Data: {:?}", indexed_block);
}

async fn create_wallet(name: String) {
    // TODO: create check to ensure there is a name and the name is not already in the data base to ensure not doubles.
    let _new_wallet = UserWallet::generate_new_wallet(name);
}

async fn wallet_login(_wallet_name: String, _wallet_password: String) {
    let path = "./localCache.json";
    let wallet;
    if path::Path::new(path).exists() {
        let file = fs::read(path).expect("Err: Unable to open localCache file...");
        let wallet_cache: WalletCache = serde_json::from_slice(&file).expect("Err:");
        if &_wallet_password != &wallet_cache.password {
            panic!("Err: Password input incorrect to use this wallet.");
        }
        if &_wallet_name != &wallet_cache.wallet_info.name {
            panic!("Err: Wallet with that name does not exist.");
        }
        wallet = wallet_cache;
    } else {
        println!("Err: you must import a wallet, you can do this via the follow command; 'cargo run import-wallet <private-key> <password>");
        return;
    };

    println!("Wallet details detials are here: {:?}", wallet);
}

async fn wallet_logout(_wallet_name: String, _wallet_password: String) {
    // TODO: should take string or password and ensure that the user is
    // really the one that wants the wallet to be logged out.
    let path = "./localCache.json";
    if path::Path::new(path).exists() {
        fs::remove_file(path).expect("Err: Unable to remove localCache file...");
        println!("NOTE: Thank you for logging out. All local data is now deleted.");
    } else {
        println!("WARNING: No login detected, Logout may have been called already..");
        println!("NOTE: If you need help, you can see all commands by running 'cargo run <command> -h' or to see list of commands by running 'cargo run command-list'");
        return;
    }
}

async fn import_wallet(wallet_name: String, private_key: String, wallet_password: String) {
    // println!("Err: The provided private key has a invalid length, Please double check and try again.");
    let key_bytes: [u8; 32] = <[u8; 32]>::from_hex(private_key)
        .expect("Err: Unable to convert private key to bytes, please check your private key");
    // Convert to GenericArray
    let key_array = GenericArray::from_slice(&key_bytes);
    let secret_key =
        SecretKey::from_bytes(key_array).expect("Err: Unable to convert secret key to bytes.");
    let signing_key = SigningKey::from(secret_key);
    let public_key = signing_key.verifying_key();

    let public_key_hex = hex::encode(public_key.to_encoded_point(false).as_bytes());
    println!("here is your public key: {:?}", public_key_hex);

    // TODO: check to see if there is a path file and if not, create and import wallet, store
    // one wallet at a time for the time being.
    let path = "./localCache.json";
    if path::Path::new(&path).exists() {
        // is existing, writing over the old data for the time being.
        let file = fs::read(path).expect("Err: Unable to read file");
        let data: WalletCache =
            serde_json::from_slice(&file).expect("Err: Unable to read files from local cache");
        println!(
            "reading file... : {:?}\nhave not implemented writing of new yet",
            data
        );
        //TODO: either rewrite over or create another section and write a new wallet on there.
    } else { 
        let user_info = UserWallet {
            name: wallet_name, // hardcoded for the moment.
            address: Address { public_key: public_key_hex },
            currency_accounts: TokenList::new(), // TODO: will need to make it so we read from the blockchain and
                                       // fetch these details from the public key generation.
        };

        let wallet_cache = WalletCache {
            wallet_info: user_info,
            private_key: key_bytes,
            password: wallet_password,
        };
        let data = serde_json::to_vec(&wallet_cache).expect("did not searlise");
        let _ = fs::write("./localCache.json", data);
    }
}

// TODO: import wallet to local cache and which will take the private keys in an [u8] and a
// password.

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
