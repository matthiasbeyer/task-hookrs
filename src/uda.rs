//! Module containing the types for User Defined Attributes (UDA)

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

