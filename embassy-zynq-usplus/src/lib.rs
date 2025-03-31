#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "a", feature = "r"))]
compile_error!("Targeting both the Cortex-A and Cortex-R cores make no sense");
