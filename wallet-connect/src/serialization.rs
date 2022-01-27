//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use crate::hex;
use serde::de::{DeserializeOwned, Error as _};
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

/// Helpers for serializing strings and empty values
/// TODO: could this be replaced by some serde annotations or crate?
pub mod jsonstring {
    use super::*;

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let json = match value {
            None => Cow::from(""),
            Some(value) => serde_json::to_string(value)
                .map_err(S::Error::custom)?
                .into(),
        };
        serializer.serialize_str(&json)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let json = Cow::<'de, str>::deserialize(deserializer)?;
        if !json.is_empty() {
            let value = serde_json::from_str(&json).map_err(D::Error::custom)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

/// Helpers for serializing byte slices as hexadecimal strings
pub mod hexstring {
    use super::*;

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Cow::<'de, str>::deserialize(deserializer)?;
        let bytes = hex::decode(&*string).map_err(D::Error::custom)?;
        Ok(bytes)
    }
}
