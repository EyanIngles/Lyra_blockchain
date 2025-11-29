// this file is used to standarderise the expected values of lyra's UID

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::offset::Utc;
use chrono::DateTime;
use sha2::{Sha256, Digest};
use rand::Rng;
use hex;
use bytes::Bytes;


#[derive(Debug)]
pub struct LUID {
    pub id: String,
    //epoch: epoch
}

impl LUID {
    
   pub fn new() -> LUID {
        let mut bytes = Self::generate_random_bytes();
        bytes.to_string();
        let time_string = Self::time_string();
        let to_hash = bytes + &time_string;
        let mut hasher = Sha256::new();
        hasher.update(to_hash.as_bytes());
        let hash_bytes = hasher.finalize();
        let hash = hex::encode(hash_bytes);

        let luid = LUID {
            id: hash
        };
        return luid;
   }

    fn generate_random_bytes() -> String {
        let num_bytes = 16;
        let mut random_bytes = vec![0u8; num_bytes];
        rand::thread_rng().fill(&mut random_bytes[..]);
        let hex_string = hex::encode(random_bytes);
        return hex_string;
    }
    
    fn time_string() -> String {
        let mut time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Err: Unable to conver time to epoch value");
        let time_u64: u64 = time.as_secs()/2;
        let time_string = time_u64.to_string();
        return time_string;
    }

}


#[test]
pub fn test_random_generator() {
    let rnd_bytes = LUID::generate_random_bytes();
    println!("{:?}", rnd_bytes);
}

#[test]
pub fn test_LUID_is_unique() {
    let mut list = Vec::new();
    list.push(LUID::new());
    let mut i = 0;
    while i < 5000 {
        let new_luid = LUID::new();
        for l in list.iter() {
            assert_ne!(l.id, new_luid.id);
        };
        list.push(new_luid);
        i += 1;
    };
}
