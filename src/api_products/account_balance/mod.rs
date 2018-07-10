//! Represenation of the Account Balance Api
//! 
//! The Account Balance API requests for the account balance of a shortcode.
//! 
//! test url: POST https://sandbox.safaricom.co.ke/mpesa/accountbalance/v1/query

use super::super::parameters::*;


/// A strcut holding request parameters for account balance api
#[derive(Debug)]
pub struct AccountBalance {
    /// This is the credential/username used to authenticate the transaction request
    initiator: String,
    /// Base64 encoded string of the Security Credential, which is encrypted using M-Pesa public key and validates the transaction on M-Pesa Core system.
    security_credential: String,
    /// A unique command passed to the M-Pesa system
    command_id: CommandIds,
    /// The shortcode of the organisation receiving the transaction.
    party_b: String,
    /// Type of the organisation receiving the transaction.
    ReceiverIdentifierType: Identifiers,
    /// Comments that are sent along with the transaction.
    Remarks: String,
    /// The timeout end-point that receives a timeout message.
    QueueTimeOutURL: String,
    /// The end-point that receives a successful transaction.
    ResultURL: String,
    /// Organisation receiving the funds.
    AccountType: String,
}