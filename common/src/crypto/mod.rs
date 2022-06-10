mod errors;

use cosmrs::bip32::secp256k1::ecdsa as secp256k1;
use ed25519_dalek as ed25519;

use rand_core::RngCore;
use rand_core::OsRng;
use tendermint::signature::Signer;

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub use errors::ParseKeyError;


#[derive(Debug, Copy, Clone)]
pub enum KeyType {
    SECP256K1 = 0,
    ED25519 = 1,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}",
            match self {
                KeyType::SECP256K1 => "secp256k1",
                KeyType::ED25519 => "ed25519",
            },
        )
    }
}

impl FromStr for KeyType {
    type Err = errors::ParseKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lowercase_key_type = value.to_ascii_lowercase();
        match lowercase_key_type.as_str() {
            "secp256k1" => Ok(KeyType::SECP256K1),
            "ed25519" => Ok(KeyType::ED25519),
            _ => Err(Self::Err::UnknownKeyType { unknown_key_type: lowercase_key_type }),
        }
    }
}

impl TryFrom<u8> for KeyType {
    type Error = errors::ParseKeyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyType::SECP256K1),
            1 => Ok(KeyType::ED25519),
            unknown_key_type => {
                Err(Self::Error::UnknownKeyType { unknown_key_type: unknown_key_type.to_string() })
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EncodeType {
    Hex,
    Base58,
}

impl Display for EncodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}",
            match self {
                EncodeType::Hex => "hex",
                EncodeType::Base58 => "base58",
            },
        )
    }
}

impl FromStr for EncodeType {
    type Err = self::errors::ParseKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lowercase_key_type = value.to_ascii_lowercase();
        match lowercase_key_type.as_str() {
            "hex" => Ok(EncodeType::Hex),
            "base58" => Ok(EncodeType::Base58),
            _ => Err(Self::Err::UnknownEncodeType { unknown_encode_type: lowercase_key_type }),
        }
    }
}

// example: ed25519:base58:4Dn2jqzgvwxPyMwunTmszbs89igFb5bFPomkZMxmK6aSUnZkwDGUF9VapE6vp8PUf2DFxxxxxxxxxxxxxxxxxxxx
fn split_key_string(value: &str) -> Result<(KeyType, EncodeType, &str), errors::ParseKeyError> {
    let str_vec: Vec<_> = value.split(":").collect();
    if str_vec.len() != 3 {
        return Err(errors::ParseKeyError::InvalidStringFormat { colon_number:  str_vec.len() as u8});
    }
    Ok((KeyType::from_str(str_vec[0])?, EncodeType::from_str(str_vec[1])?, str_vec[2]))
}

/// uniffi don't support enum method 
pub struct PrivateKey {
    key_type: KeyType,
    key_data: [u8; 32]
}

impl PrivateKey {
    /// generates a random private key
    pub fn new(key_type: KeyType) -> Self {
        match key_type {
            KeyType::SECP256K1 => {
                let sk = secp256k1::SigningKey::random(&mut OsRng);
                Self{ key_type: key_type, key_data: sk.to_bytes().into() }
            }
            KeyType::ED25519 => {
                let mut array = [0u8; 32];
                OsRng.fill_bytes(&mut array);
                let sk = ed25519::SecretKey::from_bytes(&array).unwrap();
                Self{ key_type: key_type, key_data: sk.to_bytes() }
            }
        }
    }

    /// constructs secret key from bytes &[u8]
    pub fn from_bytes(key_type: KeyType, data: &Vec<u8>) -> Result<Self, errors::ParseKeyError> {
        match key_type {
            KeyType::SECP256K1 => {
                let sk = secp256k1::SigningKey::from_bytes(data).map_err(|e| {
                    errors::ParseKeyError::InvalidKeyBytes { key_type: key_type, msg: e.to_string()}
                })?;
                Ok(Self{ key_type: key_type, key_data: sk.to_bytes().into() })
            }
            KeyType::ED25519 => {
                let sk = ed25519::SecretKey::from_bytes(data).map_err(|e| {
                    errors::ParseKeyError::InvalidKeyBytes { key_type: key_type, msg: e.to_string()}
                })?;
                Ok(Self{ key_type: key_type, key_data: sk.to_bytes() })
            }
        }
    }

    /// constructs secret key from str, the format is "keytype:encodetype:xxxxxxxxx"
    pub fn from_str(string: &String) -> Result<Self, errors::ParseKeyError> {
        let (key_type, encode_type, key_data) = split_key_string(string)?;
        match key_type {
            KeyType::SECP256K1 => {
                match encode_type {
                    EncodeType::Hex => {
                        let bytes = hex::decode(key_data).map_err(|e|{
                            errors::ParseKeyError::InvalidEncodeFormat { key_type: key_type, encode_type: encode_type, cause: e.to_string()}
                        })?;
                        return Self::from_bytes(key_type, bytes.as_ref());
                    }
                    EncodeType::Base58 => {
                        let key_vec = bs58::decode(key_data).into_vec().map_err(|e| {
                            errors::ParseKeyError::InvalidEncodeFormat { key_type: key_type, encode_type: encode_type, cause: e.to_string()}
                        })?;
                        return Self::from_bytes(key_type, key_vec.as_ref());
                    }
                }
            }
            KeyType::ED25519 => {
                match encode_type {
                    EncodeType::Hex => {
                        let bytes = hex::decode(key_data).map_err(|e|{
                            errors::ParseKeyError::InvalidEncodeFormat { key_type: key_type, encode_type: encode_type, cause: e.to_string()}
                        })?;
                        return Self::from_bytes(key_type, bytes.as_ref());
                    }
                    EncodeType::Base58 => {
                        let key_vec = bs58::decode(key_data).into_vec().map_err(|e| {
                            errors::ParseKeyError::InvalidEncodeFormat { key_type: key_type, encode_type: encode_type, cause: e.to_string()}
                        })?;
                        return Self::from_bytes(key_type, key_vec.as_ref());
                    }
                }
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.key_data.to_vec()
    }

    /// format: key_type:encode_type:xxxx
    pub fn to_string(&self, encode_type: EncodeType) -> String {
        match encode_type {
            EncodeType::Hex => {
                let s = hex::encode(self.key_data);
                format!("{}:{}:{}", self.key_type, encode_type, s)
            }
            EncodeType::Base58 => {
                let s = bs58::encode(self.key_data).into_string();
                format!("{}:{}:{}", self.key_type, encode_type, s)
            }
        }
    }

    /// signs a data [`u8`]
    pub fn sign(&self, data: &[u8]) -> Result<Signature, String> {
        match self.key_type {
            KeyType::SECP256K1 => {
                let sk = self.unwrap_as_secp256k1();
                let sig: secp256k1::Signature = sk.sign(data);
                Ok(Signature{ key_type: self.key_type, sig_data: sig.to_vec()})
            }
            KeyType::ED25519 => {
                let sk = self.unwrap_as_ed25519();
                let pk: ed25519::PublicKey = (&sk).into();
                let key_pair = ed25519::Keypair{secret: sk, public: pk};
                let sig_array = key_pair.sign(data).to_bytes();
                Ok(Signature{ key_type: self.key_type, sig_data: sig_array.to_vec()})
            }
        }
    }

    /// gets public key to byte array
    pub fn to_public_key(&self) -> PublicKey {
        match self.key_type {
            KeyType::SECP256K1 => {
                let sk = self.unwrap_as_secp256k1();
                let pk = sk.verifying_key();
                let pk_bytes = &*pk.to_bytes();
                PublicKey::from_bytes(self.key_type, pk_bytes).unwrap()
            }
            KeyType::ED25519 => {
                let sk = self.unwrap_as_ed25519();
                let pk: ed25519::PublicKey = (&sk).into();
                PublicKey::from_bytes(self.key_type, pk.to_bytes().as_ref()).unwrap()
            }
        }
    }


    fn unwrap_as_secp256k1(&self) -> secp256k1::SigningKey {
        match self.key_type {
            KeyType::ED25519 => panic!(),
            KeyType::SECP256K1 => secp256k1::SigningKey::from_bytes(self.key_data.as_ref()).unwrap(),
        }
    }

    fn unwrap_as_ed25519(&self) -> ed25519::SecretKey {
        match self.key_type {
            KeyType::ED25519 => ed25519::SecretKey::from_bytes(self.key_data.as_ref()).unwrap(),
            KeyType::SECP256K1 => panic!(),
        }
    }

}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new(KeyType::SECP256K1)
    }
}

pub struct PublicKey {
    key_type: KeyType,
    key_data: Vec<u8>
}

impl PublicKey {

    pub fn from_bytes(key_type: KeyType, data: &[u8]) -> Result<Self, errors::ParseKeyError> {
        match key_type {
            KeyType::ED25519 => {
                let pk = ed25519::PublicKey::from_bytes(data.as_ref()).map_err(|e| {
                    errors::ParseKeyError::InvalidKeyBytes { key_type: key_type, msg: e.to_string()}
                })?;
                Ok(Self{ key_type: key_type, key_data: pk.to_bytes().to_vec()})
            }
            KeyType::SECP256K1 => {
                let pk = secp256k1::VerifyingKey::from_sec1_bytes(&data).map_err(|e| {
                    errors::ParseKeyError::InvalidKeyBytes { key_type: key_type, msg: e.to_string()}
                })?;
                let pk_bytes = &*pk.to_bytes();
                Ok(Self{ key_type: key_type, key_data: pk_bytes.to_vec()})
            }
        }
    }

    pub fn verify(&self) {

    }
}

pub struct Signature {
    key_type: KeyType,
    sig_data: Vec<u8> 
}



#[cfg(test)]
mod tests {

}