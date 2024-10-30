use alloy::transports::{RpcError, TransportErrorKind};
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_crypto_bls::error::BlsError;
use eigen_services_blsaggregation::bls_agg::BlsAggregationServiceError;
use s3n_chainio::error::ChainIoError;
use s3n_config::error::ConfigError;
use jsonrpc_core::serde_json::Error;
use thiserror::Error;
/// Error returned by chainio
#[derive(Debug, Error)]
pub enum AggregatorError {
    /// Bls Aggregation Service Error
    #[error("Bls Aggregation Service Error : {0}")]
    BlsAggregationServiceError(#[from] BlsAggregationServiceError),
    /// Task Response not found
    #[error("Task Response not found")]
    TaskResponseNotFound,
    /// parse error
    #[error("Config parse error")]
    ParseError(#[from] ConfigError),
    /// Build avs registry chain reader
    #[error("Failed to build avs registry chain reader ")]
    BuildAvsRegistryChainReader(#[from] AvsRegistryError),

    /// build avswriter
    #[error("Failed to build avs wrtier in chain io ")]
    BuildAvsWriter(#[from] ChainIoError),

    /// Bls crate error
    #[error("Bls Crate Error SDK")]
    Bls(#[from] BlsError),

    /// alloy rpc error
    #[error("Alloy rpc error")]
    AlloyRpc(#[from] RpcError<TransportErrorKind>),

    /// serde json error
    #[error("Serde json error")]
    SerdeError(#[from] Error),

    /// IO error
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
