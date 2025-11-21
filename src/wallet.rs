use aes_gcm::aead::OsRng;
use bip39::Mnemonic;
use hex;
use k256::ecdsa::SigningKey;
use scanpw::scanpw;
use serde_derive::Deserialize;

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Currency {
    pub name: String,
    pub amount: u128,
}

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Address {
    uid: u64, //generate a unique ID that is one of a kind.
    public_key: String // general publickey for tracking when a transaction is occured.
}

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserWallet {
    pub name: String,
    pub address: String, // line 222 and 47 to be fixed before shifting to <Address>
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
        let phrase = Self::generate_phrase(private_key.clone());
        println!("Your secret phrase is: {:?}", phrase);
        let password_key = password.into_bytes();
        println!("password encrpypted as bytes ;) {:?}", password_key);
        //TODO: will need to add to the list of current users. this will be used for searching up
        //other users via their name or public address.
        return wallet;
    }

    fn generate_phrase(key: String) -> String {
        let key_bytes = hex::decode(key).expect("Err: Invalid Hex Input");
        let byte: [u8; 32] = key_bytes
            .try_into()
            .expect("Err: Unable to change format of bytes");
        let s =
            Mnemonic::from_entropy(&byte).expect("Err: Unable to convert bytes to Mnemonic type");
        let s_phrase = Mnemonic::to_string(&s);
        println!("phrase: {:?}", s_phrase);
        let ss = Mnemonic::parse(&s_phrase).expect("Err: Unable to convert back to Mnemonic");
        println!("ss here: {:?}", ss);
        let sss = Mnemonic::to_entropy(&ss);
        let private_key_again_1: Vec<u8> = sss
            .try_into()
            .expect("Err: Unable to convert back to vec form.");
        let private_key_again_2 = hex::encode(&private_key_again_1);
        println!("Private key again here: {:?}", private_key_again_2);
        return s_phrase;
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
    let wallet1 = UserWallet::generate_new_wallet("wallet1".to_string());
    let wallet2 = UserWallet::generate_new_wallet("wallet2".to_string());

    assert_ne!(wallet1, wallet2);
}
