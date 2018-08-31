//! This is the code for the BeetleOS kernel.
//!
//! It is a µ-kernel implemented in Rust.

#![feature(const_fn)]
#![no_std]

#[macro_use]
pub mod arch;
pub mod sync;

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

    println!("Hello {} from {}!", "UEFI", "BeetleOS");
    print!("Test: ");
    println!("Still on the same line!");
    serial_println!("This is the serial output.");

    Arch::enable_interrupts();

    loop {}
}
