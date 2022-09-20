#![cfg(feature = "abi-contract")]

use crate::node::ethereum::abi::{EthAbiParamType, EthAbiToken};
use crate::transaction::{Eip712Error, EthError};
use ethers::prelude::{abi, U256};
use ethers::types::transaction::eip712::encode_eip712_type;
use ethers::utils::keccak256;
use std::convert::TryInto;
use std::str::FromStr;
use std::{
    collections::{BTreeSet, HashMap},
    fmt,
};

mod deserializer;
use deserializer::Eip712TypedDataSerde;

type Eip712FieldName = String;

#[derive(Debug, Eq, PartialEq)]
pub struct Eip712FieldType(EthAbiParamType);

impl fmt::Display for Eip712FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Eip712FieldType {
    type Err = EthError;

    fn from_str(s: &str) -> Result<Self> {
        let param = EthAbiParamType::from_str(s)?;
        param.try_into()
    }
}

impl TryFrom<EthAbiParamType> for Eip712FieldType {
    type Error = EthError;

    fn try_from(value: EthAbiParamType) -> Result<Self> {
        if value.iter().any(|t| {
            matches!(
                t,
                EthAbiParamType::Tuple(_)
                    | EthAbiParamType::IntAlias
                    | EthAbiParamType::UintAlias
                    | EthAbiParamType::Function
            )
        }) {
            Err(EthError::Eip712Error(Eip712Error::UnsupportedError(
                "Unsupported ABI type".to_string(),
            )))
        } else {
            Ok(Eip712FieldType(value))
        }
    }
}

impl AsRef<EthAbiParamType> for Eip712FieldType {
    fn as_ref(&self) -> &EthAbiParamType {
        &self.0
    }
}

type Eip712FieldValue = EthAbiToken;
type Eip712StructName = String;
type Result<T> = std::result::Result<T, EthError>;

const EIP712_DOMAIN_TYPE_NAME: &str = "EIP712Domain";

/// EIP-712 typed data
#[derive(Debug, Default)]
pub struct Eip712TypedData {
    domain: HashMap<Eip712FieldName, Eip712FieldValue>,
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
    ///     "EIP712Domain": [
    ///       { "name": "name", "type": "string" },
    ///       { "name": "version", "type": "string" },
    ///       { "name": "chainId", "type": "uint256" },
    ///       { "name": "verifyingContract", "type": "address" }
    ///     ],
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
        let domain_separator = self.build_struct_hash(EIP712_DOMAIN_TYPE_NAME, &self.domain)?;
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
            if current_struct_names.insert(name.clone()) {
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
            match &f.r#type {
                Eip712FieldType(EthAbiParamType::Array(item_param_type)) => {
                    if let EthAbiParamType::Struct(name) = item_param_type.as_ref() {
                        struct_names.insert(name.clone());
                    }
                }
                Eip712FieldType(EthAbiParamType::FixedArray(item_param_type, _size)) => {
                    if let EthAbiParamType::Struct(name) = item_param_type.as_ref() {
                        struct_names.insert(name.clone());
                    }
                }
                Eip712FieldType(EthAbiParamType::Struct(name)) => {
                    struct_names.insert(name.clone());
                }
                _ => {}
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
    use ethers::utils::hex;

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
          "types": {
            "EIP712Domain": [
              { "name": "chainId", "type": "uint256" },
              { "name": "name", "type": "string" },
              { "name": "verifyingContract", "type": "address" },
              { "name": "version", "type": "string" }
            ],
            "Action": [
              { "name": "action", "type": "string" },
              { "name": "params", "type": "string" }
            ],
            "Cell": [
              { "name": "capacity", "type": "string" },
              { "name": "lock", "type": "string" },
              { "name": "type", "type": "string" },
              { "name": "data", "type": "string" },
              { "name": "extraData", "type": "string" }
            ],
            "Transaction": [
              { "name": "DAS_MESSAGE", "type": "string" },
              { "name": "inputsCapacity", "type": "string" },
              { "name": "outputsCapacity", "type": "string" },
              { "name": "fee", "type": "string" },
              { "name": "action", "type": "Action" },
              { "name": "inputs", "type": "Cell[]" },
              { "name": "outputs", "type": "Cell[]" },
              { "name": "digest", "type": "bytes32" }
            ]
          },
          "primaryType": "Transaction",
          "domain": {
            "chainId": 56,
            "name": "da.systems",
            "verifyingContract": "0x0000000000000000000000000000000020210722",
            "version": "1"
          },
          "message": {
            "DAS_MESSAGE": "SELL mobcion.bit FOR 100000 CKB",
            "inputsCapacity": "1216.9999 CKB",
            "outputsCapacity": "1216.9998 CKB",
            "fee": "0.0001 CKB",
            "digest": "0x53a6c0f19ec281604607f5d6817e442082ad1882bef0df64d84d3810dae561eb",
            "action": {
              "action": "start_account_sale",
              "params": "0x00"
            },
            "inputs": [
              {
                "capacity": "218 CKB",
                "lock": "das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...",
                "type": "account-cell-type,0x01,0x",
                "data": "{ account: mobcion.bit, expired_at: 1670913958 }",
                "extraData": "{ status: 0, records_hash: 0x55478d76900611eb079b22088081124ed6c8bae21a05dd1a0d197efcc7c114ce }"
              }
            ],
            "outputs": [
              {
                "capacity": "218 CKB",
                "lock": "das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...",
                "type": "account-cell-type,0x01,0x",
                "data": "{ account: mobcion.bit, expired_at: 1670913958 }",
                "extraData": "{ status: 1, records_hash: 0x55478d76900611eb079b22088081124ed6c8bae21a05dd1a0d197efcc7c114ce }"
              },
              {
                "capacity": "201 CKB",
                "lock": "das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...",
                "type": "account-sale-cell-type,0x01,0x",
                "data": "0x1209460ef3cb5f1c68ed2c43a3e020eec2d9de6e...",
                "extraData": ""
              }
            ]
          }
        }"#;

    #[test]
    fn test_eip712_typed_data_simple_encoding() {
        let typed_data = Eip712TypedData::new(SIMPLE_JSON_TYPED_DATA).unwrap();
        let encoded_data = typed_data.encode().unwrap();

        assert_eq!(
            hex::encode(encoded_data),
            "b6e85e2f61bae57b773e8ce5348e0a7aa1686992e88ceb99c08a2807b3727dae"
        );
    }

    #[test]
    fn test_eip712_typed_data_recursively_nested_encoding() {
        let typed_data = Eip712TypedData::new(RECURSIVELY_NESTED_JSON_TYPED_DATA).unwrap();
        let encoded_data = typed_data.encode().unwrap();

        assert_eq!(
            hex::encode(encoded_data),
            "42b1aca82bb6900ff75e90a136de550a58f1a220a071704088eabd5e6ce20446"
        );
    }
}
