pub use serde_derive::Deserialize;
use crate::lyst746F::TokenStd;

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)] 
pub struct Token {
    id: String, //this will need to be a UID <to be build> - should also create a standard that must be followed.
    name: String,
    owner: String, //this will need to be changed to Address.
    std: TokenStd,
    total_supply: u64, // should total supply potentially be inside the <TokenStd>?
    balance: u64
}

#[derive(serde_derive::Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenList {
    pub tokens: Vec<Token>
}


impl Token {

    pub fn new() -> Token {
        let standard = TokenStd::new_token_std("Test_ID".to_string());
        let token = Token {
            id: "Practise_token".to_string(),
            owner: "Practise_owner".to_string(), //string for now until address is implemented.
            name: "Practise_name".to_string(),
            total_supply: 100000000, //this value is just a random value and is not finalised.
            std: standard,
            balance: 0,
        };
        return token;
    }
    
    fn token_standard(token: TokenStd) -> bool {    // Check if token being created is within standard or not which will pass back a boolean value.
           return TokenStd::is_following_token_std(token);

    }
    
    pub fn mint_tokens(token: &mut Token, mint_amount: u64, owner: String) {//owner is passing a string currently until addresses are correctly implemented.
        // need to check owner against the wallet doing the transaction.
        if TokenStd::get_has_totsup(&token.std) == false {
            token.total_supply = token.total_supply + mint_amount;
            // will require additional checks such as if this is the owner of the token and so on.
        } else {
            panic!("Err: Unable to mint due to Token is within required standard") //this wording
                                                                                   //needs to be
                                                                                   //updated.
        }
    }
}

impl TokenList {

    pub fn new() -> TokenList {
        let tklst = TokenList {
            tokens: vec![]
        };
        return tklst
    }

}



#[test]
pub fn test_new_token_is_standard_approved() { //This is a temporary test - more logic required
   let standard_approved = TokenStd::new_token_std("Test_ID".to_string());
   let new_token = Token::new();

   assert_eq!(new_token.std, standard_approved);
}

