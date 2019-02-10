//! This modules handles synchronization in the kernel.

mod global_runtime_configuration;
mod mutex;

pub use self::{global_runtime_configuration::GlobalRuntimeConfiguration, mutex::Mutex};
use crate::arch::{Arch, Architecture};

/// Saves the state when disabling preemtion, so it can be restored later when dropping.
///
/// When the `PreemptionState` is dropped, it is restored. In the very rare case this is not wanted,
/// it is possible to call `core::mem::forget` on the `PreemptionState`.
#[derive(Debug, Default)]
pub struct PreemptionState {
    /// Saves whether interrupts were allowed, when preemtion was disabled.
    interrupts_enabled: bool,
}

impl PreemptionState {
    /// Reads the current state of preemptability.
    fn current() -> PreemptionState {
        PreemptionState {
            interrupts_enabled: Arch::interrupts_enabled(),
        }
    }
}

impl Drop for PreemptionState {
    fn drop(&mut self) {
        Arch::set_interrupts_enabled(self.interrupts_enabled)
    }
}

/// Disables preemption returning the previous preemption state.
fn disable_preemption() -> PreemptionState {
    let preeption_state = PreemptionState::current();

    Arch::disable_interrupts();

    preeption_state
}
