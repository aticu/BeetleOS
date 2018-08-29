#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use efi::{
    types::{EfiRt, Handle},
    SystemTable,
};
use kernel::{arch::x86_64::uefi::uefi_start, println};

/// The entry point for the UEFI loader.
///
/// This is the first function that gets called by the UEFI firmware.
#[no_mangle]
pub extern "C" fn efi_main(image_handle: Handle, system_table: EfiRt<SystemTable>) -> ! {
    uefi_start(image_handle, system_table);
}

/// The panic implementation of BeetleOS.
#[panic_implementation]
fn panic_fmt(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);

    loop {}
}
