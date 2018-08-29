//! The architecture specific parts of BeetleOS.
//!
//! This module acts as an interface between the individual architecure specific parts and the rest of the kernel.
//! It is supposed to abstract over the fact that there may be different architectures that the kernel is running on.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub use self::x86_64::write_fmt;

/// Prints text to the screen.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::arch::write_fmt(format_args!($($arg)*));
    });
}

/// Prints a line to the screen.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        $crate::arch::write_fmt(format_args!(concat!($fmt, "\r\n")));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::arch::write_fmt(format_args!(concat!($fmt, "\r\n"), $($arg)*));
    };
}
