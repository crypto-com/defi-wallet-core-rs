use crate::transaction::cosmos_sdk::CosmosError;
use crate::wallet::SecretKey;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::SignDoc;
use ethers::utils::hex;
use eyre::Context;
use std::sync::Arc;

/// Cosmos Signer
pub struct CosmosSigner {
    secret_key: Arc<SecretKey>,
}

impl CosmosSigner {
    /// Create an instance via a secret key.
    pub fn new(secret_key: Arc<SecretKey>) -> Self {
        Self { secret_key }
    }

    /// Sign the protobuf bytes directly.
    pub fn sign_direct(
        &self,
        chain_id: &str,
        account_number: &str,
        auth_info_bytes: &str,
        body_bytes: &str,
    ) -> Result<String, CosmosError> {
        let account_number = account_number
            .parse::<u64>()
            .context("Argument account_number must be an u64: {account_number}")?;
        let auth_info_bytes = hex::decode(auth_info_bytes)
            .context("Argument auth_info_bytes must be a HEX string: {auth_info_bytes}")?;
        let body_bytes = hex::decode(body_bytes)
            .context("Argument body_bytes must be a HEX string: {body_bytes}")?;

        let signed_bytes = CosmosProtoSignDoc::new(
            body_bytes,
            auth_info_bytes,
            chain_id.to_owned(),
            account_number,
        )
        .sign_into(&self.secret_key)?;

        Ok(hex::encode(signed_bytes))
    }
}

/// SignDoc for generating sign bytes from protobuf
struct CosmosProtoSignDoc {
    inner: SignDoc,
}

impl CosmosProtoSignDoc {
    /// Create an instance. User needs to assure protobuf bytes are correct.
    fn new(
        body_bytes: Vec<u8>,
        auth_info_bytes: Vec<u8>,
        chain_id: String,
        account_number: u64,
    ) -> Self {
        Self {
            inner: SignDoc {
                body_bytes,
                auth_info_bytes,
                chain_id,
                account_number,
            },
        }
    }

    /// Sign this SignDoc and produce a Raw transaction. The protobuf bytes are
    /// moved out after calling this function.
    fn sign_into(self, secret_key: &SecretKey) -> Result<Vec<u8>, CosmosError> {
        let signing_key = SigningKey::new(Box::new(secret_key.get_signing_key()));
        Ok(self.inner.sign(&signing_key)?.to_bytes()?)
    }
}

#[cfg(test)]
mod cosmos_signing_tests {
    use super::*;
    use crate::wallet::HDWallet;

    const MNEMONIC: &str = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";

    #[test]
    fn test_protobuf_signing() {
        let auth_info_bytes = "0a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c";
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let wallet = HDWallet::recover_wallet(MNEMONIC.to_string(), None).unwrap();
        let secret_key = wallet.get_key("m/44'/118'/0'/0/0".to_string()).unwrap();
        let signer = CosmosSigner::new(secret_key);
        let signature = signer
            .sign_direct("cosmoshub-4", "1", auth_info_bytes, body_bytes)
            .unwrap();

        assert_eq!(signature, "0a93010a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d12073132333435363712210a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c1a4010fc966e8b88f70cf52de3aeb16700dbfe228be19cc93f202b5e0e0e4899694c1671ba4f41b9ca442e05342e3f425a8c8ead8b35261c8dd9c75b4ce6fcb95dc3");
    }
}
