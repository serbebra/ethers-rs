#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]

mod base;
mod call;
mod contract;
mod error;
mod event;
mod factory;
mod log;

pub use base::{decode_function_data, encode_function_data, AbiError, BaseContract};
pub use call::{ContractError, EthCall};
pub use contract::Contract;
pub use error::EthError;
pub use event::{EthEvent, Event};
pub use factory::{ContractDeployer, ContractFactory};
pub use log::{decode_logs, EthLogDecode, LogMeta};

// `stream` module is empty in this snippet; assuming it is implemented elsewhere
pub mod stream;

// Conditional compilation for the `multicall` and related ABIGEN features
#[cfg(any(test, feature = "abigen"))]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
mod multicall;
#[cfg(any(test, feature = "abigen"))]
pub use multicall::{
    Multicall, MulticallContract, MulticallError, MulticallResult, MulticallVersion,
    MULTICALL_ADDRESS, MULTICALL_SUPPORTED_CHAIN_IDS,
};

// `builders` module is hidden from the docs as it's intended for internal use
#[doc(hidden)]
pub mod builders {
    pub use super::{
        call::ContractCall,
        event::Event,
        factory::{ContractDeployer, Deployer},
    };
}

// Re-exporting `abigen` related items only when `abigen` feature is enabled
#[cfg(feature = "abigen")]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
pub use ethers_contract_abigen::{
    Abigen, ContractFilter, ExcludeContracts, InternalStructs, MultiAbigen, SelectContracts,
};
#[cfg(feature = "abigen")]
pub use ethers_contract_derive::{
    abigen, EthAbiCodec, EthAbiType, EthCall, EthDisplay, EthError, EthEvent,
};

// Re-export `Lazy` but hidden from the documentation
#[doc(hidden)]
pub use once_cell::sync::Lazy;

// Conditionally compile and export `eip712` related features
#[cfg(feature = "eip712")]
pub use ethers_derive_eip712::*;
