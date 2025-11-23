// this file is used to standarderise the expected values of lyra's UID

use epoch;
use rand::Rng;
use hex;
use bytes::Bytes;


pub struct LUID {
    id: String,
    //epoch: epoch
}

impl LUID {
    
   // pub fn new() -> LUID {
        
        
   // }

    fn generate_random_bytes() -> String {
        let num_bytes = 16;
        let mut random_bytes = vec![0u8; num_bytes];
        rand::thread_rng().fill(&mut random_bytes[..]);
        let hex_string = hex::encode(random_bytes);
        return hex_string;
    }


}


#[test]
pub fn test_random_generator() {
    let rnd_bytes = LUID::generate_random_bytes();
    println!("{:?}", rnd_bytes);
}
