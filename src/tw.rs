//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//


//! This module offers functions to interact with taskwarrior. This will expect the `task` binary
//! in your path. This will always call task and never interact with your `.task` directory itself.
//! (This is in accordance with the taskwarrior api guide lines.)

use error::{Result, ResultExt};
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
        || "Failed to start task … export.",
    )?;
    export.wait().chain_err(
        || "Failed to wait for task … export to finish",
    )?;
    import(export.stdout.chain_err(
        || "Failed to capture stdout of task … export",
    )?)
}

/// This will save the given tasks to taskwarrior. Call with `Some(task)` if you just have one
/// task.
pub fn save<T>(tasks: T) -> Result<()>
where
    T: IntoIterator<Item = Task>,
{
    let import = Command::new("task")
        .arg("import")
        .stdin(Stdio::piped())
        .spawn()
        .chain_err(|| "Failed to spawn task … import")?;
    import
        .stdin
        .chain_err(|| "Failed to grap stdin of task import")?
        .write_all(
            serde_json::to_string(&tasks.into_iter().collect::<Vec<_>>())
                .chain_err(|| "Failed to Serialize")?
                .as_bytes(),
        )
        .chain_err(
            || "Failed to write serialized tasks to stdin of task import",
        )?;
    Ok(())
}
