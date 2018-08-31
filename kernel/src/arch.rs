//! The architecture specific parts of BeetleOS.
//!
//! This module acts as an interface between the individual architecure specific parts and the rest of the kernel.
//! It is supposed to abstract over the fact that there may be different architectures that the kernel is running on.

use core::fmt;

#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod x86_64;

/// The current architecture (x86_64).
#[cfg(target_arch = "x86_64")]
pub type Arch = x86_64::x86_64;

/// This type represents an abstraction of the underlying architecture.
/// 
/// Each supported architecture implements this trait on a type which is then used for architecture specific actions.
/// 
/// Using a trait here ensures that all the functions are implemented correctly by the corresponding architecture.
pub trait Architecture {
    /// Writes the formatted string to the screen.
    fn write_fmt(args: fmt::Arguments);

    /// Writes a line break to the screen.
    fn write_line_break();

    /// Determines if interrupts are currently enabled.
    /// 
    /// If `true` is returned then interrupts are possible.
    fn interrupts_enabled() -> bool;

    /// Disables interrupts.
    /// 
    /// # Safety
    /// While it is not possible to break Rust's safety guarantees using this function, caution is advised.
    /// 
    /// This function should only be called if it is ensured that interrupts are enabled again.
    fn disable_interrupts();

    /// Enables interrupts.
    /// 
    /// # Safety
    /// While it is not possible to break Rust's safety guarantees using this function, caution is advised.
    /// 
    /// This function should only be called if it is ensured that anything that should not be interrupted has finished.
    fn enable_interrupts();

    /// Disables or enables interrupts depending on the flag passed.
    /// 
    /// # Safety
    /// While it is not possible to break Rust's safety guarantees using this function, caution is advised.
    /// 
    /// This function should only be called if
    /// - it is ensured that interrupts are enabled again if interrupts were disabled.
    /// - it is ensured that anything that should not be interrupted has finished if interrupts were enabled.
    fn set_interrupts_enabled(enable: bool) {
        if enable {
            Self::enable_interrupts();
        } else {
            Self::disable_interrupts();
        }
    }
}

/// Prints text to the screen.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use $crate::arch::Architecture;
        $crate::arch::Arch::write_fmt(format_args!($($arg)*));
    });
}

/// Prints a line to the screen.
#[macro_export]
macro_rules! println {
    () => ({
        use $crate::arch::Architecture;
        $crate::arch::Arch::write_line_break();
    });
    ($($arg:tt)*) => ({
        use $crate::arch::Architecture;
        $crate::arch::Arch::write_fmt(format_args!($($arg)*));
        $crate::arch::Arch::write_line_break();
    });
}
