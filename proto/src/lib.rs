// Ignore this clippy issue for generated `prost` code.
#![allow(clippy::derive_partial_eq_without_eq)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![allow(clippy::derive_partial_eq_without_eq)] // FIXME: generate types with `Eq`

use cosmos_sdk_proto::cosmos;
pub use tendermint_proto as tendermint;

use cosmos_sdk_proto::traits::TypeUrl;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
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
            use serde::{Deserialize, Serialize};
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

impl TypeUrl for chainmain::nft::v1::MsgIssueDenom {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgIssueDenom";
}

impl TypeUrl for chainmain::nft::v1::MsgMintNft {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgMintNFT";
}

impl TypeUrl for chainmain::nft::v1::MsgEditNft {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgEditNFT";
}
impl TypeUrl for chainmain::nft::v1::MsgTransferNft {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgTransferNFT";
}

impl TypeUrl for chainmain::nft::v1::MsgBurnNft {
    const TYPE_URL: &'static str = "/chainmain.nft.v1.MsgBurnNFT";
}

/// The version (commit hash) of the LunaClassic Core used when generating this library.
pub const LUNA_CLASSIC_VERSION: &str = include_str!("prost/LUNA_CLASSIC_COMMIT");

/// luna_classic protobuf definitions.
pub mod luna_classic {
    /// wasm
    pub mod wasm {
        pub mod v1beta1 {
            include!("prost/terra.wasm.v1beta1.rs");
        }
    }
}

impl TypeUrl for luna_classic::wasm::v1beta1::MsgExecuteContract {
    const TYPE_URL: &'static str = "terra.wasm.v1beta1.MsgExecuteContract";
}
