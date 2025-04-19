use rand::Rng;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use serde_derive::Deserialize;
use crate::blockchain::Blockchain;
use std::fs;
use std::path;
use std::time::Duration;
use tokio::{time::sleep, spawn};


pub struct P2PNode {
    pub _blockchain: Arc<Mutex<Blockchain>>,
}
#[derive(serde_derive::Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Network { // this network will be used into a json file to keep track of all users and which are active to then ping.
    id: u8,
    is_active: bool,
    address: String,
}
#[derive(serde_derive::Serialize, Deserialize, Clone, Debug)]
pub struct Cluster {
    pub networks: Vec<Network>,
}

impl P2PNode {
    pub fn new(_blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNode { _blockchain }
    }
    
    pub async fn start_server(&self, address: &str) { // i think that the sockets or IP address when a node is started should be saved somewhere and then used
        // that address to ping to see if it is connectable.
        let network_path = "./network.json";
        let mut network_cluster = if !path::Path::new(network_path).exists() { // if file does not exist, then create new without any checks
            print!("\nno server found, creeating new cluster.... \nplease wait...\n");
            let new_network = Network {
                id: 1,
                is_active: true,
                address: address.to_string()
            };
            let mut new_cluster = Cluster {
                networks: vec![],
            };
            new_cluster.networks.push(new_network.clone());
            let content = serde_json::to_vec(&new_cluster).expect("could not searlise");
            let _ = fs::write(network_path, content).expect("could not write new file for network.json");
            new_cluster
        } else { // else file exists, check to see if the user has an account if not create a new id
            print!("Network cluster found... \nFetching data now, Please wait....\n");
            let file = fs::read(network_path).expect("could not find file path");
            let network: Cluster = serde_json::from_slice(&file).expect("could not Deserialize");
            network
        };
        // attempt to connect to peers - 
        //let mut stream = TcpStream::connect("127.0.0.1:8080").await.expect("unable to connect to address");
        //let _ = stream.write_all(b"hello world!").await;

        let listener = TcpListener::bind(address).await.expect("Failed to bind server"); // we are calling start server function.
        let mut _network_id = 0;
        let _network = for item in &network_cluster.networks {
            if item.address == address {
                _network_id = item.id;
                println!("your id is:: {:?}", _network_id);
                break;
            } else {
                print!("unable find your address and id... \nadding you into the system...\n");
                let next_id = network_cluster.networks.len();
                let new_network = Network {
                    id: next_id.try_into().unwrap(),
                    is_active: true,
                    address: address.to_string()
                };
                network_cluster.networks.push(new_network);
                let de = serde_json::to_vec(&network_cluster).expect("unable to Searlise");
                let _ = fs::write(network_path, de);
                break
            }
        };

        println!("✅ P2P Server listening on {}", address);
        println!("Your P2P ID is: {}", _network_id);

        loop {
            let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");

            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                #[warn(unreachable_patterns)]
                match socket.read(&mut buffer).await {
                    Ok(size) => {
                        let recieved_data = String::from_utf8_lossy(&buffer[..size]);
                        println!("📩 Received: {}", recieved_data);

                        let response = "Block received";
                        socket.write_all(response.as_bytes()).await.expect("Failed to send response");
                    }
                    
                    Err(e) => {
                        println!("⚠️ Error reading data: {}", e);
                        return
                    }
                }
            });
        }
        
    }
pub async fn monitor_network_cluster() {
    spawn(async move {
        loop {
            // get updated cluster list.
            let cluster = P2PNode::updated_cluster();
            let number: usize = P2PNode::random_number(cluster.clone()).try_into().unwrap(); // clone is fine if needed
            let is_active = cluster.networks[number].is_active;
            let address = cluster.networks[number].address.clone();

            if is_active {
                println!("✅ top");
                let connection = TcpStream::connect(address.clone());
                if connection.await.is_err() {
                    print!("unable to get a hold of address socket, changing their status to false.");
                    let new_cluster = P2PNode::changing_network_status(cluster.clone(), number, false, address.clone()).await;
                    let serde_cluster = serde_json::to_vec(&new_cluster).expect("was unable to desearlise.");
                    fs::write("./network.json", serde_cluster).expect("unable to write");
                }
                println!("connected to address: {}", &address)
            } else {
                println!("❌ bottom");
            }

            sleep(Duration::from_secs(1)).await; // Shortened for realism
        }
    });
}

pub async fn changing_network_status(mut cluster: Cluster, i: usize, status: bool, address: String) -> Cluster {
    let check = cluster.networks[i].address.clone();
    if check != address {
        panic!("ERR: Addresses do not match up - check failed....")
    }
    let network = cluster.networks.get_mut(i);
    network.unwrap().is_active = status;
    // write it back to the network with the updated list.
    cluster
}

pub fn random_number(cluster: Cluster) -> u32 { //TODO need to create a check to ensure that they are meant to be active.
    let top_range: u32 = cluster.networks.len().try_into().unwrap();
    let mut ii:u32 = 1000000;
    while ii >= top_range {
        let mut rng = rand::thread_rng();
        ii = rng.gen_range(0..top_range);
        println!("rng: {:?}", ii)
    }
    ii
}
pub fn updated_cluster() -> Cluster {
    let file = fs::read("./network.json").expect("could not find file path");
    let network: Cluster = serde_json::from_slice(&file).expect("could not Deserialize");
    network
}
            // connecting to another node
pub async fn connect_to_peer(&self, address: &str, data: String) {
    match TcpStream::connect(address).await {
        Ok(mut stream) => {
            println!("🔗 Connected to peer at {}", address);
            stream.write_all(data.as_bytes()).await.expect("Failed to send data");

            let mut buffer = vec![0; 1024];
            match stream.read(&mut buffer).await {
                Ok(size) => {
                    let response = String::from_utf8_lossy(&buffer[..size]);
                    println!("📨 Response from peer: {}", response);
                }
                Err(e) => println!("⚠️ Error reading response: {}", e),
            }
        },
        Err(e) => {println!("❌ Failed to connect to {}: {}", address, e)},
    } 
}
}

