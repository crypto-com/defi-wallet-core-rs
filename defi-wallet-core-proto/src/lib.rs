#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use tendermint_proto as tendermint;

use cosmrs::tx::MsgProto;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/COSMOS_SDK_COMMIT");
pub const CHAIN_MAIN_VERSION: &str = include_str!("prost/CHAIN_MAIN_COMMIT");

/// chainmain protobuf definitions.
pub mod chainmain {
    /// chainmain
    #[allow(clippy::module_inception)]
    pub mod chainmain {
        pub mod v1 {
            include!("prost/chainmain.chainmain.v1.rs");
        }
    }

    /// nft
    pub mod nft {
        pub mod v1 {
            include!("prost/chainmain.nft.v1.rs");
        }
    }

    /// supply
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

impl MsgProto for chainmain::nft::v1::MsgIssueDenom {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgIssueDenom";
}
