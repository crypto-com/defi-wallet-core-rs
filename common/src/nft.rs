// ! NFT module support

use crate::{impl_msg, proto, AccountId, ErrorReport, Msg, Result};

impl_msg!(
    // MsgIssueDenom defines an SDK message for creating a new denom.
    pub struct MsgIssueDenom {
        pub id: String,
        pub name: String,
        pub schema: String,
        pub sender: AccountId,
    }
);

impl_msg!(
    // MsgMintNft defines an SDK message for creating a new NFT.
    pub struct MsgMintNft {
        pub id: String,
        pub denom_id: String,
        pub name: String,
        pub uri: String,
        pub data: String,
        pub sender: AccountId,
        pub recipient: AccountId,
    }
);

impl_msg!(
    // MsgEditNft defines an SDK message for editing a nft.
    pub struct MsgEditNft {
        pub id: String,
        pub denom_id: String,
        pub name: String,
        pub uri: String,
        pub data: String,
        pub sender: AccountId,
    }
);

impl_msg!(
    // MsgTransferNft defines an SDK message for transferring an NFT to recipient.
    pub struct MsgTransferNft {
        pub id: String,
        pub denom_id: String,
        pub sender: AccountId,
        pub recipient: AccountId,
    }
);

impl_msg!(
    // MsgBurnNft defines an SDK message for burning a NFT.
    pub struct MsgBurnNft {
        pub id: String,
        pub denom_id: String,
        pub sender: AccountId,
    }
);
