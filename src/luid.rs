// this file is used to standarderise the expected values of lyra's UID

use rand::Rng;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Luid {
    pub id: String,
    //epoch: epoch
}

impl Luid {
    #[allow(dead_code)]
    pub fn new() -> Luid {
        let bytes = Self::generate_random_bytes();
        bytes.to_string();
        let time_string = Self::time_string();
        let to_hash = bytes + &time_string;
        let mut hasher = Sha256::new();
        hasher.update(to_hash.as_bytes());
        let hash_bytes = hasher.finalize();
        let hash = hex::encode(hash_bytes);

        Luid { id: hash }
    }

    fn generate_random_bytes() -> String {
        let num_bytes = 16;
        let mut random_bytes = vec![0u8; num_bytes];
        rand::thread_rng().fill(&mut random_bytes[..]);
        hex::encode(random_bytes)
    }

    fn time_string() -> String {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Err: Unable to conver time to epoch value");
        let time_u64: u64 = time.as_secs() / 2;
        time_u64.to_string()
    }
}

#[test]
pub fn test_random_generator() {
    let rnd_bytes = Luid::generate_random_bytes();
    println!("{:?}", rnd_bytes);
}

#[test]
pub fn test_luid_is_unique() {
    let mut list = Vec::new();
    list.push(Luid::new());
    let mut i = 0;
    while i < 5000 {
        let new_luid = Luid::new();
        for l in list.iter() {
            assert_ne!(l.id, new_luid.id);
        }
        list.push(new_luid);
        i += 1;
    }
}
