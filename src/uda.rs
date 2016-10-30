//! Module containing the types for User Defined Attributes (UDA)

use std::ops::{Deref, DerefMut};
use std::default::Default;
use std::collections::BTreeMap;

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

/// UDA Value
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UDAValue(String);

impl<'a> From<&'a str> for UDAValue {

    fn from(s: &str) -> UDAValue {
        UDAValue(String::from(s))
    }

}

impl From<String> for UDAValue {

    fn from(s: String) -> UDAValue {
        UDAValue(s)
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

