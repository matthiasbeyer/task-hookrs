//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Error module, containing error types
error_chain!{
    errors {
        /// Error kind indicating that the JSON parser failed
        ParserError {}
        /// Error kind indicating that the Reader failed to read something
        ReaderError {}
    }
}
