
use crate::lyst746F::TokenStd;

pub struct Token {
    id: String, //this will need to be a UID <to be build> - should also create a standard that
                //must be followed.
    name: String,
    owner: String,
    std: TokenStd,
    max_supply: u64, 
    
}

impl Token {

    pub fn new_token() -> Token {
        let standard = TokenStd::new_token_std("Test_ID".to_string());
        let token = Token {
            id: "Practise_token".to_string(),
            owner: "Practise_owner".to_string(), //string for now until address is implemented.
            name: "Practise_name".to_string(),
            max_supply: 100000000, //this value is just a random value and is not finalised.
            std: standard,
        };
        return token;
    }
    
    fn token_standard(token: TokenStd) -> bool {    // Check if token being created is within standard or not which will pass back a boolean value.
           return TokenStd::is_following_token_std(token);

    }
    
    pub fn mint_tokens(token: &mut Token, mint_amount: u64, owner: String) {//owner is passing a string currently until addresses are correctly implemented.
        // need to check owner against the wallet doing the transaction.
        if TokenStd::get_has_maxsup(&token.std) == false {
            token.max_supply = token.max_supply + mint_amount;
            // will require additional checks such as if this is the owner of the token and so on.
        } else {
            panic!("Err: Unable to mint due to Token is within required standard")
        }
    }
}


#[test]
pub fn test_new_token_is_standard_approved() { //This is a temporary test - more logic required
   let standard_approved = TokenStd::new_token_std("Test_ID".to_string());
   let new_token = Token::new_token();

   assert_eq!(new_token.std, standard_approved);
}

