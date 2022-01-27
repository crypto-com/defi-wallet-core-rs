//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use ethers::utils::{hex, hex::FromHexError};

/// encode the data as a hexadecimal string (lowercase)
pub fn encode(data: impl AsRef<[u8]>) -> String {
    hex::encode(data.as_ref())
}

/// decode a hexadecimal string into a provided buffer
pub fn decode_mut(
    bytes: impl AsRef<[u8]>,
    mut buffer: impl AsMut<[u8]>,
) -> Result<(), FromHexError> {
    hex::decode_to_slice(bytes.as_ref(), buffer.as_mut())
}

/// decode a hexadecimal string into a new byte vector
pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>, FromHexError> {
    hex::decode(bytes.as_ref())
}
