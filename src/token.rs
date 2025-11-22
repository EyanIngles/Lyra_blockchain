use crate::lyst746F::TokenStd;


pub struct Token {
    id: String, //this will need to be a UID <to be build> - should also create a standard that
                //must be followed.
    name: String,
    std: TokenStd,
    
}

impl Token {

    pub fn new_token() -> Token {
        let standard = TokenStd::new_token_std("Test_ID".to_string());
        let token = Token {
            id: "Practise_token".to_string(),
            name: "Practise_name".to_string(),
            std: standard
        };
        return token;
    }
    
    fn token_standard(token: TokenStd) -> bool {    // Check if token being created is within standard or not which will pass back a boolean value.
           return TokenStd::is_following_token_std(token);

    }




}


#[test]
pub fn test_new_token_is_standard_approved() { //This is a temporary test - more logic required
   let standard_approved = TokenStd::new_token_std("Test_ID".to_string());
   let new_token = Token::new_token();

   assert_eq!(new_token.std, standard_approved);
}

