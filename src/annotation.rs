//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing types and functions for annotations of tasks

use date::Date;

/// Annotation type for task annotations.
/// Each annotation in taskwarrior consists of a date and a description,
/// the date is named "entry", the description "description" in the JSON export.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod test {
}
