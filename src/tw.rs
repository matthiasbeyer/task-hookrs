//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//


//! This module offers functions to interact with taskwarrior. This will expect the `task` binary
//! in your path. This will always call task and never interact with your `.task` directory itself.
//! (This is in accordance with the taskwarrior api guide lines.)

use error::{Result, ResultExt, ErrorKind as EK};
use task::Task;
use std::process::{Command, Stdio};
use std::io::Write;
use import::import;
use serde_json;

/// This will give you all tasks which match the given query in the taskwarrior query syntax.
/// This is not sanitized. Never get the query string from an untrusted user.
pub fn query(query: &str) -> Result<Vec<Task>> {
    let mut cmd = Command::new("task");
    for filter in query.split_whitespace() {
        cmd.arg(filter);
    }
    let mut export = cmd.arg("export").stdout(Stdio::piped()).spawn().chain_err(
        || {
            EK::TaskCmdError
        },
    )?;
    export.wait().chain_err(|| EK::TaskCmdError)?;
    import(export.stdout.chain_err(|| EK::TaskCmdError)?)
}

/// This will save the given tasks to taskwarrior. Call with `Some(task)` if you just have one
/// task.
pub fn save<T>(tasks: T) -> Result<()>
where
    T: IntoIterator<Item = Task>,
{
    let tasks: Vec<Task> = tasks.into_iter().collect();
    let input_buffer = serde_json::to_string(&tasks).chain_err(
        || EK::SerializeError,
    )?;
    let mut import = Command::new("task")
        .arg("import")
        .stdin(Stdio::piped())
        .spawn()
        .chain_err(|| EK::TaskCmdError)?;
    import
        .stdin
        .as_mut()
        .chain_err(|| EK::TaskCmdError)?
        .write_all(input_buffer.as_bytes())
        .chain_err(|| EK::TaskCmdError)?;
    import.wait().chain_err(|| EK::TaskCmdError)?;
    Ok(())
}
