//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing TaskPriority types and trait impls

/// Enum for the priorities taskwarrior supports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low prio for a Task
    #[serde(rename = "L")]
    Low,

    /// Medium prio for a Task
    #[serde(rename = "M")]
    Medium,

    /// High prio for a Task
    #[serde(rename = "H")]
    High,
}
