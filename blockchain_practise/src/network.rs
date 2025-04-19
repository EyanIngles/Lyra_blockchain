use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;


pub struct P2PNode {
    pub _blockchain: Arc<Mutex<Blockchain>>,
}

impl P2PNode {
    pub fn new(_blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNode { _blockchain }
    }
    
    pub async fn start_server(&self, address: &str) { // i think that the sockets or IP address when a node is started should be saved somewhere and then used
        // that address to ping to see if it is connectable.
        let listener = TcpListener::bind(address).await.expect("Failed to bind server"); // we are calling start server function.

        println!("✅ P2P Server listening on {}", address);

        loop {
            let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
            //let blockchain = self.blockchain.clone(); // not used

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

