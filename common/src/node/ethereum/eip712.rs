#![cfg(feature = "abi-contract")]

use crate::node::ethereum::abi::{EthAbiParamType, EthAbiToken};
use crate::transaction::{Eip712Error, EthError};
use ethers::prelude::{abi, H160, U256};
use ethers::types::transaction::eip712::{
    encode_eip712_type, EIP712_DOMAIN_TYPE_HASH, EIP712_DOMAIN_TYPE_HASH_WITH_SALT,
};
use ethers::utils::keccak256;
use std::collections::{BTreeSet, HashMap};

mod deserializer;
use deserializer::Eip712TypedDataSerde;

type Eip712FieldName = String;
type Eip712FieldType = EthAbiParamType;
type Eip712FieldValue = EthAbiToken;
type Eip712StructName = String;
type Result<T> = std::result::Result<T, EthError>;

/// EIP-712 typed data
#[derive(Debug, Default)]
pub struct Eip712TypedData {
    domain: Eip712Domain,
    primary_type: Eip712StructName,
    type_hashes: HashMap<Eip712StructName, U256>,
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
        // Deserialize from JSON typed data.
        let serde_typed_data: Eip712TypedDataSerde =
            serde_json::from_str(json_typed_data).map_err(Eip712Error::SerdeJsonError)?;
        let mut this = Self::try_from(serde_typed_data)?;

        // Build hashes of the all associating struct types when constructing. Since these type
        // hashes could be reused when encoding primary struct and other referenced sub-structs.
        this.build_all_type_hashes()?;

        Ok(this)
    }

    /// Encode the typed data.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let domain_separator = self.domain.separator();
        let struct_hash = self.build_struct_hash(&self.primary_type, &self.values)?;
        let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();

        Ok(keccak256(digest_input).to_vec())
    }

    /// Build hashes of the all associating struct types when constructing. Since these type hashes
    /// could be reused when encoding primary struct and other referenced sub-structs.
    fn build_all_type_hashes(&mut self) -> Result<()> {
        let encoded_types = self
            .types
            .iter()
            .map(|(struct_name, struct_type)| (struct_name.clone(), struct_type.encode_type()))
            .collect::<HashMap<_, _>>();

        for struct_name in self.types.keys() {
            // If the struct type references other struct types
            // (and these in turn reference even more struct types), then the set of referenced
            // struct types is collected, sorted by name and appended to the encoding. An example
            // encoding is
            // `Transaction(Person from,Person to,Asset tx)Asset(address token,uint256 amount)Person(address wallet,string name)`.

            // Get referenced sub-struct names.
            let mut ref_struct_names = BTreeSet::new();
            self.get_referenced_struct_names(struct_name, &mut ref_struct_names)?;

            // Initialize encoded data of this struct.
            let mut encoded_data = encoded_types
                .get(struct_name)
                .ok_or_else(|| Eip712Error::MissingTypeError(struct_name.clone()))?
                .clone();

            // Append encoded data of sub-structs in sequence.
            for name in ref_struct_names {
                encoded_data.push_str(
                    encoded_types
                        .get(&name)
                        .ok_or_else(|| Eip712Error::MissingTypeError(name.clone()))?,
                );
            }

            // Hash encoded data.
            let hash = keccak256(encoded_data);

            // Save typed hash of this struct.
            self.type_hashes
                .insert(struct_name.clone(), U256::from(&hash[..]));
        }

        Ok(())
    }

    /// Build hash of specified struct name and field values. This function could be reused to
    /// construct hash of any associating structs in this typed data.
    fn build_struct_hash(
        &self,
        struct_name: &str,
        values: &HashMap<Eip712FieldName, Eip712FieldValue>,
    ) -> Result<[u8; 32]> {
        let type_hash = self
            .type_hashes
            .get(struct_name)
            .ok_or_else(|| Eip712Error::MissingTypeError(struct_name.to_owned()))?;
        let mut items = vec![abi::Token::Uint(*type_hash)];
        for field in &self.get_struct(struct_name)?.fields {
            let field_name = &field.name;

            let field_value = values
                .get(field_name)
                .ok_or_else(|| Eip712Error::MissingFieldError(field_name.clone()))?;

            items.push(self.encode_field_value(field_value)?);
        }

        Ok(keccak256(ethers::abi::encode(&items)))
    }

    /// Encode a field value to an ABI token for further struct encoding. This function supports
    /// recursively invocation, since the type `Array` or `FixedArray` could has item type of a
    /// `Struct`.
    fn encode_field_value(&self, field_value: &Eip712FieldValue) -> Result<abi::Token> {
        match field_value {
            Eip712FieldValue::Array(items) | Eip712FieldValue::FixedArray(items) => {
                let tokens = &items
                    .iter()
                    .map(|i| self.encode_field_value(i))
                    .collect::<Result<Vec<_>>>()?;
                let hash = keccak256(abi::encode(tokens));
                Ok(abi::Token::Uint(U256::from(hash)))
            }
            Eip712FieldValue::Struct(sub_name, sub_values) => {
                let hash = self.build_struct_hash(sub_name, sub_values)?;
                Ok(abi::Token::Uint(U256::from(hash)))
            }
            Eip712FieldValue::Tuple(_) => Err(Eip712Error::UnsupportedError(
                "Tuple is unsupported by EIP-712".to_owned(),
            )
            .into()),
            _ => Ok(encode_eip712_type(field_value.try_into()?)),
        }
    }

    /// Recursively get referenced struct names by a parent struct name.
    /// Argument `parent_struct_name` is the struct name to get sub-structs.
    /// Argument `current_struct_names` is mutable. It is used to return the all referenced struct
    /// names recursively.
    fn get_referenced_struct_names(
        &self,
        parent_struct_name: &str,
        current_struct_names: &mut BTreeSet<Eip712StructName>,
    ) -> Result<()> {
        let sub_struct_names = self.get_struct(parent_struct_name)?.get_sub_struct_names();
        for name in sub_struct_names {
            if !current_struct_names.contains(&name) {
                self.get_referenced_struct_names(&name, current_struct_names)?;
            }
        }

        Ok(())
    }

    /// Get struct type by name.
    fn get_struct(&self, struct_name: &str) -> Result<&Eip712Struct> {
        self.types
            .get(struct_name)
            .ok_or_else(|| Eip712Error::MissingTypeError(struct_name.to_owned()).into())
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

    /// Encode this struct type (without referenced sub-structs).
    /// e.g. struct Transaction could be encoded to `Transaction(Person from,Person to,Asset tx)`.
    fn encode_type(&self) -> String {
        let formatted_fields = self
            .fields
            .iter()
            .map(|f| format!("{} {}", f.r#type, f.name))
            .collect::<Vec<String>>()
            .join(",");

        format!("{}({})", self.name, formatted_fields)
    }

    /// Get unique sub-struct names by fields of this struct.
    /// e.g. encoding `Transaction(Person from,Person to,Asset tx)` has referenced sub-struct
    /// `Asset` and `Person`.
    fn get_sub_struct_names(&self) -> BTreeSet<Eip712StructName> {
        let mut struct_names = BTreeSet::new();
        for f in &self.fields {
            if let Eip712FieldType::Struct(name) = &f.r#type {
                struct_names.insert(name.clone());
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

/// EIP-712 domain
#[derive(Debug, Default)]
struct Eip712Domain {
    name: Option<String>,
    version: Option<String>,
    chain_id: Option<U256>,
    verifying_contract: Option<H160>,
    salt: Option<[u8; 32]>,
}

impl Eip712Domain {
    pub fn separator(&self) -> [u8; 32] {
        let domain_type_hash = if self.salt.is_some() {
            EIP712_DOMAIN_TYPE_HASH_WITH_SALT
        } else {
            EIP712_DOMAIN_TYPE_HASH
        };
        let mut tokens = vec![abi::Token::Uint(U256::from(domain_type_hash))];
        if let Some(name) = &self.name {
            tokens.push(abi::Token::Uint(U256::from(keccak256(name))));
        }
        if let Some(version) = &self.version {
            tokens.push(abi::Token::Uint(U256::from(keccak256(version))));
        }
        if let Some(chain_id) = self.chain_id {
            tokens.push(abi::Token::Uint(chain_id));
        }
        if let Some(verifying_contract) = self.verifying_contract {
            tokens.push(abi::Token::Address(verifying_contract));
        }
        if let Some(salt) = &self.salt {
            tokens.push(abi::Token::Uint(U256::from(salt)));
        }

        keccak256(abi::encode(&tokens))
    }
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

        assert_eq!(
            encoded_data,
            [
                187, 37, 204, 25, 4, 87, 40, 67, 2, 128, 149, 138, 235, 206, 18, 177, 36, 205, 201,
                31, 129, 127, 207, 185, 49, 63, 192, 93, 120, 76, 65, 192
            ],
        );
    }
}
