use crate::node::ethereum::eip712::{
    Eip712Field, Eip712FieldName, Eip712FieldType, Eip712FieldValue, Eip712Struct,
    Eip712StructName, Eip712TypedData, Result,
};
use crate::transaction::{Eip712Error, EthError};
use ethers::prelude::{H160, U256};
use ethers::types::transaction::eip712::EIP712Domain;
use ethers::utils::keccak256;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

/// EIP-712 domain for deserializing
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Eip712DomainSerde {
    name: String,
    version: String,
    chain_id: u64,
    verifying_contract: H160,
    salt: Option<String>,
}

impl From<Eip712DomainSerde> for EIP712Domain {
    fn from(domain: Eip712DomainSerde) -> Self {
        Self {
            name: domain.name,
            version: domain.version,
            chain_id: domain.chain_id.into(),
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
            r#type: serde_field.r#type.as_str().try_into()?,
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
        let primary_struct = types
            .get(&serde_typed_data.primary_type)
            .ok_or_else(|| Eip712Error::MissingTypeError(serde_typed_data.primary_type.clone()))?;
        let values = convert_message_to_values(primary_struct, &serde_typed_data.message)?;

        Ok(Self {
            domain: serde_typed_data.domain.into(),
            primary_type: serde_typed_data.primary_type,
            types,
            values,
        })
    }
}

/// Convert a JSON value to an EIP-712 field value by specified type.
fn convert_json_by_type(
    json_value: &serde_json::Value,
    field_type: &Eip712FieldType,
) -> Result<Eip712FieldValue> {
    let field_value = match field_type {
        Eip712FieldType::Address => json_to_address(json_value),
        Eip712FieldType::Bytes => json_to_bytes(json_value),
        Eip712FieldType::FixedBytes(fixed_len) => json_to_fixed_bytes(json_value, *fixed_len),
        Eip712FieldType::Int(_) => json_to_int(json_value),
        Eip712FieldType::Uint(_) => json_to_uint(json_value),
        Eip712FieldType::Bool => json_to_bool(json_value),
        Eip712FieldType::String => json_to_string(json_value),
        Eip712FieldType::Array(item_type) => json_to_array(json_value, item_type),
        Eip712FieldType::FixedArray(item_type, fixed_len) => {
            json_to_fixed_array(json_value, item_type, *fixed_len)
        }
        // TODO: Extend both `EthAbiParamType` and `EthAbiToken` to support nested struct.
        Eip712FieldType::Tuple(_) => None,
    };

    field_value.ok_or_else(|| invalid_value_for_type_error(field_type, json_value))
}

/// Convert to values of EIP-712 typed data from deserialized message.
fn convert_message_to_values(
    struct_type: &Eip712Struct,
    message: &HashMap<Eip712FieldName, serde_json::Value>,
) -> Result<HashMap<Eip712FieldName, Eip712FieldValue>> {
    struct_type
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let json_value = message
                .get(field_name)
                .ok_or_else(|| Eip712Error::MissingFieldError(field_name.clone()))?;
            let field_value = convert_json_by_type(json_value, &field.r#type)?;

            Ok((field_name.clone(), field_value))
        })
        .collect()
}

/// Convert to types of EIP-712 typed data.
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
) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| convert_json_by_type(v, item_type))
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
) -> Option<Eip712FieldValue> {
    match json_value {
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| convert_json_by_type(v, item_type))
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

    const JSON_TYPED_DATA: &str = r#"
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

    #[test]
    fn test_eip712_typed_data_deserializing() {
        let typed_data = Eip712TypedData::new(JSON_TYPED_DATA).unwrap();
        assert_eq!(typed_data.primary_type, "Person");

        // Validate domain.
        assert_eq!(typed_data.domain.name, "Ether Person");
        assert_eq!(typed_data.domain.version, "1");
        assert_eq!(typed_data.domain.chain_id, 1_u64.into());
        assert_eq!(
            typed_data.domain.verifying_contract,
            H160::from_str("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC").unwrap()
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

    // TODO: Add more cases for detailed JSON and field type conversions.
}
