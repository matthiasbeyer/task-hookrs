//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Definitions for error handling with failure

/// Failure error kind type, defining error messages
#[derive(Debug, Clone, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    /// Error kind indicating that the JSON parser failed
    #[fail(display = "Failed to create a Task from JSON")]
    ParserError,

    /// Error kind indicating that the Reader failed to read something
    #[fail(display = "Failed to read tasks from a Reader")]
    ReaderError,

    /// Error kind indicating that a call to the task warrior binary failed
    #[fail(display = "There was a problem while calling the external 'task' binary")]
    TaskCmdError,

    /// Error kind indicating that a conversion to JSON failed
    #[fail(display = "A Task could not be converted to JSON")]
    SerializeError,
}
