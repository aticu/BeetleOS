//! The UEFI specific parts of the BeetleOS kernel.
//!
//! This gets invoked, when the kernel is loaded directly by UEFI.

use core::fmt::{self, Write};
use nuefil::{memory::NamedMemoryType, system::SystemTable, Handle};
use size_format::SizeFormatterBinary;

use super::{early_init, BootMethod, BOOT_METHOD};
use crate::sync::GlobalRuntimeConfiguration;

/// A reference to the UEFI system table.
static SYSTEM_TABLE: GlobalRuntimeConfiguration<&'static SystemTable> =
    GlobalRuntimeConfiguration::new();

/// The entry point for UEFI applications.
pub fn uefi_init(image_handle: Handle, system_table: &'static SystemTable) {
    SYSTEM_TABLE.init(system_table);
    BOOT_METHOD.init(BootMethod::UEFI);

    // Fail silently if the screen cannot be cleared.
    get_system_table().ConsoleOut.clear_screen().ok();

    early_init();

    log::info!("Exiting UEFI boot services...");

    let memory_map = get_system_table()
        .BootServices
        .exit_boot_services(image_handle)
        .expect("Could not exit UEFI boot services.");

    // The boot services are now disabled.

    let mut usable_pages = 0;
    let mut total_pages = 0;

    for entry in memory_map.iter() {
        if entry.Type == NamedMemoryType::ConventionalMemory.into() {
            usable_pages += entry.NumberOfPages;
        }
        total_pages += entry.NumberOfPages;
    }

    log::info!(
        "The usable amount of memory is {}B, the total amount of memory is {}B.",
        SizeFormatterBinary::new(usable_pages * 0x1000),
        SizeFormatterBinary::new(total_pages * 0x1000)
    );
}

/// Writes the formatted string.
pub(super) fn write_fmt(args: fmt::Arguments) {
    let mut console_out = &*get_system_table().ConsoleOut;

    if console_out as *const _ as usize != 0 {
        console_out
            .write_fmt(args)
            .expect("Could not output to UEFI output.");
    }
}

/// Returns a reference to the system table.
fn get_system_table() -> &'static SystemTable {
    SYSTEM_TABLE
        .get()
        .expect("Could not read UEFI system table.")
}
