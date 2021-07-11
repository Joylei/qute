#![allow(unused)]
#![feature(llvm_asm)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
pub(crate) mod feature;
pub(crate) mod ffi;
pub(crate) mod hal;
pub mod platform;
pub(crate) mod types;
pub(crate) mod util;

pub mod errors {
    error_chain! {}
}

//re-export
pub use feature::*;
pub use types::*;
