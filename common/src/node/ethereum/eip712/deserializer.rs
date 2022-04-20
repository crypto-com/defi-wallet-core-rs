use crate::node::ethereum::eip712::{
    Eip712Domain, Eip712Field, Eip712FieldName, Eip712FieldType, Eip712FieldValue, Eip712Struct,
    Eip712StructName, Eip712TypedData, Result,
};
use crate::transaction::{Eip712Error, EthError};
use crate::utils::{deserialize_option_u64_from_any, hex_decode};
use ethers::prelude::{H160, U256};
use ethers::utils::keccak256;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

/// EIP-712 domain for deserializing
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Eip712DomainSerde {
    name: Option<String>,
    version: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_u64_from_any")]
    chain_id: Option<u64>,
    verifying_contract: Option<H160>,
    salt: Option<String>,
}

impl From<Eip712DomainSerde> for Eip712Domain {
    fn from(domain: Eip712DomainSerde) -> Self {
        Self {
            name: domain.name,
            version: domain.version,
            chain_id: domain.chain_id.map(Into::into),
            verifying_contract: domain.verifying_contract,
            salt: domain.salt.map(keccak256),
        }
    }
}

/// EIP-712 field for deserializing
#[derive(Deserialize)]
struct Eip712FieldSerde {
    name: String,
    r#type: String,
}

impl TryFrom<&Eip712FieldSerde> for Eip712Field {
    type Error = EthError;

    fn try_from(serde_field: &Eip712FieldSerde) -> Result<Self> {
        Ok(Self {
            name: serde_field.name.clone(),
            r#type: serde_field.r#type.as_str().into(),
        })
    }
}

/// EIP-712 typed data for deserializing
/// The item types of `message` is `serde_json::Value` and specified in `types`, and cannot be
/// figured out and parsed to `EthAbiToken` during deserializing.
/// Both struct types and values need to be parsed to EthAbiParamType and EthAbiToken when
/// converting Eip712TypedDataSerde to Eip712TypedData.
/// `Eip712TypedDataSerde` is used for JSON deserializing automatically. And `Eip712TypedData`
/// organizes Struct types as `EthAbiParamType` and values as `EthAbiToken` for further data
/// encoding.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Eip712TypedDataSerde {
    domain: Eip712DomainSerde,
    message: HashMap<Eip712FieldName, serde_json::Value>,
    primary_type: Eip712StructName,
    types: HashMap<Eip712StructName, Vec<Eip712FieldSerde>>,
}

impl TryFrom<Eip712TypedDataSerde> for Eip712TypedData {
    type Error = EthError;

    fn try_from(serde_typed_data: Eip712TypedDataSerde) -> Result<Self> {
        let types = convert_types(&serde_typed_data.types)?;
        let values = convert_values(
            &serde_typed_data.primary_type,
            &types,
            &serde_typed_data.message,
        )?;

        Ok(Self {
            domain: serde_typed_data.domain.into(),
            primary_type: serde_typed_data.primary_type,
            types,
            values,
            ..Default::default()
        })
    }
}

/// Convert a JSON value to an EIP-712 field value by specified type.
fn convert_json_by_type(
    json_value: &serde_json::Value,
    field_type: &Eip712FieldType,
    struct_types: &HashMap<Eip712StructName, Eip712Struct>,
) -> Result<Eip712FieldValue> {
    let field_value = match field_type {
        Eip712FieldType::Address => json_to_address(json_value),
        Eip712FieldType::Bytes => json_to_bytes(json_value),
        Eip712FieldType::FixedBytes(fixed_len) => json_to_fixed_bytes(json_value, *fixed_len),
        Eip712FieldType::Int(_) => json_to_int(json_value),
        Eip712FieldType::Uint(_) => json_to_uint(json_value),
        Eip712FieldType::Bool => json_to_bool(json_value),
        Eip712FieldType::String => json_to_string(json_value),
        Eip712FieldType::Array(item_type) => json_to_array(json_value, item_type, struct_types),
        Eip712FieldType::FixedArray(item_type, fixed_len) => {
            json_to_fixed_array(json_value, item_type, *fixed_len, struct_types)
        }
        // Solidity type `tuple` is unsupported for EIP-712.
        Eip712FieldType::Tuple(_) => None,
        // Convert to nested struct values.
        Eip712FieldType::Struct(struct_name) => {
            json_value.as_object().and_then(|json_field_values| {
                let json_field_values = json_field_values.clone().into_iter().collect();
                convert_values(struct_name, struct_types, &json_field_values)
                    .map(|values| Eip712FieldValue::Struct(struct_name.clone(), values))
                    .ok()
            })
        }
    };

    field_value.ok_or_else(|| invalid_value_for_type_error(field_type, json_value))
}

/// Convert types to EIP-712 struct types.
fn convert_types(
    types: &HashMap<Eip712StructName, Vec<Eip712FieldSerde>>,
) -> Result<HashMap<Eip712StructName, Eip712Struct>> {
    types
        .iter()
        .map(|(name, fields)| {
            let fields = fields
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<Eip712Field>>>()?;
            Ok((name.clone(), Eip712Struct::new(name.clone(), fields)))
        })
        .collect::<Result<HashMap<Eip712StructName, Eip712Struct>>>()
}

/// Convert JSON values to EIP-712 field values of specified struct type.
fn convert_values(
    struct_name: &str,
    struct_types: &HashMap<Eip712StructName, Eip712Struct>,
    json_values: &HashMap<Eip712FieldName, serde_json::Value>,
) -> Result<HashMap<Eip712FieldName, Eip712FieldValue>> {
    let struct_type = struct_types
        .get(struct_name)
        .ok_or_else(|| Eip712Error::MissingTypeError(struct_name.to_owned()))?;

    struct_type
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let json_value = json_values
                .get(field_name)
                .ok_or_else(|| Eip712Error::MissingFieldError(field_name.clone()))?;

            let field_value = convert_json_by_type(json_value, &field.r#type, struct_types)?;

            Ok((field_name.clone(), field_value))
        })
        .collect()
}

/// Construct EIP-712 error `InvalidValueForType`.
#[inline]
fn invalid_value_for_type_error(
    field_type: &Eip712FieldType,
    json_value: &serde_json::Value,
) -> EthError {
    Eip712Error::InvalidValueForType {
        r#type: format!("{field_type:?}"),
        value: json_value.to_string(),
    }
    .into()
}

#[inline]
fn json_to_address(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::String(s) => H160::from_str(s).map(Eip712FieldValue::Address).ok(),
        _ => None,
    }
}

#[inline]
fn json_to_array(
    json_value: &serde_json::Value,
    item_type: &Eip712FieldType,
    struct_types: &HashMap<Eip712StructName, Eip712Struct>,
) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| convert_json_by_type(v, item_type, struct_types))
            .collect::<Result<Vec<_>>>()
            .map(Eip712FieldValue::Array)
            .ok(),
        _ => None,
    }
}

#[inline]
fn json_to_bool(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Bool(b) => Some(Eip712FieldValue::Bool(*b)),
        _ => None,
    }
}

#[inline]
fn json_to_bytes(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|i| i.as_u64().and_then(|i| u8::try_from(i).ok()))
            .collect::<Option<Vec<u8>>>()
            .map(Eip712FieldValue::Bytes),
        _ => None,
    }
}

#[inline]
fn json_to_fixed_array(
    json_value: &serde_json::Value,
    item_type: &Eip712FieldType,
    fixed_len: usize,
    struct_types: &HashMap<Eip712StructName, Eip712Struct>,
) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| convert_json_by_type(v, item_type, struct_types))
            .collect::<Result<Vec<_>>>()
            .ok()
            .and_then(|a| if a.len() == fixed_len { Some(a) } else { None })
            .map(Eip712FieldValue::FixedArray),
        _ => None,
    }
}

#[inline]
fn json_to_fixed_bytes(
    json_value: &serde_json::Value,
    fixed_len: usize,
) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|i| i.as_u64().and_then(|i| u8::try_from(i).ok()))
            .collect::<Option<Vec<u8>>>()
            .and_then(|a| if a.len() == fixed_len { Some(a) } else { None })
            .map(Eip712FieldValue::FixedBytes),
        serde_json::Value::String(s) => hex_decode(s)
            .ok()
            .and_then(|a| if a.len() == fixed_len { Some(a) } else { None })
            .map(Eip712FieldValue::FixedBytes),
        _ => None,
    }
}

#[inline]
fn json_to_int(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Number(i) => i.as_i64().map(Into::into),
        serde_json::Value::String(s) => U256::from_str(s).ok(),
        _ => None,
    }
    .map(Eip712FieldValue::Int)
}

#[inline]
fn json_to_string(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::String(s) => Some(Eip712FieldValue::String(s.clone())),
        _ => None,
    }
}

#[inline]
fn json_to_uint(json_value: &serde_json::Value) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Number(u) => u.as_u64().map(Into::into),
        serde_json::Value::String(s) => U256::from_str(s).ok(),
        _ => None,
    }
    .map(Eip712FieldValue::Uint)
}

#[cfg(test)]
mod eip712_deserializing_tests {
    use super::*;

    const SIMPLE_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "name": "Ether Person",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
                "salt": "eip712-test-75F0CCte"
            },
            "message": {
                "name": "Bob",
                "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
            },
            "primaryType": "Person",
            "types": {
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    const RECURSIVELY_NESTED_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "chainId": "25",
                "verifyingContract": "0xEeEEeeeeEEEEeEEEEEEeEeEeeEeEEEeEeeeeeeeE"
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
                "contents": "0x0a0b0c0d"
            },
            "primaryType": "Mail",
            "types": {
                "Mail": [
                    { "name": "from", "type": "Person" },
                    { "name": "to", "type": "Person" },
                    { "name": "contents", "type": "bytes4" }
                ],
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    #[test]
    fn test_eip712_simple_typed_data_deserializing() {
        let typed_data = Eip712TypedData::new(SIMPLE_JSON_TYPED_DATA).unwrap();
        assert_eq!(typed_data.primary_type, "Person");

        // Validate domain.
        assert_eq!(typed_data.domain.name, Some("Ether Person".to_owned()));
        assert_eq!(typed_data.domain.version, Some("1".to_owned()));
        assert_eq!(typed_data.domain.chain_id, Some(1_u64.into()));
        assert_eq!(
            typed_data.domain.verifying_contract,
            Some(H160::from_str("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC").unwrap())
        );
        assert_eq!(
            typed_data.domain.salt,
            Some(keccak256("eip712-test-75F0CCte"))
        );

        // Validate types.
        let mut types = HashMap::new();
        let inserted_result = types.insert(
            "Person".to_owned(),
            Eip712Struct::new(
                "Person".to_owned(),
                vec![
                    Eip712Field {
                        name: "name".to_owned(),
                        r#type: Eip712FieldType::String,
                    },
                    Eip712Field {
                        name: "wallet".to_owned(),
                        r#type: Eip712FieldType::Address,
                    },
                ],
            ),
        );
        assert!(inserted_result.is_none());
        assert_eq!(typed_data.types, types);

        // Validate values.
        let mut values = HashMap::new();
        let inserted_result = values.insert(
            "name".to_owned(),
            Eip712FieldValue::String("Bob".to_owned()),
        );
        assert!(inserted_result.is_none());
        let inserted_result = values.insert(
            "wallet".to_owned(),
            Eip712FieldValue::Address(
                H160::from_str("0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB").unwrap(),
            ),
        );
        assert!(inserted_result.is_none());
        assert_eq!(typed_data.values, values);
    }

    #[test]
    fn test_eip712_recursively_nested_typed_data_deserializing() {
        let typed_data = Eip712TypedData::new(RECURSIVELY_NESTED_JSON_TYPED_DATA).unwrap();
        assert_eq!(typed_data.primary_type, "Mail");

        // Validate domain.
        assert_eq!(typed_data.domain.name, None);
        assert_eq!(typed_data.domain.version, None);
        assert_eq!(typed_data.domain.chain_id, Some(25_u64.into()));
        assert_eq!(
            typed_data.domain.verifying_contract,
            Some(H160::from_str("0xEeEEeeeeEEEEeEEEEEEeEeEeeEeEEEeEeeeeeeeE").unwrap())
        );
        assert_eq!(typed_data.domain.salt, None);

        // Validate types.
        let mut types = HashMap::new();
        let inserted_result = types.insert(
            "Mail".to_owned(),
            Eip712Struct::new(
                "Mail".to_owned(),
                vec![
                    Eip712Field {
                        name: "from".to_owned(),
                        r#type: Eip712FieldType::Struct("Person".to_owned()),
                    },
                    Eip712Field {
                        name: "to".to_owned(),
                        r#type: Eip712FieldType::Struct("Person".to_owned()),
                    },
                    Eip712Field {
                        name: "contents".to_owned(),
                        r#type: Eip712FieldType::FixedBytes(4),
                    },
                ],
            ),
        );
        assert!(inserted_result.is_none());
        let inserted_result = types.insert(
            "Person".to_owned(),
            Eip712Struct::new(
                "Person".to_owned(),
                vec![
                    Eip712Field {
                        name: "name".to_owned(),
                        r#type: Eip712FieldType::String,
                    },
                    Eip712Field {
                        name: "wallet".to_owned(),
                        r#type: Eip712FieldType::Address,
                    },
                ],
            ),
        );
        assert!(inserted_result.is_none());
        assert_eq!(typed_data.types, types);

        // Validate values.
        let mut from_person_values = HashMap::new();
        let inserted_result = from_person_values.insert(
            "name".to_owned(),
            Eip712FieldValue::String("Cow".to_owned()),
        );
        assert!(inserted_result.is_none());
        let inserted_result = from_person_values.insert(
            "wallet".to_owned(),
            Eip712FieldValue::Address(
                H160::from_str("0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826").unwrap(),
            ),
        );
        assert!(inserted_result.is_none());
        let mut to_person_values = HashMap::new();
        let inserted_result = to_person_values.insert(
            "name".to_owned(),
            Eip712FieldValue::String("Bob".to_owned()),
        );
        assert!(inserted_result.is_none());
        let inserted_result = to_person_values.insert(
            "wallet".to_owned(),
            Eip712FieldValue::Address(
                H160::from_str("0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB").unwrap(),
            ),
        );
        assert!(inserted_result.is_none());
        let mut values = HashMap::new();
        let inserted_result = values.insert(
            "from".to_owned(),
            Eip712FieldValue::Struct("Person".to_owned(), from_person_values),
        );
        assert!(inserted_result.is_none());
        let inserted_result = values.insert(
            "to".to_owned(),
            Eip712FieldValue::Struct("Person".to_owned(), to_person_values),
        );
        assert!(inserted_result.is_none());
        let inserted_result = values.insert(
            "contents".to_owned(),
            Eip712FieldValue::FixedBytes(vec![10, 11, 12, 13]),
        );
        assert!(inserted_result.is_none());
        assert_eq!(typed_data.values, values);
    }

    // TODO: Add more cases for detailed JSON and field type conversions.
}
