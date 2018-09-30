//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Error module, containing error types
error_chain!{
    errors {
        /// Error kind indicating that the JSON parser failed
        ParserError {
            description("Failed to create a Task from JSON")
        }
        /// Error kind indicating that the Reader failed to read something
        ReaderError {
            description("Failed to read tasks from a Reader")
        }
        /// Error kind indicating that a call to the task warrior binary failed
        TaskCmdError {
            description("There was a problem while calling the external 'task' binary")
        }
        /// Error kind indicating that a conversion to JSON failed
        SerializeError {
            description("A Task could not be converted to JSON")
        }
    }
}
