#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod data;
mod library;
mod lua;
#[allow(improper_ctypes, dead_code)]
mod lua_core;
mod mtype;
mod stack;

use alloc::string::String;
pub use data::*;
pub use library::*;
pub use lua::*;
pub use lua_core::{Int, State};
pub use mtype::*;
pub use stack::*;

/// Various errors that may be returned.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Runtime(String),
    Data(DataErr),
    Library(LibraryErr),
}
impl From<LibraryErr> for Error {
    fn from(e: LibraryErr) -> Self {
        Error::Library(e)
    }
}
impl From<DataErr> for Error {
    fn from(e: DataErr) -> Self {
        Error::Data(e)
    }
}
