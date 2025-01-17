use ethers::prelude::*;
use ff::PrimeField;
use num_bigint::BigUint;
use num_traits::{Euclid, Num};
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use std::str::FromStr;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct Fp([u64; 4]);

impl Into<U256> for Fp {
    fn into(self) -> U256 {
        U256::from_str_radix(
            &BigUint::from_bytes_le(self.to_repr().as_ref()).to_str_radix(16),
            16,
        )
        .unwrap()
    }
}

impl TryFrom<U256> for Fp {
    type Error = eyre::Report;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        Fp::from_str_vartime(&value.to_string()).ok_or(eyre::Report::msg("Invalid U256 value!"))
    }
}

impl FromStr for Fp {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fp::from_str_vartime(s).ok_or(eyre::Report::msg("Invalid Fp!"))?)
    }
}

impl Serialize for Fp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num = BigUint::from_bytes_le(self.to_repr().as_ref()).to_string();
        serializer.serialize_str(&num.to_string())
    }
}

impl Fp {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, eyre::Report> {
        Ok(Fp::from_str(
            &BigUint::from_bytes_le(bytes)
                .rem_euclid(&BigUint::from_str_radix(&Fp::MODULUS[2..], 16).unwrap())
                .to_string(),
        )
        .unwrap())
    }
}

struct FpStr;

impl<'de> Deserialize<'de> for Fp {
    fn deserialize<D>(deserializer: D) -> Result<Fp, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FpStr)
    }
}

impl<'de> Visitor<'de> for FpStr {
    type Value = Fp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "expecting a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Fp::from_str_vartime(s).ok_or(de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}
