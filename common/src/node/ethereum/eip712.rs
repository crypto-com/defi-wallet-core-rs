use crate::{address_from_str, EthError};
use ethers::types::transaction::eip712::EIP712Domain;
use ethers::utils::keccak256;

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

/// EIP-712 typed data
pub struct Eip712TypedData {
    domain: Eip712Domain,
}

impl Eip712TypedData {
    /// Contruct an EIP-712 typed data.
    pub fn new(domain: Eip712Domain) -> Self {
        Self { domain }
    }

    /// Sign the typed data.
    pub fn sign(&self) -> Result<Vec<u8>, EthError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eip712_sign_typed_data() {
        todo!()
    }
}
