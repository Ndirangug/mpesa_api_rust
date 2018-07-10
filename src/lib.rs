//! A carte that provides bindings for the [Mpesa restful API](https://developer.safaricom.com)
//! 
//! # Usage
//! The crate is implemented in a way that's easy to use without much complexity
//! * Create a new instance of `AccessToken` and pass in your consumer key and consumer secret as `Strings`
//! * Retrieve your access token by calling `token()` on your instance of `AccessToken`
//! * Call the relevant function with your accesstoken

extern crate reqwest;
extern crate base64;
extern crate hyper;
extern crate serde_json;
extern crate actix_web;
extern crate actix;
extern crate tokio;
extern crate bytes;
extern crate futures;
pub mod access_token;
pub mod parameters;
pub mod api_products;