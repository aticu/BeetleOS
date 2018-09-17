//! The x86_64 specific part of the BeetleOS kernel.
//!
//! This abstracts the details of the x86_64 platform.

mod architecture_implementation;
#[macro_use]
pub mod serial;
mod logger;
pub mod uefi;

pub use self::architecture_implementation::x86_64;

use atomic::{Atomic, Ordering};
use raw_cpuid::CpuId;
use ::x86_64::registers::{
    control::{Cr0, Cr0Flags},
    model_specific::{Efer, EferFlags},
};

/// The types of methods that the system can be booted with.
#[derive(Clone, Copy)]
enum BootMethod {
    /// The system was booted directly by the UEFI firmware.
    UEFI(uefi::Status),
}

/// The method used to boot the system.
static BOOT_METHOD: Atomic<Option<BootMethod>> = Atomic::new(None);

/// Returns the current boot method.
fn get_boot_method() -> BootMethod {
    BOOT_METHOD
        .load(Ordering::SeqCst)
        .expect("Could not read boot method.")
}

/// Performs early initialization for the x86_64 architecture.
fn early_init() {
    // Initialize the logger. If initialization fails, logging won't work.
    match log::set_logger(&logger::KERNEL_LOGGER) {
        _ => (),
    }
    log::set_max_level(crate::LOG_LEVEL);

    // Check that the CPU supports the necessary features.
    let cpuid = CpuId::new();

    let mut compatible = true;

    if let Some(features) = cpuid.get_feature_info() {
        if !features.has_apic() {
            log::error!("No APIC found.");
            compatible = false;
        }
    } else {
        log::error!("No APIC found.");
        compatible = false;
    }

    if let Some(features) = cpuid.get_extended_function_info() {
        if !features.has_syscall_sysret() {
            log::error!("Syscall/sysret not supported.");
            compatible = false;
        }
        if !features.has_execute_disable() {
            log::error!("NXE bit not supported.");
            compatible = false;
        }
    } else {
        log::error!("Syscall/sysret not supported.");
        log::error!("NXE bit not supported.");
        compatible = false;
    }

    if compatible {
        log::debug!("The CPU is compatible with BeetleOS.")
    } else {
        panic!("Your computer is not compatible with BeetleOS.");
    }

    if let Some(vendor_info) = cpuid.get_vendor_info() {
        if let Some(features) = cpuid.get_extended_function_info() {
            if let Some(model_name) = features.processor_brand_string() {
                log::info!(
                    "Starting BeetleOS on a {} CPU (Model: {}).",
                    vendor_info,
                    model_name.trim()
                );
            }
        }
    }

    // Enable the required features
    // This is safe, because only some features are enabled and they don't corrupt memory safety when enabling them at this stage
    unsafe {
        // Enable syscall/sysret and the NXE bit
        Efer::update(|flags| {
            flags.insert(EferFlags::SYSTEM_CALL_EXTENSIONS | EferFlags::NO_EXECUTE_ENABLE);
        });

        // Disable writing to read-only pages from kernel
        Cr0::update(|flags| flags.insert(Cr0Flags::WRITE_PROTECT));
    }
}

/// Exits qemu during an integration test.
#[cfg(feature = "qemu_integration_test")]
pub fn exit_integration_test(exit_code: IntegrationTestExitCode) -> ! {
    use ::x86_64::instructions::port::Port;

    let mut exit_port = Port::<u32>::new(0xf4);

    match exit_code {
        IntegrationTestExitCode::Success => {
            // This is safe because it runs on qemu and the port is mapped to exit qemu when written to
            unsafe { exit_port.write(0) }
        }
        IntegrationTestExitCode::Failure(message) => {
            serial_print!("{}", message);
            // This is safe because it runs on qemu and the port is mapped to exit qemu when written to
            unsafe { exit_port.write(1) }
        }
    }

    unreachable!("qemu should have exited now!");
}

/// The possible exit codes for integration tests.
#[cfg(any(feature = "qemu_integration_test"))]
pub enum IntegrationTestExitCode {
    /// The test was successful.
    Success,
    /// The test failed.
    Failure(&'static str),
}
