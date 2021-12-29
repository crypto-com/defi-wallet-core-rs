#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use tendermint_proto as tendermint;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/COSMOS_SDK_COMMIT");
pub const CHAIN_MAIN_VERSION: &str = include_str!("prost/CHAIN_MAIN_COMMIT");

/// chainmain protobuf definitions.
pub mod chainmain {
    /// Authentication of accounts and transactions.
    pub mod chainmain {
        pub mod v1 {
            include!("prost/chainmain.chainmain.v1.rs");
        }
    }

    /// Balances.
    pub mod nft {
        pub mod v1 {
            include!("prost/chainmain.nft.v1.rs");
        }
    }

    /// Crisis handling
    pub mod supply {
        pub mod v1 {
            include!("prost/chainmain.supply.v1.rs");
        }
    }
}

/// Cosmos protobuf definitions.
mod cosmos {

    /// Base functionality.
    pub mod base {

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }
    }
}
