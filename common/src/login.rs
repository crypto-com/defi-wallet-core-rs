use crate::{EthError, SecretKey};
use siwe::Message;
use std::fmt::Display;

/// The wrapper structure that contains
/// all information from the EIP-4361 plaintext message:
/// https://eips.ethereum.org/EIPS/eip-4361
pub struct LoginInfo {
    /// the message content
    /// TODO: if external bindings are necessary,
    /// perhaps custom fields should be used?
    /// FIXME: siwe depends on `chrono`: https://rustsec.org/advisories/RUSTSEC-2020-0159
    pub msg: Message,
}

impl LoginInfo {
    /// constructs the plaintext message and signs it according to EIP-191
    /// (as per EIP-4361). The returned vector is a serialized recoverable signature
    /// (as used in Ethereum).
    pub fn sign(&self, private_key: &SecretKey) -> Result<Vec<u8>, EthError> {
        let message = self.msg.to_string();
        private_key
            .eth_sign(message.as_bytes(), self.msg.chain_id)
            .map_err(|_e| EthError::SignatureError)
            .map(|x| x.to_vec())
    }

    /// It verified the signature matches + also verifies the content of the message:
    /// - address in the message matches the address recovered from the signature
    /// - the time is valid
    /// ...
    /// NOTE: the server may still need to do extra verifications according to its needs
    /// (e.g. verify chain-id, nonce, uri + possibly fetch additional data associated
    /// with the given Ethereum address, such as ERC-20/ERC-721/ERC-1155 asset ownership)
    pub fn verify(&self, signature: &[u8]) -> Result<(), EthError> {
        let sig: [u8; 65] = signature
            .try_into()
            .map_err(|_e| EthError::SignatureError)?;
        let result = self.msg.verify(sig, None, None, None);
        result.map_err(|_e| EthError::SignatureError).map(|_| ())
    }
}

impl Display for LoginInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::{EthNetwork, LoginInfo, SecretKey, WalletCoin, WalletCoinFunc};
    use ethers::prelude::Address;
    use siwe::Message;
    use std::str::FromStr;

    fn get_logininfo(mwallet: Option<&SecretKey>) -> LoginInfo {
        let address = mwallet
            .map(|wallet| {
                WalletCoinFunc {
                    coin: WalletCoin::Ethereum {
                        network: EthNetwork::Mainnet,
                    },
                }
                .derive_address(wallet)
                .expect("address string")
            })
            .unwrap_or_else(|| "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string());
        let msg = Message {
            address: Address::from_str(&address).unwrap().into(),
            domain: "service.org".parse().unwrap(),
            statement: Some(
                "I accept the ServiceOrg Terms of Service: https://service.org/tos".to_string(),
            ),
            version: "1".parse().unwrap(),
            chain_id: 1,
            uri: "https://service.org/login".parse().unwrap(),
            nonce: "32891756".to_string(),
            issued_at: "2021-09-30T16:25:24Z".parse().unwrap(),
            expiration_time: None,
            not_before: None,
            request_id: None,
            resources: vec![
                "ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/"
                    .parse()
                    .unwrap(),
                "https://example.com/my-web2-claim.json".parse().unwrap(),
            ],
        };
        LoginInfo { msg }
    }

    #[test]
    pub fn test_sign() {
        let wallet = SecretKey::default();
        let login_info = get_logininfo(Some(&wallet));

        let signature = login_info.sign(&wallet);
        assert!(signature.is_ok());
        let sig = signature.unwrap();
        assert!(login_info.verify(&sig).is_ok());
    }

    #[test]
    pub fn test_display() {
        let expected_text = r#"service.org wants you to sign in with your Ethereum account:
0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2

I accept the ServiceOrg Terms of Service: https://service.org/tos

URI: https://service.org/login
Version: 1
Chain ID: 1
Nonce: 32891756
Issued At: 2021-09-30T16:25:24Z
Resources:
- ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/
- https://example.com/my-web2-claim.json"#;

        let login_info = get_logininfo(None);
        println!("{}", login_info);
        assert_eq!(expected_text, format!("{}", login_info));
    }
}
