use crate::node::ethereum::abi::{EthAbiParamType, EthAbiToken};
use crate::{address_from_str, EthError};
use ethers::prelude::abi::{ParamType, Token};
use ethers::prelude::{H256, U256};
use ethers::types::transaction::eip712::{self, encode_eip712_type, EIP712Domain, Eip712Error};
use ethers::utils::keccak256;
use std::collections::HashMap;

type Eip712FieldName = String;
type Eip712FieldType = EthAbiParamType;
type Eip712FieldValue = EthAbiToken;
type Eip712StructName = String;

/// EIP-712 typed struct
pub struct Eip712Struct {
    type_hash: U256,
    domain: Eip712Domain,
    fields: Vec<Eip712Field>,
}

impl Eip712Struct {
    /// Contruct an EIP-712 typed struct.
    pub fn new(
        struct_name: Eip712StructName,
        domain: Eip712Domain,
        fields: Vec<Eip712Field>,
    ) -> Result<Self, EthError> {
        let type_hash = build_struct_type_hash(&struct_name, &fields);
        Ok(Self {
            type_hash,
            domain,
            fields,
        })
    }

    /// Encode the typed values.
    pub fn encode(
        &self,
        values: HashMap<Eip712FieldName, Eip712FieldValue>,
    ) -> Result<Vec<u8>, EthError> {
        let domain_separator = self.domain.internal.separator();
        let hash = self.build_hash(values)?;
        let digest_input = [&[0x19, 0x01], &domain_separator[..], &hash[..]].concat();
        Ok(keccak256(digest_input).to_vec())
    }

    fn build_hash(
        &self,
        values: HashMap<Eip712FieldName, Eip712FieldValue>,
    ) -> Result<[u8; 32], EthError> {
        let mut items = vec![Token::Uint(self.type_hash)];

        let tokens = self
            .fields
            .iter()
            .map(|f| {
                let field_name = &f.name;
                values
                    .get(field_name)
                    .map(Into::into)
                    .ok_or_else(|| EthError::CommonError(format!("Missing field {field_name}")))
            })
            .collect::<Result<Vec<Token>, EthError>>()?;

        for token in tokens {
            match &token {
                Token::Tuple(_) => {
                    // TODO:
                    // Crate `ether-rs` uses `Token::Tuple` to save values of
                    // nested struct. Since we have already fixed to use
                    // `Eip712Struct`. Field of nested struct could be
                    // implemented in `Eip712Field`.
                    return Err(Eip712Error::NestedEip712StructNotImplemented.into());
                }
                _ => {
                    items.push(encode_eip712_type(token));
                }
            }
        }

        Ok(keccak256(ethers::abi::encode(&items)))
    }
}

/// EIP-712 domain
pub struct Eip712Domain {
    internal: EIP712Domain,
}

impl Eip712Domain {
    /// Contruct an EIP-712 domain.
    pub fn new(
        chain_id: u64,
        name: String,
        version: String,
        verifying_contract: String,
        salt: Option<String>,
    ) -> Result<Self, EthError> {
        Ok(Self {
            internal: EIP712Domain {
                name,
                version,
                chain_id: chain_id.into(),
                verifying_contract: address_from_str(&verifying_contract)?,
                salt: salt.map(|s| keccak256(s)),
            },
        })
    }
}

/// EIP-712 field
pub struct Eip712Field {
    name: String,
    r#type: Eip712FieldType,
}

impl Eip712Field {
    /// Contruct an EIP-712 struct field.
    pub fn new(name: String, r#type: Eip712FieldType) -> Self {
        Self { name, r#type }
    }
}

fn build_struct_type_hash(struct_name: &str, fields: &[Eip712Field]) -> U256 {
    let fields: Vec<(String, ParamType)> = fields
        .iter()
        .map(|f| (f.name.to_owned(), ParamType::from(&f.r#type)))
        .collect();
    let type_hash = eip712::make_type_hash(struct_name.to_owned(), &fields);
    U256::from(&type_hash[..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SecretKey;
    use ethers::prelude::{Address, LocalWallet, Signer};
    use std::str::FromStr;

    #[test]
    fn test_eip712_sign_typed_data() {
        let structure = test_eip712_struct();
        let encoded_data = structure.encode(test_eip712_values()).unwrap();
        let hash = H256::from_slice(&encoded_data);
        let wallet = test_eip712_wallet();

        assert_eq!(
            wallet.sign_hash(hash, false).to_vec(),
            [
                127, 151, 100, 76, 156, 92, 86, 250, 18, 47, 151, 225, 144, 86, 127, 25, 7, 53, 66,
                110, 73, 22, 156, 2, 7, 183, 201, 61, 141, 27, 198, 212, 65, 82, 185, 72, 193, 77,
                41, 27, 44, 158, 145, 19, 51, 16, 182, 123, 46, 120, 139, 33, 129, 84, 21, 180,
                227, 90, 91, 150, 92, 198, 205, 235, 28
            ],
        );
    }

    fn test_eip712_domain() -> Eip712Domain {
        Eip712Domain::new(
            1,
            "Eip712Test".to_string(),
            "1".to_string(),
            "0x0000000000000000000000000000000000000001".to_string(),
            Some("eip712-test-75F0CCte".to_string()),
        )
        .unwrap()
    }

    fn test_eip712_struct() -> Eip712Struct {
        Eip712Struct::new(
            "FooBar".to_string(),
            test_eip712_domain(),
            vec![
                Eip712Field::new("foo".to_string(), Eip712FieldType::Int(256)),
                Eip712Field::new("bar".to_string(), Eip712FieldType::Uint(256)),
                Eip712Field::new("fizz".to_string(), Eip712FieldType::Bytes),
                Eip712Field::new("buzz".to_string(), Eip712FieldType::FixedBytes(32)),
                Eip712Field::new("far".to_string(), Eip712FieldType::String),
                Eip712Field::new("out".to_string(), Eip712FieldType::Address),
            ],
        )
        .unwrap()
    }

    fn test_eip712_values() -> HashMap<Eip712FieldName, Eip712FieldValue> {
        let mut values = HashMap::new();
        values.insert("foo".to_string(), Eip712FieldValue::Int(U256::from(10)));
        values.insert("bar".to_string(), Eip712FieldValue::Uint(U256::from(20)));
        values.insert(
            "fizz".to_string(),
            Eip712FieldValue::Bytes(b"fizz".to_vec()),
        );
        values.insert(
            "buzz".to_string(),
            Eip712FieldValue::FixedBytes(keccak256("buzz").to_vec()),
        );
        values.insert(
            "far".to_string(),
            Eip712FieldValue::String("space".to_string()),
        );
        values.insert(
            "out".to_string(),
            Eip712FieldValue::Address(Address::from([0; 20])),
        );

        values
    }

    fn test_eip712_wallet() -> LocalWallet {
        let hex = "24e585759e492f5e810607c82c202476c22c5876b10247ebf8b2bb7f75dbed2e";
        let secret_key = SecretKey::from_hex(hex.to_owned()).unwrap();

        LocalWallet::from(secret_key.get_signing_key()).with_chain_id(1_u64)
    }
}
