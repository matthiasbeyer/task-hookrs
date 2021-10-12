//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module for wrapping chrono::naive::datetime::NaiveDateTime

use std::ops::{Deref, DerefMut};

use chrono::NaiveDateTime;
use serde::de::Error as SerdeError;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

/// Date is a NaiveDateTime-Wrapper object to be able to implement foreign traits on it
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Date(NaiveDateTime);

impl Deref for Date {
    type Target = NaiveDateTime;

    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}

impl DerefMut for Date {
    fn deref_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.0
    }
}

impl From<NaiveDateTime> for Date {
    fn from(ndt: NaiveDateTime) -> Date {
        Date(ndt)
    }
}

/// The date-time parsing template used to parse the date time data exported by taskwarrior.
pub static TASKWARRIOR_DATETIME_TEMPLATE: &'static str = "%Y%m%dT%H%M%SZ";

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = self.0.format(TASKWARRIOR_DATETIME_TEMPLATE);
        serializer.serialize_str(&format!("{}", formatted))
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'v> Visitor<'v> for DateVisitor {
            type Value = Date;

            fn expecting(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(
                    fmt,
                    "a string which matches {}",
                    TASKWARRIOR_DATETIME_TEMPLATE
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Date, E>
            where
                E: SerdeError,
            {
                NaiveDateTime::parse_from_str(value, TASKWARRIOR_DATETIME_TEMPLATE)
                    .map(|d| Date(d))
                    .map_err(|e| SerdeError::custom(e.to_string()))
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}
