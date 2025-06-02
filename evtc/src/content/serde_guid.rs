use crate::content::{GuidExt, GUID};
use serde::{
    de::{Error, Visitor},
    Deserializer, Serializer,
};
use std::fmt;

#[inline]
pub fn serialize<S>(guid: &GUID, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&guid.format_hyphenated())
}

#[inline]
pub fn deserialize<'de, D>(deserializer: D) -> Result<GUID, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(GuidVisitor)
}

struct GuidVisitor;

impl GuidVisitor {
    fn parse(string: &str) -> Option<GUID> {
        GUID::try_from(string)
            .ok()
            .or_else(|| u128::from_str_radix(string, 16).ok().map(GUID::from_u128))
    }

    fn deserialize_parse<E>(string: &str) -> Result<GUID, E>
    where
        E: Error,
    {
        Self::parse(string).ok_or_else(|| E::custom("invalid guid"))
    }

    fn deserialize_convert<T, E>(value: T) -> Result<GUID, E>
    where
        T: Into<u128>,
    {
        Ok(GUID::from_u128(value.into()))
    }
}

impl Visitor<'_> for GuidVisitor {
    type Value = GUID;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or a hexadecimal string")
    }

    #[inline]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_convert(v)
    }

    #[inline]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_convert(v)
    }

    #[inline]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_convert(v)
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_convert(v)
    }

    #[inline]
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_convert(v)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_parse(v)
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::deserialize_parse(&v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GUID: GUID = GUID::from_u128(0x1B56F702912BE7428182CA57036AEE99);

    #[test]
    fn parse() {
        assert_eq!(
            GuidVisitor::parse("1B56F702912BE7428182CA57036AEE99"),
            Some(TEST_GUID)
        );
        assert_eq!(
            GuidVisitor::parse("1B56F702-912B-E742-8182-CA57036AEE99"),
            Some(TEST_GUID)
        );
    }

    #[test]
    fn roundtrip() {
        let string = TEST_GUID.format_hyphenated();
        assert_eq!(GuidVisitor::parse(&string), Some(TEST_GUID));
    }
}
