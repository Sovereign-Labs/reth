use crate::{GotExpectedBoxed, U256};

/// Represents error variants that can happen when trying to validate a
/// [Transaction](crate::Transaction)
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(thiserror_no_std::Error))]
pub enum InvalidTransactionError {
    /// The sender does not have enough funds to cover the transaction fees
    #[cfg_attr(feature = "std", error(
        "sender does not have enough funds ({}) to cover transaction fees: {}", _0.got, _0.expected
    ))]
    InsufficientFunds(GotExpectedBoxed<U256>),
    /// The nonce is lower than the account's nonce, or there is a nonce gap present.
    ///
    /// This is a consensus error.
    #[cfg_attr(feature = "std", error("transaction nonce is not consistent"))]
    NonceNotConsistent,
    /// The transaction is before Spurious Dragon and has a chain ID.
    #[cfg_attr(feature = "std", error("transactions before Spurious Dragon should not have a chain ID"))]
    OldLegacyChainId,
    /// The chain ID in the transaction does not match the current network configuration.
    #[cfg_attr(feature = "std", error("transaction's chain ID does not match"))]
    ChainIdMismatch,
    /// The transaction requires EIP-2930 which is not enabled currently.
    #[cfg_attr(feature = "std", error("EIP-2930 transactions are disabled"))]
    Eip2930Disabled,
    /// The transaction requires EIP-1559 which is not enabled currently.
    #[cfg_attr(feature = "std", error("EIP-1559 transactions are disabled"))]
    Eip1559Disabled,
    /// The transaction requires EIP-4844 which is not enabled currently.
    #[cfg_attr(feature = "std", error("EIP-4844 transactions are disabled"))]
    Eip4844Disabled,
    /// The transaction requires EIP-7702 which is not enabled currently.
    #[cfg_attr(feature = "std", error("EIP-7702 transactions are disabled"))]
    Eip7702Disabled,
    /// Thrown if a transaction is not supported in the current network configuration.
    #[cfg_attr(feature = "std", error("transaction type not supported"))]
    TxTypeNotSupported,
    /// The calculated gas of the transaction exceeds `u64::MAX`.
    #[cfg_attr(feature = "std", error("gas overflow (maximum of u64)"))]
    GasUintOverflow,
    /// The transaction is specified to use less gas than required to start the invocation.
    #[cfg_attr(feature = "std", error("intrinsic gas too low"))]
    GasTooLow,
    /// The transaction gas exceeds the limit
    #[cfg_attr(feature = "std", error("intrinsic gas too high"))]
    GasTooHigh,
    /// Thrown to ensure no one is able to specify a transaction with a tip higher than the total
    /// fee cap.
    #[cfg_attr(feature = "std", error("max priority fee per gas higher than max fee per gas"))]
    TipAboveFeeCap,
    /// Thrown post London if the transaction's fee is less than the base fee of the block.
    #[cfg_attr(feature = "std", error("max fee per gas less than block base fee"))]
    FeeCapTooLow,
    /// Thrown if the sender of a transaction is a contract.
    #[cfg_attr(feature = "std", error("transaction signer has bytecode set"))]
    SignerAccountHasBytecode,
}

/// Represents error variants that can happen when trying to convert a transaction to
/// [`PooledTransactionsElement`](crate::PooledTransactionsElement)
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(thiserror_no_std::Error))]
pub enum TransactionConversionError {
    /// This error variant is used when a transaction cannot be converted into a
    /// [`PooledTransactionsElement`](crate::PooledTransactionsElement) because it is not supported
    /// for P2P network.
    #[cfg_attr(feature = "std", error("Transaction is not supported for p2p"))]
    UnsupportedForP2P,
}

/// Represents error variants than can happen when trying to convert a
/// [`TransactionSignedEcRecovered`](crate::TransactionSignedEcRecovered) transaction.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(thiserror_no_std::Error))]
pub enum TryFromRecoveredTransactionError {
    /// Thrown if the transaction type is unsupported.
    #[cfg_attr(feature = "std", error("Unsupported transaction type: {0}"))]
    UnsupportedTransactionType(u8),
    /// This error variant is used when a blob sidecar is missing.
    #[cfg_attr(feature = "std", error("Blob sidecar missing for an EIP-4844 transaction"))]
    BlobSidecarMissing,
}
