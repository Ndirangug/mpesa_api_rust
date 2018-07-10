//!Representation of the B2C Api
//!
//! This API enables Business to Customer (B2C) transactions between a company and customers who are the end users of
//! its products or services. Use of this API requires a valid and verified B2C M-Pesa Short code.
//! 
//! testing url: POST https://sandbox.safaricom.co.ke/mpesa/b2c/v1/paymentrequest
 
/// A struct holding B2C request parameters 
#[derive(Debug)]
pub struct B2C {
    /// This is the credential/username used to authenticate the transaction request
    InitiatorName: String,
    /// Base64 encoded string of the Security Credential, which is encrypted using M-Pesa public key and validates the transaction on M-Pesa Core system.
    SecurityCredential: String,
    /// Unique command for each transaction type e.g. SalaryPayment, BusinessPayment, PromotionPayment
    CommandID: String,
    /// The amount being transacted
    Amount: String,
    /// Organization’s shortcode initiating the transaction.
    PartyA: String,
    /// Phone number receiving the transaction
    PartyB: String,
    /// Comments that are sent along with the transaction.
    Remarks: String,
    /// The timeout end-point that receives a timeout response
    QueueTimeOutURL: String,
    /// The end-point that receives the response of the transaction
    ResultURL: String,
    /// Optional
    Occasion: String,
}