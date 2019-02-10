//! This module exposes a type that implements the Architecture trait.

use core::fmt;

use x86_64_crate::instructions::interrupts;

use crate::arch::{
    x86_64::{get_boot_method, serial, uefi, BootMethod},
    Architecture,
};

/// The struct that implements the architecture trait and repressents this architecture.
#[allow(non_camel_case_types)]
pub struct x86_64;

impl Architecture for x86_64 {
    fn write_fmt(args: fmt::Arguments) {
        match get_boot_method() {
            BootMethod::UEFI => {
                uefi::write_fmt(args);
                serial::write_fmt(args);
            }
        }
    }

    fn write_line_break() {
        match get_boot_method() {
            BootMethod::UEFI => {
                uefi::write_fmt(format_args!("\r\n"));
                serial::write_fmt(format_args!("\n"));
            }
        }
    }

    fn interrupts_enabled() -> bool {
        interrupts::are_enabled()
    }

    fn enable_interrupts() {
        interrupts::enable()
    }

    fn disable_interrupts() {
        interrupts::disable()
    }
}
