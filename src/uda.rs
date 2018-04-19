//! Module containing the types for User Defined Attributes (UDA)

use std::ops::{Deref, DerefMut};
use std::default::Default;
use std::collections::BTreeMap;
use std::result::Result as RResult;
use std::fmt;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de;

/// UDA Name
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UDAName(String);

impl<'a> From<&'a str> for UDAName {
    fn from(s: &str) -> UDAName {
        UDAName(String::from(s))
    }
}
impl From<String> for UDAName {
    fn from(s: String) -> UDAName {
        UDAName(s)
    }
}

impl Serialize for UDAName {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UDAName {
    fn deserialize<D>(deserializer: D) -> RResult<UDAName, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| UDAName(s))
    }
}

/// UDA Value
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum UDAValue {
    /// UDA is a string
    Str(String),
    /// UDA is an integer
    U64(u64),
    /// UDA is a float
    F64(f64),
}

impl Serialize for UDAValue {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &UDAValue::Str(ref s) => s.serialize(serializer),
            &UDAValue::U64(s) => s.serialize(serializer),
            &UDAValue::F64(s) => s.serialize(serializer),
        }
    }
}

struct UDAVisitor;

impl<'de> Visitor<'de> for UDAVisitor {
    type Value = UDAValue;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an UDA value like a string, float or int")
    }

    fn visit_u64<E>(self, value: u64) -> Result<UDAValue, E>
    where
        E: de::Error,
    {
        Ok(UDAValue::U64(value))
    }
    fn visit_f64<E>(self, value: f64) -> Result<UDAValue, E>
    where
        E: de::Error,
    {
        Ok(UDAValue::F64(value))
    }
    fn visit_str<E>(self, value: &str) -> Result<UDAValue, E>
    where
        E: de::Error,
    {
        Ok(UDAValue::Str(value.to_owned()))
    }
}

impl<'de> Deserialize<'de> for UDAValue {
    fn deserialize<D>(deserializer: D) -> RResult<UDAValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UDAVisitor)
    }
}

/// Wrapper type for BTreeMap<UDAName, UDAValue> so serde does not automatically implement the
/// Deserialize trait on it.
#[derive(Clone, Debug)]
pub struct UDA(BTreeMap<UDAName, UDAValue>);

impl Deref for UDA {
    type Target = BTreeMap<UDAName, UDAValue>;

    fn deref(&self) -> &BTreeMap<UDAName, UDAValue> {
        &self.0
    }
}

impl DerefMut for UDA {
    fn deref_mut(&mut self) -> &mut BTreeMap<UDAName, UDAValue> {
        &mut self.0
    }
}

impl Default for UDA {
    fn default() -> UDA {
        UDA(BTreeMap::new())
    }
}

impl Serialize for UDA {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UDA {
    fn deserialize<D>(deserializer: D) -> RResult<UDA, D::Error>
    where
        D: Deserializer<'de>,
    {
        BTreeMap::deserialize(deserializer).map(|btm| UDA(btm))
    }
}
