use aes_gcm::aead::OsRng;
use hex;
use k256::ecdsa::SigningKey;
use scanpw::scanpw;
use serde_derive::Deserialize;
use std::fs;

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Currency {
    pub name: String,
    pub amount: u128,
}

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserWallet {
    pub name: String,
    pub address: String,
    pub currency_accounts: Vec<Currency>,
}

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WalletCache {
    pub wallet_info: UserWallet,
    pub private_key: [u8; 32],
    pub password: String,
}

impl UserWallet {
    pub fn generate_new_wallet(name: String) -> UserWallet {
        // TODO: add name for native currency to then use
        // for the currency section. and create a checker list to ensure that the name does
        // not exist.
        let password = scanpw!("Enter Password: "); // password may not be needed here....
        let (private_key, public_key) = Self::generate_keys();
        let _current_account = Currency {
            name: name.clone(),
            amount: 0,
        };
        let wallet = UserWallet {
            name: name.clone(),
            address: public_key.to_string(),
            currency_accounts: vec![],
        };
        println!("New Wallet Users: {:?}", wallet.name);
        println!("Private Key: {:?}", private_key);
        println!("Public Key: {:?}", public_key);
        let password_key = password.into_bytes();
        println!("password encrpypted as bytes ;) {:?}", password_key);
        //TODO: will need to add to the list of current users. this will be used for searching up
        //other users via their name or public address.
        return wallet;
    }

    fn generate_keys() -> (String, String) {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = private_key.verifying_key();

        let private_key_hex = hex::encode(private_key.to_bytes());
        let public_key_hex = hex::encode(public_key.to_encoded_point(false).as_bytes());

        (private_key_hex, public_key_hex)
    }

    // TODO: let this be used to encrypt the passwords before being saved and then saved as a local
    // pem file that is encrypted and another function to descrypt it.
    fn encrypt_local_wallet() {
        // TODO: save keys to a pem file created. Should this be the process of the login wallet?
        //let private_file_name = name.clone() + "private_key.pem";
        //let public_file_name = name.clone() + "public_key.pem";
        //let password_key = password.into_bytes();
        //println!("{:?}", password_key);
        //let _public_file = fs::write(public_file_name, public_key.as_bytes());
        //let _privale_file = fs::write(private_file_name, private_key.as_bytes());
    }
    //TODO: pub fn transfer_currency(&mut self, ) // create function to transfer token and amount
    //to another address, if address doesnt exist on chain, bounce back or abort.
}

#[test]
fn test_wallet_generating() {
    let wallet1 = UserWallet::generate_new_wallet("wallet1".to_string(), "password1".to_string());
    let wallet2 = UserWallet::generate_new_wallet("wallet2".to_string(), "password2".to_string());

    assert_ne!(wallet1, wallet2);
}
