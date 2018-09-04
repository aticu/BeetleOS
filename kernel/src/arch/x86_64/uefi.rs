//! The UEFI specific parts of the BeetleOS kernel.
//!
//! This gets invoked, when the kernel is loaded directly by UEFI.

use core::fmt::{self, Write};
use efi::{
    types::{EfiRt, Handle},
    SystemTable,
};

use super::{early_init, BootMethod, BOOT_METHOD};
use crate::main;

/// A reference to the UEFI system table.
static mut SYSTEM_TABLE: Option<EfiRt<SystemTable>> = None;

/// The entry point for UEFI applications.
pub fn uefi_start(_image_handle: Handle, system_table: EfiRt<SystemTable>) -> ! {
    // This access is safe, because this code is run in isolation as the first code that runs.
    // It is also the only place where those values are changed.
    unsafe {
        SYSTEM_TABLE.get_or_insert(system_table);
        BOOT_METHOD.get_or_insert(BootMethod::UEFI);
    }
    early_init();

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
    // This is safe under the assumption that the system table will be set early on (before this is called) and never changed.
    unsafe {
        SYSTEM_TABLE
            .as_ref()
            .expect("Could not read UEFI system table.")
    }
}
