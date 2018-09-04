//! This module defines the kernel logger.

use log::{Level, Log, Metadata, Record};

/// The type of the kernel logger.
pub struct KernelLogger;

/// The kernel logger.
pub static KERNEL_LOGGER: KernelLogger = KernelLogger;

impl Log for KernelLogger {
    #[cfg(feature = "integration_test")]
    fn enabled(&self, _metadata: &Metadata) -> bool {
        false
    }

    #[cfg(not(feature = "integration_test"))]
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // The colors to use for terminal output.
        let reset = "\x1b[0m";
        let error = "\x1b[31m"; // Red
        let warn = "\x1b[33m"; // Yellow
        let debug = ""; // Default
        let trace = "\x1b[90m"; // Dark gray

        match record.metadata().level() {
            Level::Error => {
                println!("{}", record.args());
                serial_println!("{}{}{}: {}", error, record.level(), reset, record.args());
            }
            Level::Warn => {
                println!("{}", record.args());
                serial_println!("{}{}{}: {}", warn, record.level(), reset, record.args());
            }
            Level::Info => {
                println!("{}", record.args());
                serial_println!("{}", record.args());
            }
            Level::Debug => {
                serial_println!("{}{}{}: {}", debug, record.level(), reset, record.args());
            }
            Level::Trace => {
                serial_println!("{}{}{}: {}", trace, record.level(), reset, record.args());
            }
        }
    }

    fn flush(&self) {}
}
