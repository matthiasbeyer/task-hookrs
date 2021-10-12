//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! This module offers functions to interact with taskwarrior. This will expect the `task` binary
//! in your path. This will always call task and never interact with your `.task` directory itself.
//! (This is in accordance with the taskwarrior api guide lines.)

use error::ErrorKind as EK;
use import::import;
use std::io::Write;
use std::iter::once;
use std::process::{Child, Command, Stdio};
use task::Task;

use failure::Fallible as Result;
use failure::ResultExt;
use serde_json;

/// This will give you all tasks which match the given query in the taskwarrior query syntax.
/// This is not sanitized. Never get the query string from an untrusted user.
pub fn query(query: &str) -> Result<Vec<Task>> {
    let mut cmd = add_query_to_cmd(query, Command::new("task"));
    cmd.stdout(Stdio::piped());
    run_query_cmd(cmd)
}

/// This will take a Command, and append the given query string splited at whitespace followed by
/// the "export" command to the arguments of the Command.
pub fn add_query_to_cmd(query: &str, mut cmd: Command) -> Command {
    for x in query.split_whitespace().chain(once("export")) {
        cmd.arg(x);
    }
    cmd
}

/// This executes the given Command and trys to convert the Result into a Vec<Task>.
pub fn run_query_cmd(mut cmd: Command) -> Result<Vec<Task>> {
    let mut export = cmd.spawn().context(EK::TaskCmdError)?;
    export.wait().context(EK::TaskCmdError)?;
    import(export.stdout.ok_or(EK::TaskCmdError)?)
}

/// This function runs the given Command, pipes the tasks as JSON to it and returns a handle to the child process.
pub fn save_to_cmd<'a>(tasks: Vec<&'a Task>, mut cmd: Command) -> Result<Child> {
    let input_buffer = serde_json::to_string(&tasks).context(EK::SerializeError)?;
    let mut import = cmd.spawn().context(EK::TaskCmdError)?;
    import
        .stdin
        .as_mut()
        .ok_or(EK::TaskCmdError)?
        .write_all(input_buffer.as_bytes())
        .context(EK::TaskCmdError)?;
    Ok(import)
}

/// This will save the given tasks to taskwarrior. Call with `Some(&task)` if you just have one
/// task.
/// This will block until the save was successful.
pub fn save<'a, T>(tasks: T) -> Result<()>
where
    T: IntoIterator<Item = &'a Task>,
{
    save_async(tasks)?.wait().context(EK::TaskCmdError)?;
    Ok(())
}

/// This function returns the handle to a child process which saves the given tasks.
pub fn save_async<'a, T>(tasks: T) -> Result<Child>
where
    T: IntoIterator<Item = &'a Task>,
{
    let mut cmd = Command::new("task");
    cmd.arg("import").stdin(Stdio::piped());
    save_to_cmd(tasks.into_iter().collect(), cmd)
}
