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
    ) -> Result<String, EthError> {
        let domain_separator = self.domain.internal.separator();
        let hash = self.build_hash(values)?;
        let digest_input = [&[0x19, 0x01], &domain_separator[..], &hash[..]].concat();
        let encoded_data = keccak256(digest_input);

        Ok(H256::from(encoded_data).to_string())
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

    #[test]
    fn test_eip712_sign_typed_data() {
        // Ok(self.sign_hash(H256::from(encoded), false))
        todo!()
    }
}
