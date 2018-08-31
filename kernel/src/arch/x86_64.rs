//! The x86_64 specific part of the BeetleOS kernel.
//!
//! This abstracts the details of the x86_64 platform.

pub mod uefi;
pub mod serial;

use core::fmt;
use x86_64::instructions::interrupts;

use super::Architecture;

/// The types of methods that the system can be booted with.
enum BootMethod {
    /// The system was booted directly by the UEFI firmware.
    UEFI,
}

/// The method used to boot the system.
static mut BOOT_METHOD: Option<BootMethod> = None;

/// Returns the current boot method.
fn get_boot_method() -> &'static BootMethod {
    // This is safe under the assumption that the boot method will be set early on (before this is called) and never changed.
    unsafe {
        BOOT_METHOD
            .as_ref()
            .expect("Could not read boot method.")
    }
}

/// The struct that implements the architecture trait and repressents this architecture.
#[allow(non_camel_case_types)]
pub struct x86_64;

impl Architecture for x86_64 {
    fn write_fmt(args: fmt::Arguments) {
        match get_boot_method() {
            BootMethod::UEFI => uefi::write_fmt(args),
        }
    }

    fn write_line_break() {
        match get_boot_method() {
            BootMethod::UEFI => uefi::write_fmt(format_args!("\r\n")),
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