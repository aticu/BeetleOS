//! This is the code for the BeetleOS kernel.
//!
//! It is a µ-kernel implemented in Rust.

#![no_std]

#[macro_use]
pub mod arch;
pub mod memory;
pub mod sync;

/// Sets the log level for the kernel.
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

use crate::arch::{Arch, Architecture};

/// The main function for the kernel.
///
/// This is called by the architecture specific code after initialization.
///
/// It then continues to initialize the kernel and finally enters user mode.
pub fn main() -> ! {
    // First disable interrupts, if it didn't previously happen.
    // They will be restored after this function.
    Arch::disable_interrupts();

    log::debug!("Reached the main function.");

    Arch::enable_interrupts();

    loop {}
}
