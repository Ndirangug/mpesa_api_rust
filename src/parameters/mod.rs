//! A module that conatins represenations of various parameters for use in invoking mpesa API products
//! and handling responses received.
//! The enum variants should always be used with the `to_String()` method or `format!()` macro so as to get the correct value as defined in the [official Mpesa Api Documenation](https://developer.safaricom.com)

use std::fmt::{Display, self};


/// Reprersenattion of the `Command Ids` used to identify the various API Products to be invoked
/// To be usd as parameters to functions
/// 
/// The enum implememts the `Display` trait so you can use the `toString()` method on its variants or use the `format!() macro`
/// # Example
/// ```
/// # use mpesa::parameters::*;
/// 
/// let command = CommandIds::TransactionReversal.to_string();
/// 
/// assert_eq!(String::from("TransactionReversal"), command);
/// ```
#[derive(Debug)]
pub enum CommandIds {
    /// Reversal for an erroneous C2B transaction
    TransactionReversal,
    /// Used to send money from an employer to employees e.g. salaries
    SalaryPayment,
    /// Used to send money from business to customer e.g. refunds
    BusinessPayment,
    /// Used to send money when promotions take place e.g. raffle winners
    PromotionPayment,
    /// Used to check the balance in a paybill/buy goods account (includes utility, MMF, Merchant, Charges paid account)
    AccountBalance,
    /// Used to simulate a transaction taking place in the case of C2B Simulate Transaction or to initiate a transaction on behalf of the customer (STK Push).
    CustomerPayBillOnline,
    /// Used to query the details of a transaction
    TransactionStatusQuery,
    /// Similar to STK push, uses M-Pesa PIN as a service
    CheckIdentity,
    /// Sending funds from one paybill to another paybill
    BusinessPayBill,
    /// sending funds from buy goods to another buy goods
    BusinessBuyGoods,
    /// Transfer of funds from utility to MMF account
    DisburseFundsToBusiness,
    /// Transferring funds from one paybills MMF to another paybills MMF account
    BusinessToBusinessTransfer,
    /// Transferring funds from paybills MMF to another paybills utility account.
    BusinessTransferFromMMFToUtility
}

/// Identifier types - both sender and receiver - identify an M-Pesa transaction’s sending and receiving party as either
/// a shortcode, a till number or a MSISDN (phone number). There are three identifier types that can be used with MPesa APIs.
/// 
/// The enum implememts the `Display` trait so you can use the `toString()` method on its variants or use the `format!() macro`
/// # Example
/// ```
/// # use mpesa::parameters::*;
/// 
/// let identifier = Identifiers::ShortCode.to_string();
/// 
/// assert_eq!(String::from("4"), identifier);
/// ```
#[derive(Debug)]
#[derive(Clone)]
pub enum Identifiers {
    MSISDN = 1,
    TillNumber,
    ShortCode = 4,
}

/// Representation of reponses expected from the API after a call.
/// 
/// The enum implememts the `Display` trait so you can use the `toString()` method on its variants or use the `format!() macro`
/// # Example
/// ```
/// # use mpesa::parameters::*;
/// 
/// let result_code = ResultCodes::DuplicateDetected.to_string();
/// 
/// assert_eq!(String::from("15"), result_code);
/// ```
#[derive(Debug)]
#[derive(Clone)]
pub enum ResultCodes {
    Success,
    InsufficientFunds,
    LessThanMinimumTransactionValue,
    MoreThanMaximumTransactionValue,
    WouldExceedDailyTransferLimit,
    WouldExceedMinimumBalance,
    UnresolvedPrimaryParty,
    UnresolvedReceiverParty,
    WouldExceedMaximumBalance,
    DebitAccountInvalid = 11,
    CreditAccountInvalid,
    UnresolvedDebitAccount,
    UnresolvedCreditAccount,
    DuplicateDetected,
    InternalFailure = 17,
    UnresolvedInitiator = 20,
    TrafficBlockingConditionInPlace = 26

}

/// Response codes are sent from the clients endpoints back to the gateway. This is done to acknowledge that the
/// client has received the results
/// 
/// The enum implememts the `Display` trait so you can use the `toString()` method on its variants or use the `format!() macro`
/// # Example
/// ```
/// # use mpesa::parameters::*;
/// 
/// let response_code = ResponseCodes::SuccessNotC2B.to_string();
/// 
/// assert_eq!(String::from("00000000"), response_code);
/// ```
#[derive(Debug)]
pub enum ResponseCodes {
    SuccessC2B, // 0
    SuccessNotC2B, // 00000000
    RejectTranscation // 1
}

/// Representation of errors that may occur 
/// 
/// Safaricom APIs are built to comply with HTTP Status codes. The following are error codes that will be returned
/// whenever there are errors in a request. Server errors are rare but do occur whenever there are connectivity issues.
#[derive(Debug)]
pub enum MpesaRequestError {
    /// 400
    BadRequest,
    /// 401
    Unauthorized,
    /// 403
    Forbidden,
    /// 404
    NotFound,
    /// 405
    MethodNotAllowed,
    /// 406 -  You requested a format that isn’t json
    NotAcceptable,
    /// 429 - You’re requesting too many kittens! Slow down!
    TooManyRequests,
    /// 500 - We had a problem with our server. Try again later.
    InternalServerError,
    /// 503 -  We’re temporarily offline for maintenance. Please try again later.
    ServiceUnavailable,
    
}

impl Display for MpesaRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
       match self {
        &MpesaRequestError::BadRequest => write!(f, "Error 400: BadRequest"),
        &MpesaRequestError::Unauthorized => write!(f, "Error 401: Unauthorized"),
        &MpesaRequestError::Forbidden => write!(f, "Error 403: Forbidden"),
        &MpesaRequestError::NotFound => write!(f, "Error 404: Not Found"),
        &MpesaRequestError::MethodNotAllowed => write!(f, "Error 405: MethodNotAllowed "),
        &MpesaRequestError::NotAcceptable => write!(f, "Error 406: NotAcceptable, You requested a format that isn’t json"),
        &MpesaRequestError::TooManyRequests => write!(f, "Error 429: You’re requesting too many kittens! Slow down!"),
        &MpesaRequestError::InternalServerError => write!(f, "Error 500: InternalServerError, We had a problem with our server. Try again later. "),
        &MpesaRequestError::ServiceUnavailable => write!(f, "Error 503: ServiceUnavailable, We’re temporarily offline for maintenance. Please try again later"),
       
        
        }
    }
}

impl Display for ResponseCodes {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
       match self {
        &ResponseCodes::SuccessC2B => write!(f, "0"),
        &ResponseCodes::SuccessNotC2B => write!(f, "00000000"),
        &ResponseCodes::RejectTranscation => write!(f, "1"),
        
        }
    }
}

impl Display for ResultCodes {

   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let val = (*self).clone();
        write!(f, "{}", val as usize )
    }
}

impl Display for Identifiers {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = (*self).clone();
        write!(f, "{}", val as usize )
    }
}

impl Display for CommandIds {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            &CommandIds::TransactionReversal => write!(f, "TransactionReversal"),
            &CommandIds::TransactionStatusQuery => write!(f, "TransactionStatusQuery"),
            &CommandIds::AccountBalance => write!(f, "AccountBalance"),
            &CommandIds::BusinessBuyGoods => write!(f, "BusinessBuyGoods"),
            &CommandIds::BusinessPayBill => write!(f, "BusinessPayBill"),
            &CommandIds::BusinessPayment => write!(f, "BusinessPayment"),
            &CommandIds::BusinessToBusinessTransfer => write!(f, "BusinessToBusinessTransfer"),
            &CommandIds::BusinessTransferFromMMFToUtility => write!(f, "BusinessTransferFromMMFToUtility"),
            &CommandIds::CheckIdentity => write!(f, "CheckIdentity"),
            &CommandIds::CustomerPayBillOnline => write!(f, "CustomerPayBillOnline"),
            &CommandIds::DisburseFundsToBusiness => write!(f, "DisburseFundsToBusiness"),
            &CommandIds::PromotionPayment => write!(f, "PromotionPayment"),
            &CommandIds::SalaryPayment => write!(f, "SalaryPayment"),
            
        }
    }
}