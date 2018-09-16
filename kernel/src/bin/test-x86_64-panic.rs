//! This binary runs the panic test.
//!
//! This test makes sure that the system boots properly using UEFI.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{
    arch::x86_64::{exit_integration_test, uefi::uefi_init, IntegrationTestExitCode},
    serial_println,
};
use nuefil::{system::SystemTable, Handle};

/// The entry point for the UEFI loader.
///
/// This is the first function that gets called by the UEFI firmware.
#[no_mangle]
pub extern "C" fn efi_main(image_handle: Handle, system_table: &'static SystemTable) -> ! {
    uefi_init(image_handle, system_table);

    panic!();
}

/// The panic implementation of the panic test.
#[panic_handler]
fn panic_fmt(panic_info: &PanicInfo) -> ! {
    serial_println!("{}", panic_info);

    exit_integration_test(IntegrationTestExitCode::Success);
}
