use super::KeyType;
use super::EncodeType;


/// uniffi don't support Box<dyn std::error::Error> so convert a lower error to a string. discard the source
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseKeyError {
    #[error("key_type: {key_type} {msg}")]
    InvalidKeyBytes{key_type: KeyType, msg: String},
    #[error("invalid format, colon_number expected 2 but {colon_number}. the format is key_type:encode_type:xxxxx")]
    InvalidStringFormat{colon_number: u8},
    #[error("unknown key type '{unknown_key_type}'")]
    UnknownKeyType { unknown_key_type: String },
    #[error("unknown encode type '{unknown_encode_type}'")]
    UnknownEncodeType { unknown_encode_type: String },
    #[error("invalid encode format: key_type: {key_type}  encode_type: {encode_type} cause: {cause}")]
    InvalidEncodeFormat { key_type: KeyType, encode_type: EncodeType, cause: String },
    #[error("invalid key length: expected the input of {expected_length} bytes, but {received_length} was given")]
    InvalidLength { expected_length: usize, received_length: usize },
    #[error("invalid key data: {error_message}")]
    InvalidData { error_message: String },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseSignatureError {
    #[error("unknown key type '{unknown_key_type}'")]
    UnknownKeyType { unknown_key_type: String },
    #[error("invalid signature length: expected the input of {expected_length} bytes, but {received_length} was given")]
    InvalidLength { expected_length: usize, received_length: usize },
    #[error("invalid signature data: {error_message}")]
    InvalidData { error_message: String },
}
