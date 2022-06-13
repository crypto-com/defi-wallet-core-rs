use anyhow::Result;
use common::EthError;
use defi_wallet_core_common as common;
use std::fmt;

#[cxx::bridge(namespace = "org::defi_wallet_core")]
pub mod ffi {
    #[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
    pub struct U256 {
        data: [u64; 4],
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
    pub struct U256WithOverflow {
        result: U256,
        overflow: bool,
    }

    extern "Rust" {
        // U256
        #[cxx_name = "u256"]
        /// Convert from a decimal string.
        fn u256_from_dec_str(value: String) -> Result<U256>;
        #[cxx_name = "u256"]
        /// Converts a string slice in a given base to an integer. Only supports radixes of 10
        /// and 16.
        fn u256_from_str_radix(txt: String, radix: u32) -> Result<U256>;
        /// The maximum value which can be inhabited by this type.
        fn u256_max_value() -> U256;
        /// Convert to Decimal String
        fn to_string(self: &U256) -> String;
        /// Addition which saturates at the maximum value.
        fn saturating_add(self: &U256, other: U256) -> U256;
        /// Subtraction which saturates at zero.
        fn saturating_sub(self: &U256, other: U256) -> U256;
        /// Multiplication which saturates at maximum value.
        fn saturating_mul(self: &U256, other: U256) -> U256;
        /// Returns the addition along with a boolean indicating whether an arithmetic overflow
        /// would occur. If an overflow would have occurred then the wrapped value is returned.
        fn overflowing_add(self: &U256, other: U256) -> U256WithOverflow;
        /// Returns the subtraction along with a boolean indicating whether an arithmetic overflow
        /// would occur. If an overflow would have occurred then the wrapped value is returned.
        fn overflowing_sub(self: &U256, other: U256) -> U256WithOverflow;
        /// Returns the multiplication along with a boolean indicating whether an arithmetic overflow
        /// would occur. If an overflow would have occurred then the wrapped value is returned.
        fn overflowing_mul(self: &U256, other: U256) -> U256WithOverflow;
        /// Returns the fast exponentiation by squaring along with a boolean indicating whether an
        /// arithmetic overflow would occur. If an overflow would have occurred then the wrapped
        /// value is returned.
        fn overflowing_pow(self: &U256, other: U256) -> U256WithOverflow;
        /// Negates self in an overflowing fashion.
        /// Returns !self + 1 using wrapping operations to return the value that represents
        /// the negation of this unsigned value. Note that for positive unsigned values
        /// overflow always occurs, but negating 0 does not overflow.
        fn overflowing_neg(self: &U256) -> U256WithOverflow;
        /// add, exception is rasided if overflow
        fn add(self: &U256, other: U256) -> Result<U256>;
        /// sub, exception is rasided if overflow
        fn sub(self: &U256, other: U256) -> Result<U256>;
        /// mul, exception is rasided if overflow
        fn mul(self: &U256, other: U256) -> Result<U256>;
        /// pow, exception is rasided if overflow
        fn pow(self: &U256, other: U256) -> Result<U256>;
        /// Negates self in an overflowing fashion.
        /// Returns !self + 1 using wrapping operations to return the value that represents
        /// the negation of this unsigned value.
        fn neg(self: &U256) -> U256;
        /// Returns a pair `(self / other)`
        fn div(self: &U256, other: U256) -> U256;
        /// Returns a pair `(self % other)`
        fn rem(self: &U256, other: U256) -> U256;
        /// Write to the slice in big-endian format.
        fn to_big_endian(self: &U256, bytes: &mut Vec<u8>);
        /// Write to the slice in little-endian format.
        fn to_little_endian(self: &U256, bytes: &mut Vec<u8>);
        /// Converts the input to a U256 and converts from Ether to Wei.
        fn parse_ether(eth: String) -> Result<U256>;
        /// Multiplies the provided amount with 10^{units} provided.
        fn parse_units(amount: String, units: String) -> Result<U256>;
        /// Format the output for the user which prefer to see values in ether (instead of wei)
        /// Divides the input by 1e18
        fn format_ether(self: &U256) -> U256;
        /// Convert to common ethereum unit types: ether, gwei, or wei
        /// formatted in _ETH decimals_ (e.g. "1.50000...") wrapped as string
        fn format_units(self: &U256, units: String) -> Result<String>;
    }
}

use ffi::*;

#[macro_export]
/// Macro for implementing uint types
/// type: the uint type
/// other: the parameter used in function
/// overflow: the arithmetic result struct with overflow field
macro_rules! implement_uint {
    ($type:ident, $other:ty, $overflow:ident) => {
        /// Convert $type to ethers::types::$type
        impl From<$type> for ethers::types::$type {
            fn from(d: $type) -> Self {
                ethers::types::$type(d.data)
            }
        }

        /// Convert &$type to ethers::types::$type
        impl From<&$type> for ethers::types::$type {
            fn from(d: &$type) -> Self {
                ethers::types::$type(d.data)
            }
        }

        /// Convert ethers::types::$type to $type:
        impl From<ethers::types::$type> for $type {
            fn from(d: ethers::types::$type) -> Self {
                Self { data: d.0 }
            }
        }

        impl $overflow {
            fn get_data(self) -> $other {
                self.result
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let data: ethers::types::$type = self.into();
                write!(f, "{}", data.to_string())
            }
        }

        impl $type {
            /// Convert to decimal string
            pub fn to_dec_string(&self) -> String {
                self.to_string()
            }

            /// The maximum value which can be inhabited by this type.
            pub fn max_value() -> Self {
                ethers::types::$type::max_value().into()
            }

            /// Convert to Decimal String
            pub fn from_dec_str<S: AsRef<str>>(value: S) -> Result<Self, EthError> {
                Ok(ethers::types::$type::from_dec_str(value.as_ref())
                    .map_err(EthError::DecConversion)?
                    .into())
            }

            /// Converts a string slice in a given base to an integer. Only supports radixes of
            /// 10 and 16.
            pub fn from_str_radix<S: AsRef<str>>(txt: S, radix: u32) -> Result<Self, EthError> {
                Ok(ethers::types::$type::from_str_radix(txt.as_ref(), radix)
                    .map_err(EthError::StrRadixConversion)?
                    .into())
            }

            /// Addition which saturates at the maximum value.
            pub fn saturating_add(&self, other: $other) -> $other {
                let data: ethers::types::$type = self.into();
                let data = data.saturating_add(other.into()).into();
                data
            }

            /// Subtraction which saturates at zero.
            pub fn saturating_sub(&self, other: $other) -> $other {
                let data: ethers::types::$type = self.into();
                let data = data.saturating_sub(other.into()).into();
                data
            }

            /// Multiplication which saturates at maximum value.
            pub fn saturating_mul(&self, other: $other) -> $other {
                let data: ethers::types::$type = self.into();
                let data = data.saturating_mul(other.into()).into();
                data
            }

            /// Returns the addition along with a boolean indicating whether an arithmetic overflow
            /// would occur. If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_add(&self, other: $other) -> $overflow {
                let data: ethers::types::$type = self.into();
                let (result, overflow) = data.overflowing_add(other.into());
                $overflow {
                    result: result.into(),
                    overflow,
                }
            }

            /// Returns the subtraction along with a boolean indicating whether an arithmetic overflow
            /// would occur. If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_sub(&self, other: $other) -> $overflow {
                let data: ethers::types::$type = self.into();
                let (result, overflow) = data.overflowing_sub(other.into());
                $overflow {
                    result: result.into(),
                    overflow,
                }
            }

            /// Returns the multiplication along with a boolean indicating whether an arithmetic overflow
            /// would occur. If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_mul(&self, other: $other) -> $overflow {
                let data: ethers::types::$type = self.into();
                let (result, overflow) = data.overflowing_mul(other.into());
                $overflow {
                    result: result.into(),
                    overflow,
                }
            }

            /// Returns the fast exponentiation by squaring along with a boolean indicating whether an
            /// arithmetic overflow would occur. If an overflow would have occurred then the wrapped
            /// value is returned.
            pub fn overflowing_pow(&self, other: $other) -> $overflow {
                let data: ethers::types::$type = self.into();
                let (result, overflow) = data.overflowing_pow(other.into());
                $overflow {
                    result: result.into(),
                    overflow,
                }
            }

            /// Negates self in an overflowing fashion.
            /// Returns !self + 1 using wrapping operations to return the value that represents
            /// the negation of this unsigned value. Note that for positive unsigned values
            /// overflow always occurs, but negating 0 does not overflow.
            pub fn overflowing_neg(&self) -> $overflow {
                let data: ethers::types::$type = self.into();
                let (result, overflow) = data.overflowing_neg();
                $overflow {
                    result: result.into(),
                    overflow,
                }
            }

            /// add, exception is rasided if overflow
            pub fn add(&self, other: $other) -> Result<$other, EthError> {
                let u256_with_overflow = self.overflowing_add(other);
                if u256_with_overflow.overflow {
                    Err(EthError::Overflow)
                } else {
                    Ok(u256_with_overflow.get_data())
                }
            }

            /// sub, exception is rasided if overflow
            pub fn sub(&self, other: $other) -> Result<$other, EthError> {
                let u256_with_overflow = self.overflowing_sub(other);
                if u256_with_overflow.overflow {
                    Err(EthError::Overflow)
                } else {
                    Ok(u256_with_overflow.get_data())
                }
            }

            /// mul, exception is rasided if overflow
            pub fn mul(&self, other: $other) -> Result<$other, EthError> {
                let u256_with_overflow = self.overflowing_mul(other);
                if u256_with_overflow.overflow {
                    Err(EthError::Overflow)
                } else {
                    Ok(u256_with_overflow.get_data())
                }
            }

            /// pow, exception is rasided if overflow
            pub fn pow(&self, other: $other) -> Result<$other, EthError> {
                let u256_with_overflow = self.overflowing_pow(other);
                if u256_with_overflow.overflow {
                    Err(EthError::Overflow)
                } else {
                    Ok(u256_with_overflow.get_data())
                }
            }

            /// Negates self in an overflowing fashion.
            /// Returns !self + 1 using wrapping operations to return the value that represents
            /// the negation of this unsigned value.
            pub fn neg(&self) -> $other {
                let u256_with_overflow = self.overflowing_neg();
                u256_with_overflow.get_data()
            }

            /// Returns a pair `(self / other)`
            pub fn div(&self, other: $other) -> $other {
                let data: ethers::types::$type = self.into();
                let (div, _) = data.div_mod(other.into());
                let div = div.into();
                div
            }

            /// Returns a pair `(self % other)`
            pub fn rem(&self, other: $other) -> $other {
                let data: ethers::types::$type = self.into();
                let (_, rem) = data.div_mod(other.into());
                let rem = rem.into();
                rem
            }

            /// Write to the slice in big-endian format.
            pub fn to_big_endian<T: AsMut<[u8]>>(&self, mut bytes: T) {
                let data: ethers::types::$type = self.into();
                data.to_big_endian(bytes.as_mut());
            }

            /// Write to the slice in little-endian format.
            pub fn to_little_endian<T: AsMut<[u8]>>(&self, mut bytes: T) {
                let data: ethers::types::$type = self.into();
                data.to_little_endian(bytes.as_mut());
            }

            /// Check equality with `other`
            pub fn equal<T: fmt::Display>(&self, other: T) -> bool {
                self.to_string() == other.to_string()
            }
        }
    };
}

implement_uint!(U256, U256, U256WithOverflow);

impl U256 {
    /// Converts the input to a U256 and converts from Ether to Wei.
    pub fn parse_ether(eth: String) -> Result<U256, EthError> {
        Ok(ethers::utils::parse_ether(eth)
            .map_err(EthError::ParseError)?
            .into())
    }

    /// Multiplies the provided amount with 10^{units} provided.
    pub fn parse_units<S: AsRef<str>>(amount: String, units: S) -> Result<U256, EthError> {
        Ok(ethers::utils::parse_units(amount, units.as_ref())
            .map_err(EthError::ParseError)?
            .into())
    }

    /// Format the output for the user which prefer to see values in ether (instead of wei)
    /// Divides the input by 1e18
    pub fn format_ether(&self) -> U256 {
        ethers::utils::format_ether(self).into()
    }

    /// Convert to common ethereum unit types: ether, gwei, or wei
    /// formatted in _ETH decimals_ (e.g. "1.50000...") wrapped as string
    pub fn format_units<S: AsRef<str>>(&self, units: S) -> Result<String, EthError> {
        ethers::utils::format_units(self, units.as_ref()).map_err(EthError::ParseError)
    }
}

/// The maximum value which can be inhabited by this type.
pub fn u256_max_value() -> U256 {
    U256::max_value()
}

/// Convert from a decimal string.
pub fn u256_from_dec_str(value: String) -> Result<U256> {
    Ok(U256::from_dec_str(value)?)
}

/// Converts a string slice in a given base to an integer. Only supports radixes of 10 and 16.
pub fn u256_from_str_radix(txt: String, radix: u32) -> Result<U256> {
    Ok(U256::from_str_radix(txt, radix)?)
}

/// Converts the input to a U256 and converts from Ether to Wei.
pub fn parse_ether(eth: String) -> Result<U256> {
    Ok(U256::parse_ether(eth)?)
}

/// Multiplies the provided amount with 10^{units} provided.
pub fn parse_units(amount: String, units: String) -> Result<U256> {
    Ok(U256::parse_units(amount, units)?)
}

#[cfg(test)]
mod uint_tests {
    use super::*;

    #[test]
    fn test_u256() {
        let data = U256::from_dec_str("100000000000000000000000000").unwrap();
        assert_eq!(
            data,
            U256::from_str_radix("100000000000000000000000000", 10).unwrap()
        );
        assert_eq!(
            data,
            U256::from_str_radix("0x52B7D2DCC80CD2E4000000", 16).unwrap()
        );
        // +
        let data = data.add(U256::from_dec_str("100").unwrap()).unwrap();
        assert_eq!(
            data,
            U256::from_dec_str("100000000000000000000000100").unwrap()
        );

        // -
        let data = data.sub(U256::from_dec_str("10000000").unwrap()).unwrap();
        assert_eq!(
            data,
            U256::from_dec_str("99999999999999999990000100").unwrap()
        );

        // *
        let data = data.mul(U256::from_dec_str("2").unwrap()).unwrap();
        assert_eq!(
            data,
            U256::from_dec_str("199999999999999999980000200").unwrap()
        );

        // div
        let data = data.div(U256::from_dec_str("100").unwrap());
        assert_eq!(
            data,
            U256::from_dec_str("1999999999999999999800002").unwrap()
        );

        // rem
        let data = data.rem(U256::from_dec_str("1000000").unwrap());
        assert_eq!(data, U256::from_dec_str("800002").unwrap(),);

        // pow
        let data = data.pow(U256::from_dec_str("3").unwrap()).unwrap();
        assert_eq!(data, U256::from_dec_str("512003840009600008").unwrap(),);

        // neg
        let data = data.neg();
        assert_eq!(
            data,
            U256::max_value()
                .sub(U256::from_dec_str("512003840009600007").unwrap())
                .unwrap()
        );
    }
}
