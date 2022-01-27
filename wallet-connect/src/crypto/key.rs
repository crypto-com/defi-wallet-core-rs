//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use super::aead::{self, OpenError};
use crate::hex;
use crate::protocol::EncryptionPayload;
use ethers::utils::hex::FromHexError;
use rand::{rngs::OsRng, Rng};
use secrecy::{ExposeSecret, SecretString};
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::str::FromStr;
use zeroize::Zeroizing;

/// A wrapper around the symmetric key
#[derive(Clone, Eq, PartialEq)]
pub struct Key(Zeroizing<[u8; 32]>);

impl Key {
    /// generates a random key
    pub fn random() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill(&mut key);
        Key::from_raw(key)
    }

    /// converts a raw byte array to the wrapper
    pub fn from_raw(raw: [u8; 32]) -> Self {
        Key(raw.into())
    }

    /// hexadecimal representation of the key
    pub fn display(&self) -> SecretString {
        let keyref: &[u8] = &*self.0;
        SecretString::new(hex::encode(keyref))
    }

    /// gets a raw slice reference to the key
    pub fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// encrypt using the key
    pub fn seal(&self, data: impl AsRef<[u8]>) -> EncryptionPayload {
        aead::seal(self.as_ref(), data.as_ref())
    }

    /// decrypt using the key
    pub fn open(&self, payload: &EncryptionPayload) -> Result<Vec<u8>, OpenError> {
        aead::open(self.as_ref(), payload)
    }
}

impl FromStr for Key {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 32];
        hex::decode_mut(s, &mut bytes)?;
        Ok(Key::from_raw(bytes))
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Key(********)")
    }
}

impl Serialize for Key {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.display().expose_secret())
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Cow::<'de, str>::deserialize(deserializer)?;
        Key::from_str(&s).map_err(de::Error::custom)
    }
}
