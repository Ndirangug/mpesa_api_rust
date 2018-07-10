//! Represenattion of the revsal api
//! 
//! Reverses a B2B, B2C or C2B M-Pesa transaction
//! 
//! test url: POST https://sandbox.safaricom.co.ke/mpesa/reversal/v1/request


/// A struct holding request parameters for the Reversal Api
#[derive(Debug)]
pub struct Reversal {
    /// This is the credential/username used to authenticate the transaction request.
    Initiator: String,
    /// Base64 encoded string of the Security Credential, which is encrypted using M-Pesa public key and validates the transaction on M-Pesa Core system.
    SecurityCredential: String,
    /// Unique command for each transaction type, possible values are: TransactionReversal
    CommandID: String,
    /// Organization/MSISDN sending the transaction
    PartyA: String,
    /// Type of organization receiving the transaction
    RecieverIdentifierType: String,
    /// Comments that are sent along with the transaction.
    Remarks: String,
    /// The path that stores information of time out transaction
    QueueTimeOutURL: String,
    /// The path that stores information of transaction.
    ResultURL: String,
    /// Organization Receiving the funds
    TransactionID: String,
    /// Optional.
    Occasion: String,
}