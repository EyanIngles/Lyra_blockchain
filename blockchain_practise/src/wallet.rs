use hex;
use k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng};

#[derive(PartialEq, Debug)]
pub struct Currency {
    pub name: String,
    pub amount: u128,
}
#[derive(PartialEq, Debug)]
pub struct UserWallet {
    pub name: String,
    pub address: String,
    pub currency_accounts: Currency,
}

impl UserWallet {
    pub fn generate_new_wallet(name: String) -> Self {
        // TODO: add name for native currency to then use
        // for the currency section.
        let (private_key, public_key) = Self::generate_keys();
        let current_account = Currency {
            name: name.clone(),
            amount: 0,
        };
        let wallet = UserWallet {
            name: name.clone(),
            address: public_key.to_string(),
            currency_accounts: current_account,
        };
        println!("New Wallet Users: {:?}", wallet.name);
        println!("Private Key: {:?}", private_key);
        println!("Public Key: {:?}", public_key);
        wallet
    }

    fn generate_keys() -> (String, String) {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = private_key.verifying_key();

        let private_key_hex = hex::encode(private_key.to_bytes());
        let public_key_hex = hex::encode(public_key.to_encoded_point(false).as_bytes());

        (private_key_hex, public_key_hex)
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

