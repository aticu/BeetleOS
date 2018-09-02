//! The x86_64 specific part of the BeetleOS kernel.
//!
//! This abstracts the details of the x86_64 platform.

pub mod uefi;
#[macro_use]
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
    unsafe { BOOT_METHOD.as_ref().expect("Could not read boot method.") }
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

/// Exits qemu during an integration test.
#[cfg(all(feature = "integration_test", feature = "qemu"))]
pub fn exit_integration_test(exit_code: IntegrationTestExitCode) -> ! {
    use x86_64::instructions::port::Port;

    let mut exit_port = Port::<u32>::new(0xf4);

    unsafe {
        match exit_code {
            IntegrationTestExitCode::Success => exit_port.write(0),
            IntegrationTestExitCode::Failure(message) => {
                serial_print!("{}", message);
                exit_port.write(1)
            }
        }
    }

    unreachable!("qemu should have exited now!");
}

/// The possible exit codes for integration tests.
#[cfg(feature = "integration_test")]
pub enum IntegrationTestExitCode {
    /// The test was successful.
    Success,
    /// The test failed.
    Failure(&'static str),
}
