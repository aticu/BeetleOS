//! The UEFI specific parts of the BeetleOS kernel.
//!
//! This gets invoked, when the kernel is loaded directly by UEFI.

use super::{BootMethod, BOOT_METHOD};
use core::fmt::{self, Write};
use crate::main;
use efi::{
    types::{EfiRt, Handle},
    SystemTable,
};

/// A reference to the UEFI system table.
static mut SYSTEM_TABLE: Option<EfiRt<SystemTable>> = None;

/// The entry point for UEFI applications.
pub fn uefi_start(_image_handle: Handle, system_table: EfiRt<SystemTable>) -> ! {
    unsafe {
        SYSTEM_TABLE.get_or_insert(system_table);
        BOOT_METHOD.get_or_insert(BootMethod::UEFI);
    }

    main();
}

/// Writes the formatted string.
pub(super) fn write_fmt(args: fmt::Arguments) {
    (&*get_system_table().con_out)
        .write_fmt(args)
        .expect("Could not output to UEFI output.")
}

/// Returns a reference to the system table.
fn get_system_table() -> &'static SystemTable {
    unsafe {
        SYSTEM_TABLE
            .as_ref()
            .expect("Could not read UEFI system table.")
    }
}
