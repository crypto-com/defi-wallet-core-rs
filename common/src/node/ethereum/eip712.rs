#![cfg(feature = "abi-contract")]

use crate::node::ethereum::abi::{EthAbiParamType, EthAbiToken};
use crate::transaction::{Eip712Error, EthError};
use ethers::prelude::abi::{ParamType, Token};
use ethers::prelude::U256;
use ethers::types::transaction::eip712;
use ethers::utils::keccak256;
use std::collections::HashMap;

mod deserializer;
use deserializer::Eip712TypedDataSerde;

type Eip712FieldName = String;
type Eip712FieldType = EthAbiParamType;
type Eip712FieldValue = EthAbiToken;
type Eip712StructName = String;
type Result<T> = std::result::Result<T, EthError>;

/// EIP-712 typed data
#[derive(Debug)]
pub struct Eip712TypedData {
    domain: eip712::EIP712Domain,
    primary_type: Eip712StructName,
    types: HashMap<Eip712StructName, Eip712Struct>,
    values: HashMap<Eip712FieldName, Eip712FieldValue>,
}

impl Eip712TypedData {
    /// Contruct an EIP-712 typed data from a JSON string of specified schema as below. The field
    /// `domain`, `message` (values), `primaryType` and `types` are all mandatory as described in
    /// [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
    ///
    /// {
    ///   "domain": {
    ///     "name": "Ether Mail",
    ///     "version": "1",
    ///     "chainId": 1,
    ///     "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
    ///   },
    ///   "message": {
    ///     "from": {
    ///       "name": "Cow",
    ///       "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
    ///     },
    ///     "to": {
    ///       "name": "Bob",
    ///       "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
    ///     },
    ///     "contents": "Hello, Bob!"
    ///   },
    ///   "primaryType": "Mail",
    ///   "types": {
    ///     "Mail": [
    ///       { "name": "from", "type": "Person" },
    ///       { "name": "to", "type": "Person" },
    ///       { "name": "contents", "type": "string" }
    ///     ],
    ///     "Person": [
    ///       { "name": "name", "type": "string" },
    ///       { "name": "wallet", "type": "address" }
    ///     ]
    ///   }
    /// }
    pub fn new(json_typed_data: &str) -> Result<Self> {
        let serde_typed_data: Eip712TypedDataSerde =
            serde_json::from_str(json_typed_data).map_err(Eip712Error::SerdeJsonError)?;
        serde_typed_data.try_into()
    }

    /// Encode the typed data.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let domain_separator = self.domain.separator();
        let struct_hash = self.build_struct_hash()?;
        let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();

        Ok(keccak256(digest_input).to_vec())
    }

    /// TODO
    fn build_struct_hash(&self) -> Result<[u8; 32]> {
        let mut items = vec![Token::Uint(self.build_type_hash())];

        let tokens = self
            .get_primary_struct()?
            .fields
            .iter()
            .map(|field| {
                let field_name = &field.name;
                self.values
                    .get(&field.name)
                    .map(Into::into)
                    .ok_or_else(|| Eip712Error::MissingFieldError(field_name.clone()).into())
            })
            .collect::<Result<Vec<Token>>>()?;

        for token in tokens {
            match &token {
                Token::Tuple(_) => {
                    // TODO
                    // TODO:
                    // Crate `ether-rs` uses `Token::Tuple` to save values of nested struct. Since
                    // we have already fixed to use `Eip712Struct`. Field of nested struct could be
                    // implemented in `Eip712Field`.
                    return Err(Eip712Error::EthersError(
                        eip712::Eip712Error::NestedEip712StructNotImplemented,
                    )
                    .into());
                }
                _ => {
                    items.push(eip712::encode_eip712_type(token));
                }
            }
        }

        Ok(keccak256(ethers::abi::encode(&items)))
    }

    /// TODO
    fn build_type_hash(&self) -> U256 {
        todo!()
        // let hash = keccak256(formatted_struct);
        // U256::from(&hash[..])
    }

    /// TODO
    fn get_primary_struct(&self) -> Result<&Eip712Struct> {
        self.types
            .get(&self.primary_type)
            .ok_or_else(|| Eip712Error::MissingTypeError(self.primary_type.clone()).into())
    }
}

/// EIP-712 struct type
#[derive(Debug, Eq, PartialEq)]
struct Eip712Struct {
    name: Eip712StructName,
    fields: Vec<Eip712Field>,
}

impl Eip712Struct {
    /// Contruct an EIP-712 struct type.
    fn new(name: Eip712StructName, fields: Vec<Eip712Field>) -> Self {
        Self { name, fields }
    }

    /// Build hash of this struct type.
    /// TODO
    fn encode_type(&self) -> String {
        let formatted_fields = self
            .fields
            .iter()
            .map(|f| format!("{} {}", f.r#type, f.name))
            .collect::<Vec<String>>()
            .join(",");

        format!("{}({})", self.name, formatted_fields)
    }

    /// Get the referenced struct names by fields of this struct.
    /// TODO
    fn get_referenced_struct_names(&self) -> Vec<Eip712StructName> {
        let mut struct_names = vec![];
        for f in self.fields {
            match f.r#type {
                Eip712FieldType::Struct(name) => struct_names.push(name),
                _ => (),
            }
        }
        struct_names
    }
}

/// EIP-712 field
#[derive(Debug, Eq, PartialEq)]
struct Eip712Field {
    name: String,
    r#type: Eip712FieldType,
}

#[cfg(test)]
mod eip712_encoding_tests {
    use super::*;

    const SIMPLE_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "name": "Ether Person",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "message": {
                "name": "Bob",
                "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
            },
            "primaryType": "Person",
            "types": {
                "EIP712Domain": [
                    { "name": "name", "type": "string" },
                    { "name": "version", "type": "string" },
                    { "name": "chainId", "type": "uint256" },
                    { "name": "verifyingContract", "type": "address" }
                ],
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    const RECURSIVELY_NESTED_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "name": "Ether Mail",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "message": {
                "from": {
                    "name": "Cow",
                    "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                },
                "to": {
                    "name": "Bob",
                    "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                },
                "contents": "Hello, Bob!"
            },
            "primaryType": "Mail",
            "types": {
                "EIP712Domain": [
                    { "name": "name", "type": "string" },
                    { "name": "version", "type": "string" },
                    { "name": "chainId", "type": "uint256" },
                    { "name": "verifyingContract", "type": "address" }
                ],
                "Mail": [
                    { "name": "from", "type": "Person" },
                    { "name": "to", "type": "Person" },
                    { "name": "contents", "type": "string" }
                ],
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    #[test]
    fn test_eip712_typed_data_simple_encoding() {
        let typed_data = Eip712TypedData::new(SIMPLE_JSON_TYPED_DATA).unwrap();
        let encoded_data = typed_data.encode().unwrap();

        assert_eq!(
            encoded_data,
            [
                182, 232, 94, 47, 97, 186, 229, 123, 119, 62, 140, 229, 52, 142, 10, 122, 161, 104,
                105, 146, 232, 140, 235, 153, 192, 138, 40, 7, 179, 114, 125, 174
            ]
        );
    }

    #[test]
    fn test_eip712_typed_data_recursively_nested_encoding() {
        let typed_data = Eip712TypedData::new(RECURSIVELY_NESTED_JSON_TYPED_DATA).unwrap();
        let encoded_data = typed_data.encode().unwrap();

        assert_eq!(encoded_data, [1, 2, 3],);
    }
}
