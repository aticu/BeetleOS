//! This is the code for the BeetleOS kernel.
//!
//! It is a µ-kernel implemented in Rust.

#![no_std]

pub mod arch;

/// The main function for the kernel.
///
/// This is called by the architecture specific code after initialization.
///
/// It then continues to initialize the kernel and finally enters user mode.
pub fn main() -> ! {
    println!("Hello {} from {}!", "UEFI", "BeetleOS");
    print!("Test: ");
    println!("Still on the same line!");

    loop {}
}
