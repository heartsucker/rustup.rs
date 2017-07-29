#![recursion_limit = "1024"]

extern crate rustup_dist;
extern crate rustup_utils;
#[macro_use]
extern crate error_chain;
extern crate url;
extern crate regex;
extern crate itertools;
extern crate rustc_serialize;
extern crate tempfile;
extern crate time;
extern crate toml;
extern crate tuf;
#[cfg(unix)]
extern crate libc;

pub use errors::*;
pub use notifications::*;
pub use config::*;
pub use toolchain::*;
pub use rustup_utils::{utils, notify, toml_utils};

pub const ROOT_KEYS: &'static [&str] = &[
    "qfrfBrkB4lBBSDEBlZgaTGS_SrE6UfmON9kP4i3dJFY=" // TODO This is a dummy key for testing
];

mod errors;
mod notifications;
mod toolchain;
mod config;
mod install;
pub mod settings;
pub mod telemetry;
pub mod command;
pub mod telemetry_analysis;
pub mod env_var;
