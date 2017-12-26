extern crate libc;
extern crate rvs;

mod rvsc;
mod error;

pub use rvsc::*;
pub use error::*;

#[cfg(test)] extern crate tempdir;
#[cfg(test)] #[macro_use(assert_diff)] extern crate difference;
#[cfg(test)] mod tests;
