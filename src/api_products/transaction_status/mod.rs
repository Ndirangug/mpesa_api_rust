//! Represenattion of the Transaction Status API
//! 
//! Transaction Status API checks the status of a B2B, B2C and C2B APIs transactions.
//! 
//! test url: POST https://sandbox.safaricom.co.ke/mpesa/transactionstatus/v1/query


/// struct holding Transaction status reqest parameters
#[derive(Debug)]
pub struct TransactionSatus {
    /// Unique command for each transaction type, possible values are:TransactionStatusQuery
    CommandID: String,
    /// Organization /MSISDN sending the transaction
    ShortCode: String,
    /// Type of organization receiving the transaction
    IdentifierType: String,
    /// Comments that are sent along with the transaction.
    Remarks: String,
    /// The name of Initiator to initiating the request
    Initiator: String,
    /// Base64 encoded string of the Security Credential, which is encrypted using M-Pesa public key and validates the transaction on M-Pesa Core system.
    SecurityCredential: String,
    /// The path that stores information of time out transaction.
    QueueTimeOutURL: String,
    /// The path that stores information of transaction.
    ResultURL: String,
    /// Organization Receiving the funds
    TransactionID: String,
    /// Optional
    Occasion: String,
}