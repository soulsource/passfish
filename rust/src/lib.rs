extern crate libc;
extern crate serde;
extern crate dirs;
extern crate mockall_double;

#[cfg(test)]
extern crate mockall;

#[allow(clippy::all)]
pub mod interface;
#[warn(clippy::pedantic)]
mod implementation;
