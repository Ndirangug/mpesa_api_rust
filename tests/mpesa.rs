extern crate mpesa;
use mpesa::access_token::*;
use mpesa::parameters::*;

#[test]
fn test_access_token() {
    let consumer_key = String::from("0SKHgBn66azzyz5Y22ZufBhP6m5JwmQT");
    let consumer_secret = String::from("GUm27XVgi697SUfE");
    let mut access_token = AccessToken::new(consumer_key, consumer_secret);
    

    match access_token.token() {
        Ok(token) => println!("Here is the token {}", token ),
        Err(err) => eprintln!("Couldnot get token\n {}", err),
    }
    
}

#[test]
fn test_enum_conversion(){
    let string1 = CommandIds::TransactionReversal;

    
}