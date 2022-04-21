use ethers::utils::hex::{self, FromHexError};
use serde::de;

#[allow(dead_code)]
pub(crate) fn deserialize_option_u64_from_any<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let json_value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    Ok(match json_value {
        serde_json::Value::Number(u) => Some(
            u.as_u64()
                .ok_or_else(|| de::Error::custom(format!("Must be an u64: {u}")))?,
        ),
        serde_json::Value::String(s) => Some(s.parse::<u64>().map_err(de::Error::custom)?),
        _ => None,
    })
}

pub(crate) fn hex_decode(hex_string: &str) -> Result<Vec<u8>, FromHexError> {
    let hex_string = hex_string.strip_prefix("0x").unwrap_or(hex_string);
    hex::decode(hex_string)
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_utils_hex_decoding() {
        let hex_string = "0xaf6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747";
        let decoded_data = hex_decode(hex_string).unwrap();
        assert_eq!(
            decoded_data,
            [
                175, 111, 41, 63, 38, 33, 191, 181, 167, 13, 124, 241, 35, 89, 107, 209, 72, 39,
                247, 55, 105, 194, 78, 223, 38, 136, 179, 206, 44, 134, 215, 71
            ]
        );

        assert_eq!(hex_decode(&hex_string[2..]).unwrap(), decoded_data);
    }
}
