//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Definitions for error handling with failure

/// Failure error kind type, defining error messages
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error kind indicating that the JSON parser failed
    #[error("Failed to create a Task from JSON")]
    ParserError,

    /// Error kind indicating that the Reader failed to read something
    #[error("Failed to read tasks from a Reader")]
    ReaderError,

    /// Error kind indicating that a call to the task warrior binary failed
    #[error("There was a problem while calling the external 'task' binary")]
    TaskCmdError,

    /// Error kind indicating that a conversion to JSON failed
    #[error("A Task could not be converted to JSON")]
    SerializeError,

    /// Error wrapper for std::io::Error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error wrapper for serde_json::Error
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
