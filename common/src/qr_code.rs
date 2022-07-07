use std::{fmt::Display, num::ParseIntError, str::FromStr};

use ethers::{
    abi::{ethereum_types::FromDecStrErr, ParamType},
    prelude::{Address, NameOrAddress, U256},
};
use regex::Regex;

use crate::{abi::EthAbiParamType, EthAmount, EthError};

/// Parameter value types in EIP681 requests
/// TODO: negative/signed numbers?
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Address(NameOrAddress),
    String(String),
    Number(U256),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Address(NameOrAddress::Address(address)) => write!(f, "{:?}", address),
            Value::Address(NameOrAddress::Name(address)) => write!(f, "{}", address),
            Value::String(string) => write!(f, "{}", string),
            Value::Number(number) => write!(f, "{}", number),
        }
    }
}

/// Possible parameters in EIP681 requests
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Value(EthAmount),
    GasLimit(u64),
    GasPrice(u64),
    Other(ParamType, Value),
}

/// Structure holding the EIP681 request
pub struct EIP681Request {
    /// if the "pay-" prefix is included;
    /// it's optional for now, but if there's a new URN format in the future, it should be included
    /// "Future upgrades that are partially or fully incompatible with this proposal must use a prefix other than pay-
    /// that is separated by a dash (-) character from whatever follows it."
    pub has_pay_tag: bool,
    /// "target_address is mandatory and denotes either the beneficiary of native token payment
    /// or the contract address with which the user is asked to interact."
    pub target_address: NameOrAddress,
    /// "chain_id is optional and contains the decimal chain ID, such that transactions on various
    /// test- and private networks can be requested. If no chain_id is present,
    /// the client's current network setting remains effective."
    pub chain_id: Option<u64>,
    /// "If function_name is missing, then the URL is requesting payment in the native token of the blockchain,
    /// which is ether in our case. The amount is specified in value parameter, in the atomic unit (i.e. wei).
    /// The use of scientific notation is strongly encouraged."
    pub function_name: Option<String>,
    /// "Note that the indicated amount is only a suggestion (as are all the supplied arguments)
    /// which the user is free to change. With no indicated amount,
    /// the user should be prompted to enter the amount to be paid.
    /// Similarly gasLimit and gasPrice are suggested user-editable values for gas limit and gas price,
    /// respectively, for the requested transaction.
    /// It is acceptable to abbreviate gasLimit as gas, the two are treated synonymously."
    pub parameters: Vec<Parameter>,
}

impl EIP681Request {
    /// Constructor to help to construct a normal ETH/native token transfer request
    pub fn get_normal_transfer(chain_id: Option<u64>, amount: EthAmount, to: Address) -> Self {
        let parameters = vec![Parameter::Value(amount)];
        EIP681Request {
            has_pay_tag: false,
            target_address: NameOrAddress::Address(to),
            chain_id,
            function_name: None,
            parameters,
        }
    }

    /// Constructor to help to construct a ERC20 token transfer request
    pub fn get_erc20_transfer(
        contract_address: Address,
        chain_id: Option<u64>,
        amount: U256,
        to: Address,
    ) -> Self {
        let parameters = vec![
            Parameter::Other(
                ParamType::Address,
                Value::Address(NameOrAddress::Address(to)),
            ),
            Parameter::Other(ParamType::Uint(256), Value::Number(amount)),
        ];
        EIP681Request {
            has_pay_tag: false,
            target_address: NameOrAddress::Address(contract_address),
            chain_id,
            function_name: Some("transfer".to_string()),
            parameters,
        }
    }
}

impl Display for EIP681Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ethereum:")?;
        if self.has_pay_tag {
            write!(f, "pay-")?;
        }
        match self.target_address {
            NameOrAddress::Address(ref address) => write!(f, "{:?}", address)?,
            NameOrAddress::Name(ref name) => write!(f, "{}", name)?,
        };
        if let Some(chain_id) = self.chain_id {
            write!(f, "@{}", chain_id)?;
        }
        if let Some(ref function_name) = self.function_name {
            write!(f, "/{}", function_name)?;
        }
        if !self.parameters.is_empty() {
            write!(f, "?")?;
        }
        for (i, parameter) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, "&")?;
            }
            match parameter {
                Parameter::Value(EthAmount::EthDecimal { amount }) => {
                    write!(f, "value={}e18", amount)?
                }
                Parameter::Value(EthAmount::GweiDecimal { amount }) => {
                    write!(f, "value={}e9", amount)?
                }
                Parameter::Value(EthAmount::WeiDecimal { amount }) => {
                    write!(f, "value={}", amount)?
                }
                Parameter::GasLimit(ref value) => write!(f, "gasLimit={}", value)?,
                Parameter::GasPrice(ref value) => write!(f, "gasPrice={}", value)?,
                Parameter::Other(ref name, ref value) => {
                    write!(f, "{}={}", name, value)?;
                }
            }
        }

        Ok(())
    }
}

/// Parsing error options
#[derive(Debug, thiserror::Error)]
pub enum EIP681ParseError {
    #[error("Invalid EIP681 request (note that the current parser is incomplete and may reject some valid requests)")]
    InvalidRequest,
    #[error("Invalid address: {0}")]
    InvalidAddress(rustc_hex::FromHexError),
    #[error("Invalid chain id or number: {0}")]
    InvalidChainIdOrNumber(ParseIntError),
    #[error("Invalid EIP681 parameter (note that the current parser is incomplete and may reject some valid parameters)")]
    InvalidParameter,
    #[error("Unsupported exponent (for the moment): {0}")]
    UnsupportedExponent(String),
    #[error("Invalid parameter key: {0}")]
    InvalidKey(EthError),
    #[error("Invalid number: {0}")]
    InvalidNumberValue(FromDecStrErr),
}

impl FromStr for EIP681Request {
    type Err = EIP681ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            /*
            request                 = schema_prefix target_address [ "@" chain_id ] [ "/" function_name ] [ "?" parameters ]
            schema_prefix           = "ethereum" ":" [ "pay-" ]
            target_address          = ethereum_address
            chain_id                = 1*DIGIT
            function_name           = STRING
            ethereum_address        = ( "0x" 40*HEXDIG ) / ENS_NAME
            parameters              = parameter *( "&" parameter )
            parameter               = key "=" value
            key                     = "value" / "gas" / "gasLimit" / "gasPrice" / TYPE
            value                   = number / ethereum_address / STRING
            number                  = [ "-" / "+" ] *DIGIT [ "." 1*DIGIT ] [ ( "e" / "E" ) [ 1*DIGIT ] ]
            */
            // FIXME: ENS_NAME and some parameter values are not supported yet
            static ref EIP681_EXP: Regex = Regex::new(r"^ethereum:(pay-)?(0x[[:xdigit:]]{40})(@\d+)?(/\w+)?(\?\w+=[\w|\.]+(&\w+=[\w|\.]+)*)?$").expect("EIP681_EXP regex should compile");
        }
        let captures = EIP681_EXP
            .captures(s)
            .ok_or(EIP681ParseError::InvalidRequest)?;
        let has_pay_tag = captures.get(1).is_some();
        let etherum_address =
            Address::from_str(&captures[2]).map_err(EIP681ParseError::InvalidAddress)?;
        let target_address = NameOrAddress::Address(etherum_address);
        let chain_id = match captures.get(3) {
            Some(capture) => Some(
                capture
                    .as_str()
                    .parse::<u64>()
                    .map_err(EIP681ParseError::InvalidChainIdOrNumber)?,
            ),
            None => None,
        };
        let function_name = captures.get(4).map(|m| m.as_str()[1..].to_string());
        let mut parameters = Vec::new();
        if let Some(parameters_str) = captures.get(5) {
            for parameter_str in parameters_str.as_str()[1..].split('&') {
                let mut parameter_parts = parameter_str.split('=');
                let key = parameter_parts
                    .next()
                    .ok_or(EIP681ParseError::InvalidParameter)?;
                let value = parameter_parts
                    .next()
                    .ok_or(EIP681ParseError::InvalidParameter)?;
                match key {
                    "value" => {
                        let mut value_parts = value.split(&['e', 'E']);
                        let amount = value_parts
                            .next()
                            .ok_or(EIP681ParseError::InvalidParameter)?
                            .to_string();
                        match value_parts.next() {
                            Some("18") => {
                                parameters.push(Parameter::Value(EthAmount::EthDecimal { amount }))
                            }
                            Some("9") => {
                                parameters.push(Parameter::Value(EthAmount::GweiDecimal { amount }))
                            }
                            Some("0") | None => {
                                parameters.push(Parameter::Value(EthAmount::WeiDecimal { amount }))
                            }
                            Some(exponent) => {
                                return Err(EIP681ParseError::UnsupportedExponent(
                                    exponent.to_string(),
                                ))
                            }
                        };
                    }
                    "gasLimit" | "gas" => {
                        let value = value
                            .parse::<u64>()
                            .map_err(EIP681ParseError::InvalidChainIdOrNumber)?;
                        parameters.push(Parameter::GasLimit(value));
                    }
                    "gasPrice" => {
                        let value = value
                            .parse::<u64>()
                            .map_err(EIP681ParseError::InvalidChainIdOrNumber)?;
                        parameters.push(Parameter::GasPrice(value));
                    }
                    _ => {
                        let param_type = ParamType::try_from(&EthAbiParamType::from(key))
                            .map_err(EIP681ParseError::InvalidKey)?;
                        let value = match param_type {
                            ParamType::Uint(_) | ParamType::Int(_) => {
                                let value = U256::from_dec_str(value)
                                    .map_err(EIP681ParseError::InvalidNumberValue)?;
                                Value::Number(value)
                            }
                            ParamType::Address => {
                                let value = Address::from_str(value)
                                    .map_err(EIP681ParseError::InvalidAddress)?;
                                Value::Address(NameOrAddress::Address(value))
                            }
                            _ => Value::String(value.to_string()),
                        };
                        let param = Parameter::Other(param_type, value);
                        parameters.push(param);
                    }
                }
            }
        }
        Ok(Self {
            has_pay_tag,
            target_address,
            chain_id,
            function_name,
            parameters,
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use ethers::{
        abi::ParamType,
        prelude::{Address, NameOrAddress, U256},
    };

    use crate::{
        qr_code::{EIP681Request, Parameter, Value},
        EthAmount,
    };

    #[test]
    pub fn test_parse() {
        let request = EIP681Request::from_str(
            "ethereum:0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359?value=2.014e18",
        )
        .unwrap();
        assert_eq!(request.has_pay_tag, false);
        assert_eq!(
            request.target_address,
            NameOrAddress::Address(
                Address::from_str("0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359").unwrap()
            )
        );
        assert_eq!(request.chain_id, None);
        assert_eq!(request.function_name, None);
        assert_eq!(request.parameters.len(), 1);
        assert_eq!(
            request.parameters[0],
            Parameter::Value(EthAmount::EthDecimal {
                amount: "2.014".to_string()
            })
        );
        let request2 = EIP681Request::from_str("ethereum:0x89205a3a3b2a69de6dbf7f01ed13b2108b2c43e7/transfer?address=0x8e23ee67d1332ad560396262c48ffbb01f93d052&uint256=1").unwrap();
        assert_eq!(request2.has_pay_tag, false);
        assert_eq!(
            request2.target_address,
            NameOrAddress::Address(
                Address::from_str("0x89205a3a3b2a69de6dbf7f01ed13b2108b2c43e7").unwrap()
            )
        );
        assert_eq!(request2.chain_id, None);
        assert_eq!(request2.function_name, Some("transfer".to_string()));
        assert_eq!(request2.parameters.len(), 2);
        assert_eq!(
            request2.parameters[0],
            Parameter::Other(
                ParamType::Address,
                Value::Address(NameOrAddress::Address(
                    Address::from_str("0x8e23ee67d1332ad560396262c48ffbb01f93d052").unwrap()
                ))
            )
        );
        assert_eq!(
            request2.parameters[1],
            Parameter::Other(
                ParamType::Uint(256),
                Value::Number(U256::from_dec_str("1").unwrap())
            )
        );
    }

    #[test]
    pub fn test_display() {
        let info = EIP681Request {
            has_pay_tag: false,
            target_address: NameOrAddress::Address(
                "0x0000000000000000000000000000000000000000"
                    .parse()
                    .unwrap(),
            ),
            chain_id: None,
            function_name: None,
            parameters: vec![],
        };
        assert_eq!(
            info.to_string(),
            "ethereum:0x0000000000000000000000000000000000000000"
        );
        let info = EIP681Request::get_normal_transfer(
            None,
            EthAmount::EthDecimal {
                amount: "2.014".to_string(),
            },
            "0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359"
                .parse()
                .unwrap(),
        );
        assert_eq!(
            info.to_string(),
            "ethereum:0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359?value=2.014e18"
        );
        let info = EIP681Request::get_erc20_transfer(
            "0x89205a3a3b2a69de6dbf7f01ed13b2108b2c43e7"
                .parse()
                .unwrap(),
            None,
            U256::from(1),
            "0x8e23ee67d1332ad560396262c48ffbb01f93d052"
                .parse()
                .unwrap(),
        );
        assert_eq!(
            info.to_string(),
            "ethereum:0x89205a3a3b2a69de6dbf7f01ed13b2108b2c43e7/transfer?address=0x8e23ee67d1332ad560396262c48ffbb01f93d052&uint256=1"
        );
    }
}
