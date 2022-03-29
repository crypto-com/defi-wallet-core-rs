#![cfg(feature = "uniffi-binding")]

use crate::{
    PublicKeyBytesError, PublicKeyBytesWrapper, UniffiCustomTypeConverter,
    COMPRESSED_SECP256K1_PUBKEY_SIZE,
};

impl UniffiCustomTypeConverter for PublicKeyBytesWrapper {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        if val.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
            Err(PublicKeyBytesError::InvalidLength.into())
        } else {
            Ok(PublicKeyBytesWrapper(val))
        }
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}
