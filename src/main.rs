extern crate mpesa;
use mpesa::access_token::AccessToken;
fn main() {
    println!("{}", AccessToken::new(String::from("0SKHgBn66azzyz5Y22ZufBhP6m5JwmQT"), String::from("GUm27XVgi697SUfE")).token().expect("unable to retrieve token at this time");
}