//! This binary is what starts the actual kernel on x86_64.
//!
//! Most of the actual kernel code can be found in the library.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{arch::x86_64::uefi::uefi_init, main, println};
use nuefil::{system::SystemTable, Handle};

/// The entry point for the UEFI loader.
///
/// This is the first function that gets called by the UEFI firmware.
#[no_mangle]
pub extern "C" fn efi_main(image_handle: Handle, system_table: &'static SystemTable) -> ! {
    uefi_init(image_handle, system_table);

    main();
}

/// The panic implementation of BeetleOS.
#[panic_handler]
fn panic_fmt(panic_info: &PanicInfo) -> ! {
    log::error!("Panic:");

    println!("{}", panic_info);

    loop {}
}
