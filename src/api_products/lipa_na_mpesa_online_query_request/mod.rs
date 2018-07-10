//! Represenattion of lipa na mpesa online query request api
//! 
//! test url: POST https://sandbox.safaricom.co.ke/mpesa/stkpushquery/v1/query


/// A struct containing requesr parametrs for the lipa na mpesa online query request api
#[derive(Debug)]
pub struct LipaNaMpesaOnlineQueryRequest {
    /// Business Short Code
    BusinessShortCode: String,
    /// Password
    Password: String,
    /// Timestamp
    Timestamp: String,
    /// Checkout RequestID
    CheckoutRequestID: String,
    
}

/// Represention of responses expected from lipa na mpesa online query request api call
#[derive(Debug)]
pub enum LipaNaMpesaOnlineQueryRequestResponse {
    /// Merchant Request ID
    MerchantRequestID,
    /// Check out Request ID
    CheckoutRequestID,
    /// Response Code
    ResponseCode,
    /// Result Desc
    ResultDesc,
    /// Response Description message
    ResponseDescription,
    /// Result Code
    ResultCode,
}