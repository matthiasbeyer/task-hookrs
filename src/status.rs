//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing `TaskStatus` type and trait impls

use std::fmt::{Display, Formatter, Error as FmtError};

/// Enum for status taskwarrior supports.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TaskStatus {
    /// Pening status type
    Pending,

    /// Deleted status type
    Deleted,

    /// Completed status type
    Completed,

    /// Waiting status type
    Waiting,

    /// Recurring status type
    Recurring
}

impl Display for TaskStatus {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &TaskStatus::Pending   => write!(fmt, "Pending"),
            &TaskStatus::Deleted   => write!(fmt, "Deleted"),
            &TaskStatus::Completed => write!(fmt, "Completed"),
            &TaskStatus::Waiting   => write!(fmt, "Waiting"),
            &TaskStatus::Recurring => write!(fmt, "Recurring"),
        }
    }
}

