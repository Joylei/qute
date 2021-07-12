#![allow(unused)]
#![feature(llvm_asm)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

pub(crate) mod feature;
pub(crate) mod ffi;
pub(crate) mod hal;
pub mod platform;
pub(crate) mod types;
pub(crate) mod util;

use std::io;
#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    SemError(String),
    #[error("not supported platform, only for ITE8528")]
    PlatformNotSupport,
    #[error("{0}")]
    InvalidValue(String),
    #[error("{0}")]
    Timeout(String),
}

pub type Result<T> = std::result::Result<T, Error>;

//re-export
pub use feature::*;
pub use types::*;
