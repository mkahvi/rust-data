//! Ini concepts.

#![allow(dead_code)]

pub(crate) mod core;

mod config;
/// INI file handling configuration.
pub use config::Config as Config;

mod section;
/// INI file [section] handling.
pub use section::Section as Section;

mod keyvalue;
/// INI file key=value handling.
pub use keyvalue::KeyValue as KeyValue;

mod constants;
/// Constants
pub use constants::*;
