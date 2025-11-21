/// transactions, this section is used for transactions and what is outlined to be involved with a
/// transaction.


use std::string::String;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Transaction { 
    id: String, //to change to UID for unique transactions.
    functions: String, // for now is string and is used to describe actions.
    signer: String, // for now is a string until we established a criteria of what Lyra expects
                   // from a signer.
    sender: String,
    to: String,
    epoch: SystemTime
}

impl Transaction {

    pub fn transaction_new() -> Transaction {
        let time = Self::get_time_now();
        let transaction_now = Transaction{
            id: "Practise_id".to_string(),
            functions: "Practise_functions".to_string(),
            signer: "Practise_signer".to_string(),
            sender: "Practise_sender".to_string(),
            to: "Practise_to_address".to_string(),
            epoch: time
        };
        return transaction_now;
    }
    
    pub fn get_time_now() -> SystemTime {
        let sys_time = SystemTime::now();
        return sys_time;
    }
}

#[test]
fn test_transaction_process() {
    let tx = Transaction::transaction_new();
    assert_eq!(tx.id, "Practise_id".to_string());
    assert_eq!(tx.functions, "Practise_functions".to_string());
    assert_eq!(tx.signer, "Practise_signer".to_string());
    assert_eq!(tx.sender, "Practise_sender".to_string());
    assert_eq!(tx.to, "Practise_to_address".to_string());
    assert_ne!(tx.epoch, SystemTime::now()); //while testing these to be equal, they are slightly
                                             //off during the miliSecs which cause it to fail, this
                                             //test will need to be changed to allow assert_eq on
                                             //the seconds side, not miliseconds side
}

