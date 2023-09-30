use serde::{
    de::{Error, Visitor},
    Deserializer, Serializer,
};
use std::{fmt, num::ParseIntError};

#[inline]
pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format(*num))
}

fn format(num: u128) -> String {
    format!("{:0>32X}", num)
}

#[inline]
pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(HexVisitor)
}

struct HexVisitor;

impl HexVisitor {
    fn parse(string: &str) -> Result<u128, ParseIntError> {
        u128::from_str_radix(string, 16)
    }

    fn try_parse<E>(string: &str) -> Result<u128, E>
    where
        E: Error,
    {
        Self::parse(string).map_err(|err| E::custom(format!("{}: \"{}\"", err, string)))
    }

    fn try_<T, E>(value: T) -> Result<u128, E>
    where
        T: Into<u128>,
    {
        Ok(value.into())
    }
}

impl Visitor<'_> for HexVisitor {
    type Value = u128;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or a hexadecimal string")
    }

    #[inline]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_(v)
    }

    #[inline]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_(v)
    }

    #[inline]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_(v)
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_(v)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_parse(v)
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::try_parse(&v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let num = 0x1B56F702912BE7428182CA57036AEE99;
        let string = format(num);

        assert_eq!(HexVisitor::parse(&string), Ok(num));
    }
}
