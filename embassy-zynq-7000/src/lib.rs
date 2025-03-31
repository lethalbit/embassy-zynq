#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "dual-core", feature = "single-core"))]
compile_error!("Enabling both dual-core and single-core makes no sense");
