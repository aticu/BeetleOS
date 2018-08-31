//! This module handles serial IO for the x86_64 architecture.

use core::fmt::{
    self,
    Write
};
use lazy_static::lazy_static;
use uart_16550::SerialPort;

use crate::sync::Mutex;

lazy_static! {
    /// Provides access to the serial port.
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(0x3f8);

        serial_port.init();

        Mutex::new(serial_port)
    };
}

/// Prints the formatted arguments to the serial port.
pub fn write_fmt(args: fmt::Arguments) {
    SERIAL.lock().write_fmt(args).expect("Could not write to serial.")
}

/// Print to the serial output.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::arch::x86_64::serial::write_fmt(format_args!($($arg)*));
    };
}

/// Print a line to the serial output.
#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}