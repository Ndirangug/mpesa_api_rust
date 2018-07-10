//! A module to make sure the Mpesa API access token is retreived afresh before it expires
//! 
//! This module is responsible for making sure that the access token
//! required to access the mpesa APi endpoints is always valid. 
//! The acess token expires after exactly 3600sec(1hr)
//! This module however will always retrive a new access token every 55min(for convinience) and cache it so subsequent 
//! calls that require the token don't always have to retrieving one from the API server.
//! 
//! So all you have to do is make sure you create a new instance of `AccessToken` as early in your program as possible 
//! and make sure it lives throughout your program so that a new a token doesnt have to be retrieved every time its needed.
//! 
//!  # Example
//! ```
//!     # use mpesa::access_token::AccessToken;
//!     
//!    let consumer_key = String::from("foo");
//!    let consumer_secret = String::from("bar");
//!    let mut access_token = AccessToken::new(consumer_key, consumer_secret);
//!     
//!     
//!    match access_token.token() {
//!        Ok(token) => {
//!                         println!("Here is the token {}", token );
//!                     },
//!        Err(err) => {
//!                     eprintln!("Couldnot get token{}", err);     
//!                     }
//!     }
//! ```

use reqwest;
use base64;
use hyper::header::{Basic, Authorization, Headers};
use actix_web::{client, HttpMessage, HttpRequest, HttpResponse, FutureResponse, AsyncResponder};
use bytes::Bytes;
use futures::future::Future;
use std::sync::mpsc;
use actix;
use futures;

use std::time::{Instant, Duration};
use std::fmt::{self, Display};
use std::convert::From;
use std::str::FromStr;
use serde_json;
use std::error::Error;

/// Constant representing time after which to get a new access token.
const ACCEESS_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 55);




#[derive(Debug)]
/// A struct that stores the token itself and an `Instant` of the last rerievel time.
/// Fields are private  to ensure data integrity. The token can be gotten via the public method `token()`
pub struct AccessToken {
   token: Option<String>,
   last_retrieved: Option<Instant>,
   credentials: String,
} 

/// Definition of possible errors when dealing with the access token.
/// Meant to be used as a return value in functions
#[derive(Debug)]
pub enum MpesaAccessTokenError {
    RetrievalConnectionError(reqwest::Error),
    RetrievalInvalidResponseError(String),
    RetrievalTimeoutError,
    InvalidAccessToken(String),
    EmptyAccessToken
}

impl AccessToken {
    /// Creates a new empty instance of AccessToken.
    /// the `token()` method can later be used to get the access token string
    /// 
    /// # Example
    /// ```
    /// # use mpesa::access_token::AccessToken;
    /// 
    /// // declared as mutable because its fields will be expected to change
    /// let consumer_key = String::from("foo");
    /// let consumer_secret = String::from("bar");
    /// let mut access_token = AccessToken::new(consumer_key, consumer_secret);
    /// 
    /// 
    /// ```
   pub fn new(consumer_key: String, consumer_secret: String) -> AccessToken{
       let credentials_str = format!("{}:{}", consumer_key, consumer_secret);
        AccessToken {
            token: None,
            last_retrieved: None,
            credentials: base64::encode(&credentials_str)
            
        } 
    }

    /// Public function to get the access token
    /// 
    /// The function itself implements a solution to check whether a valid token exists and whether it is valid
    /// so you dont have to worry about doing that yourself
    /// 
    /// # Example
    /// ```
    /// # use mpesa::access_token::AccessToken;
    /// 
    /// let consumer_key = String::from("foo");
    /// let consumer_secret = String::from("bar");
    /// let mut access_token = AccessToken::new(consumer_key, consumer_secret);
    /// 
    /// 
    /// match access_token.token() {
    ///    Ok(token) => {
    ///                     println!("Here is the token {}", token );
    ///                     
    ///                 },
    ///    Err(err) => {
    ///                 eprintln!("Couldnot get token{}", err);
    ///                 
    ///                 },
    ///     }
    /// 
    /// ```
    pub fn token(&mut self) -> Result<String, MpesaAccessTokenError>{
        let mut token_to_return = String::new();
        //check if there is an access token available locally
        if let Some(token) = self.token.clone() {
            //there is a token
            //now check if its expired
            let last_retrieved = self.last_retrieved.unwrap();

            if Instant::now().duration_since(last_retrieved) < ACCEESS_TOKEN_EXPIRATION {
                //not expired..go ahead and return the token
                token_to_return = format!("{}", token);
            }else {
               //expired...retrieve a new token and return the string value
                token_to_return = format!("{}", &self.get_token()?);
               

            }

        }else {
           //no token..retrieve a new one
           token_to_return =format!("{}", &self.get_token()?);
        }

        Ok(token_to_return)
    }
   
    /// A private function called to retieve a fresh access token from the Mpesa API server
     fn get_token(&mut self) -> Result<String, MpesaAccessTokenError>{
        let url = String::from("http://localhost/accessToken");
        let mut token_json = String::new();
        let mut response = reqwest::Client::new().get(&url)
                        .header(Authorization(Basic::from_str(&self.credentials).unwrap()))
                        .send()?;
        
        if response.status().is_success() {
            token_json = response.text()?;
        } else {
            return Err(MpesaAccessTokenError::RetrievalInvalidResponseError(format!("{}", response.status())));
        }                
                                    
        let token_json: serde_json::Value = serde_json::from_str(&token_json)?;

        self.token = Some(format!("{}", &token_json["access_token"].as_str().unwrap()));
        self.last_retrieved = Some(Instant::now()); 

        Ok(format!("{}", token_json["access_token"].as_str().unwrap()))
       
    }

    
}

// Allows a `reqwest::Error` to be converted to an `MpesaAccessTokenError`
impl From<reqwest::Error> for MpesaAccessTokenError {
    
    fn from(error: reqwest::Error) -> Self {
        MpesaAccessTokenError::RetrievalConnectionError(error)
    }
}

impl From<serde_json::Error> for MpesaAccessTokenError {
     fn from(error: serde_json::Error) -> Self {
        MpesaAccessTokenError::InvalidAccessToken(format!("{}", error))
    }
}

impl Display for MpesaAccessTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MpesaAccessTokenError::RetrievalConnectionError(ref reqwest_error) => write!(f, "MpesaAccessTokenError::RetrievalConnectionError -- {}", reqwest_error),
            &MpesaAccessTokenError::RetrievalInvalidResponseError(ref status_code) => write!(f, "MpesaAccessTokenError::RetrievalInvalidResponseError -- Status Code: {}", status_code),
            &MpesaAccessTokenError::RetrievalTimeoutError => write!(f, "MpesaAccessTokenError::RetrievalTimeoutError -- No valid response could be gotten after the maximum number of tries"),
            &MpesaAccessTokenError::InvalidAccessToken(ref description) => write!(f, "MpesaAccessTokenError::InvalidAccessToken -- {} ", description),
            &MpesaAccessTokenError::EmptyAccessToken => write!(f, "MpesaAccessTokenError::EmptyAccessToken -- the `token` filed is empty"),
        }
        
    }
}

impl Error for MpesaAccessTokenError {
    fn description(&self) -> &str{
        stringify!("{}", self)
    }
}


 
