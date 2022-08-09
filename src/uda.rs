//! Module containing the types for User Defined Attributes (UDA)

use std::collections::BTreeMap;
use std::fmt;
use std::result::Result as RResult;

use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

/// The name of a UDA is just a string.
pub type UDAName = String;

/// A UDA can have different value types.
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
            UDAValue::Str(ref s) => s.serialize(serializer),
            UDAValue::U64(s) => s.serialize(serializer),
            UDAValue::F64(s) => s.serialize(serializer),
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

/// The UDA Type is just a BTreeMap<UDAName, UDAValue> in which all fields of a task are saved,
/// which are not part of the taskwarrior standard. (This makes them user defined attributes.)
pub type UDA = BTreeMap<UDAName, UDAValue>;
