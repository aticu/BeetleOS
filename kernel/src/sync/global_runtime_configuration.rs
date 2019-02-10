//! Provides a one-time initialized global runtime configuration.

use spin::Once;

/// Represents a one time initialized global configuration at runtime.
pub struct GlobalRuntimeConfiguration<T> {
    /// The instance that actually holds the value.
    ///
    /// This also does the synchronization.
    inner: Once<T>,
}

impl<T> GlobalRuntimeConfiguration<T> {
    /// Creates a new global runtime configuration.
    pub const fn new() -> GlobalRuntimeConfiguration<T> {
        GlobalRuntimeConfiguration { inner: Once::new() }
    }

    /// Initializes the configuration with the given value, if it isn't already initialized.
    pub fn init(&self, val: T) {
        self.inner.call_once(|| val);
    }

    /// Returns the value in the global configuration, if it exists.
    pub fn get(&self) -> Option<&T> {
        self.inner.r#try()
    }
}
