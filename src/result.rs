//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing `Result` type

use std::result::Result as RResult;

use error::TaskError;

/// Result type wrapping the standard std::result::Result type
pub type Result<T> = RResult<T, TaskError>;
