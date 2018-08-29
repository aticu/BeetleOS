//! The x86_64 specific part of the BeetleOS kernel.
//!
//! This abstracts the details of the x86_64 platform.

pub mod uefi;

use core::fmt;

/// The types of methods that the system can be booted with.
enum BootMethod {
    /// The system was booted directly by the UEFI firmware.
    UEFI,
}

/// The method used to boot the system.
static mut BOOT_METHOD: Option<BootMethod> = None;

/// Writes the formatted string.
pub fn write_fmt(args: fmt::Arguments) {
    match unsafe { BOOT_METHOD.as_ref().unwrap() } {
        BootMethod::UEFI => uefi::write_fmt(args),
    }
}
