//! Handles mutual exclusion to data.
//!
//! This is a modification of the Mutex code from the spin crate (see
//! https://crates.io/crates/spin).

use core::{
    cell::UnsafeCell,
    default::Default,
    fmt,
    marker::Sync,
    ops::{Deref, DerefMut, Drop},
    option::Option::{self, None, Some},
    sync::atomic::{spin_loop_hint, AtomicBool, Ordering},
};

use super::{disable_preemption, PreemptionState};

/// This type provides MUTual EXclusion based on spinning.
///
/// # Description
///
/// This structure behaves a lot like a normal Mutex. There are some
/// differences:
///
/// - It may be used outside the runtime.
/// - A normal mutex will fail when used without the runtime, this will just
/// lock
/// - When the runtime is present, it will call the deschedule function when
/// appropriate
/// - No lock poisoning. When a fail occurs when the lock is held, no
/// guarantees are made
pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    _preemption_state: PreemptionState,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new spinlock wrapping the supplied data.
    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this mutex, returning the underlying data.
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let Mutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    fn obtain_lock(&self) -> PreemptionState {
        let mut preemption_state;
        loop {
            preemption_state = disable_preemption();

            let lock_switch = !self.lock.compare_and_swap(false, true, Ordering::Acquire);

            if lock_switch {
                break;
            }

            // Wait until the lock looks unlocked before retrying
            while self.lock.load(Ordering::Relaxed) {
                spin_loop_hint();
            }
        }

        preemption_state
    }

    /// Locks the spinlock and returns a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    pub fn lock(&self) -> MutexGuard<T> {
        let preemption_state = self.obtain_lock();

        MutexGuard {
            lock: &self.lock,
            _preemption_state: preemption_state,
            // This is safe, because the data is protected by the lock
            data: unsafe { &mut *self.data.get() },
        }
    }

    /// Tries to lock the mutex.
    ///
    /// If it is already locked, it will return None.
    /// Otherwise it returns a guard within Some.
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let preemption_state = disable_preemption();

        let lock_switch = !self.lock.compare_and_swap(false, true, Ordering::Acquire);

        if lock_switch {
            Some(MutexGuard {
                lock: &self.lock,
                _preemption_state: preemption_state,
                // This is safe, because the data is protected by the lock
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "Mutex {{ data: {:?} }}", &*guard),
            None => write!(f, "Mutex {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}
