//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! This crate exports functionality to import and export taskwarrior-compatible JSON by
//! translating the JSON into rust types and vice-versa.
//!
//! For example:
//!
//! ```
//!   use std::io::stdin;
//!
//!   use task_hookrs::task::Task;
//!   use task_hookrs::import::import;
//!
//!   if let Ok(tasks) = import(stdin()) {
//!       for task in tasks {
//!           println!("Task: {}, entered {:?} is {} -> {}",
//!                     task.uuid(),
//!                     task.entry(),
//!                     task.status(),
//!                     task.description());
//!       }
//!   }
//! ```
//!
#![deny(missing_docs)]
#![doc(html_root_url = "https://matthiasbeyer.github.io/task-hookrs/")]
#![deny(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true
)]

extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;

#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate log;

pub mod annotation;
pub mod date;
pub mod error;
pub mod import;
pub mod priority;
pub mod project;
pub mod status;
pub mod tag;
pub mod task;
pub mod tw;
pub mod uda;
pub mod urgency;
