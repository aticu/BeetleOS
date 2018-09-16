//! The UEFI specific parts of the BeetleOS kernel.
//!
//! This gets invoked, when the kernel is loaded directly by UEFI.

use core::fmt::{self, Write};
use nuefil::{system::SystemTable, Handle};

use super::{early_init, BootMethod, BOOT_METHOD};

/// A reference to the UEFI system table.
static mut SYSTEM_TABLE: Option<&'static SystemTable> = None;

/// The entry point for UEFI applications.
pub fn uefi_init(image_handle: Handle, system_table: &'static SystemTable) {
    // This access is safe, because this code is run in isolation as the first code that runs.
    // It is also the only function where those values are changed.
    unsafe {
        SYSTEM_TABLE.get_or_insert(system_table);
        BOOT_METHOD.get_or_insert(BootMethod::UEFI(Status::BootServicesActive));
    }

    // Fail silently if the screen cannot be cleared.
    let _ = get_system_table().ConsoleOut.clear_screen();

    early_init();

    log::info!("Exiting UEFI boot services...");

    let memory_map = get_system_table()
        .BootServices
        .exit_boot_services(image_handle)
        .expect("Could not get UEFI memory map.");

    // This access is safe, because this code is run in isolation as the first code that runs.
    // It is also the only function where this value is changed.
    unsafe {
        *BOOT_METHOD.as_mut().unwrap() = BootMethod::UEFI(Status::BootServicesInactive);
    }

    let mut usable_pages = 0;
    let mut total_pages = 0;

    for entry in memory_map.iter() {
        if entry.Type == nuefil::memory::MemoryType::ConventionalMemory {
            usable_pages += entry.NumberOfPages;
        }
        total_pages += entry.NumberOfPages;
    }

    log::info!("The usable amount of memory is {}MiB, the total amount of memory is {}MiB.", usable_pages / 256, total_pages / 256);
}

/// Writes the formatted string.
pub(super) fn write_fmt(args: fmt::Arguments) {
    (&*get_system_table().ConsoleOut)
        .write_fmt(args)
        .expect("Could not output to UEFI output.")
}

/// Returns a reference to the system table.
fn get_system_table() -> &'static SystemTable {
    // This is safe under the assumption that the system table will be set early on (before this is called) and never changed.
    unsafe {
        SYSTEM_TABLE
            .as_mut()
            .expect("Could not read UEFI system table.")
    }
}

/// Represents the statuses the UEFI kernel can be in.
pub(super) enum Status {
    /// The boot services are still active.
    BootServicesActive,
    /// The boot services are inactive.
    BootServicesInactive,
}
