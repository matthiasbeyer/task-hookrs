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
    while_true,
)]

extern crate chrono;
#[macro_use] extern crate log;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub mod annotation;
pub mod date;
pub mod error;
pub mod import;
pub mod priority;
pub mod project;
pub mod result;
pub mod status;
pub mod tag;
pub mod task;

