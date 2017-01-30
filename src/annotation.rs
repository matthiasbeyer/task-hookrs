//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing types and functions for annotations of tasks

use std::result::Result as RResult;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::MapVisitor as DeserializeMapVisitor;

use date::Date;

/// Annotation type for task annotations.
/// Each annotation in taskwarrior consists of a date and a description,
/// the date is named "entry", the description "description" in the JSON export.
#[derive(Clone, Debug)]
pub struct Annotation {
    entry: Date,
    description: String
}

impl Annotation {

    /// Create a new Annotation object
    pub fn new(entry: Date, description: String) -> Annotation {
        Annotation {
            entry: entry,
            description: description,
        }
    }

    /// Get the entry date
    pub fn entry(&self) -> &Date {
        &self.entry
    }

    /// Get the description text
    pub fn description(&self) -> &String {
        &self.description
    }

}

impl Serialize for Annotation {

    fn serialize<S>(&self, serializer: &mut S) -> RResult<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("Annotation", 2));
        try!(serializer.serialize_struct_elt(&mut state, "entry", &self.entry));
        try!(serializer.serialize_struct_elt(&mut state, "description", &self.description));
        serializer.serialize_struct_end(state)
    }

}

impl Deserialize for Annotation {

    fn deserialize<D>(deserializer: &mut D) -> RResult<Annotation, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &[
            "entry",
            "description"
        ];

        deserializer.deserialize_struct("Annotation", FIELDS, AnnotationDeserializeVisitor)
    }

}

/// Helper type for the `Deserialize` implementation
struct AnnotationDeserializeVisitor;

impl Visitor for AnnotationDeserializeVisitor {
    type Value = Annotation;

    fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "a taskwarrior annotation object")
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> RResult<Annotation, V::Error>
        where V: DeserializeMapVisitor
    {
        let mut entry       = None;
        let mut description = None;

        loop {
            let key : Option<String> = try!(visitor.visit_key());
            if key.is_none() {
                break;
            }
            let key = key.unwrap();

            match &key[..] {
                "entry" => {
                    entry = Some(try!(visitor.visit_value()));
                },
                "description" => {
                    description = Some(try!(visitor.visit_value()));
                },

                field => {
                    use serde::de::impls::IgnoredAny;

                    debug!("field '{}' ignored", field);
                    let _: IgnoredAny = try!(visitor.visit_value());
                }
            }
        }

        let entry = match entry {
            Some(entry) => entry,
            None => try!(visitor.missing_field("entry")),
        };

        let description = match description {
            Some(description) => description,
            None => try!(visitor.missing_field("description")),
        };

        try!(visitor.end());

        Ok(Annotation::new(entry, description))
    }
}

#[cfg(test)]
mod test {
}
