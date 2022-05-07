use crate::transaction::cosmos_sdk::parser::structs::CosmosTxBody;
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;

/// Base parser for standard Cosmos messages
struct BaseParser {}

impl CosmosParser for BaseParser {
    fn transform_tx_body(&self, _tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        todo!()
    }
}

#[cfg(test)]
mod cosmos_base_parsing_tests {
    use super::*;

    #[test]
    fn test_proto_body_parsing() {
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let parser = CosmosParser::new();
        let body = parser.parse_proto_body(body_bytes).unwrap();
    }
}
