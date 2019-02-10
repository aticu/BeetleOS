//! Implements the x86_64 physical memory model.
//!
//! # The phyiscal memory model
//! The model mainly describes ownership semantics of page frames. An entity that owns a page frame
//! is responsible for freeing it once it is not needed anymore. Freeing in this case means returning ownership to the phyiscal memory allocator.
//!
//! ## Sharing of page frames
//! Page frames may be shared. This is indicated by a shared bit in the page table that references the frame.
// TODO: where is that information stored? (probably in the address spaces)
//! More information about the sharing (such as the number of references) is not stored inside of the page table.
//!
//! As soon as memory ceases to be shared, the shared bit shall be removed.
//!
//! The entity freeing the shared memory is responsible for updating the reference count and removing the shared bit from the last reference, once the
//! reference count reaches one.
//!
//! ## Types of page frames
//! Page frames are separated into different types, each of which have different ownership semantics.
//!
//! The following list is read from top to bottom. If more than one type would apply, the first type on the list takes precendence.
//!
//! - In transit: This frame type applies to frames that were allocated, but not yet assigned somewhere. This type of frame should be very short lived.
//! - Unallocated: This frame type applies to frames not being used and therefore not referenced in any page table.
//! - Top level page table: This frame type applies to all level 4 page table frames.
//! - Kernel memory: This frame type applies to all frames that have the global bit set in references to them
//!   (note that this does not mean that all memory only available in kernel mode has this type, a kernel stack for example may have a different type).
//! - Shared memory: This frame type applies to all frames that have the shared bit set in references to them.
//! - Normal memory: This frame type applies to all other frames.
//!
//! ## Ownership of page frames
//! - In transit frames are owned by the `PhysFrame` or `PhysFrameRange` value containing them.
//! - Unallocated frames are owned by the physical memory allocator.
//! - Top level page table frames are owned by the address space they belong to.
//! - Kernel memory frames are owned by the part of the kernel responsible for the memory
//!   (for example the kernel heap owns the kernel heap frames).
//! - Shared memory frames are owned by each of their shareholders (page tables referencing them). Once they cease to be shared,
//!   the last remaining owner gains exclusive ownership of the frames and their type changes to normal memory.
//! - Normal memory frames are owned by the page table referencing them.

use core::ptr;
use x86_64_crate::structures::paging::{PageSize, PhysFrame, PhysFrameRange};

use crate::memory::PhysicalAddress;

/// This trait provides an interface for accessing physical memory.
///
/// This trait can be implemented for each access method available. These methods may become invalid as
/// initialization progresses to further stages.
///
/// # Safety
/// The traits methods on implementations of this trait should only be used while their access method is still valid.
pub(in crate::arch::x86_64::memory) unsafe trait PhysicalMemoryAccessor {
    /// Reads the value of type `T` from the given physical address.
    ///
    /// The address is treated as a *const T, only that it references a physical address that may not be
    /// mapped to the same virtual address.
    ///
    /// # Safety
    /// `address` must be a valid physical address pointing to unallocated memory.
    unsafe fn read<T>(address: PhysicalAddress) -> T;

    /// Writes the value of type `T` to the given physical address.
    ///
    /// The address is treated as a *mut T, only that it references a physical address that may not be
    /// mapped to the same virtual address.
    ///
    /// # Safety
    /// `address` must be a valid physical address pointing to unallocated memory.
    unsafe fn write<T>(address: PhysicalAddress, val: T);
}

/// A physical memory accessor using identity mapped pages.
///
/// # Safety
/// This should only be used while pages are still identity mapped.
struct IdentityMappedAccessor;

// This is safe under the assumption that this will only be used while the pages are identity mapped
unsafe impl PhysicalMemoryAccessor for IdentityMappedAccessor {
    unsafe fn read<T>(address: PhysicalAddress) -> T {
        ptr::read(address.0.as_u64() as *const T)
    }

    unsafe fn write<T>(address: PhysicalAddress, val: T) {
        ptr::write(address.0.as_u64() as *mut T, val)
    }
}

trait FrameAllocator<S>
where
    S: PageSize,
{
    /// Attempts to allocate a frame of the appropriate size.
    ///
    /// This method should only return `None` if there are no frames left.
    fn allocate(&mut self) -> Option<PhysFrame<S>>;

    /// Attempts to allocate a frame range of the appropriate size with `num` frames in it.
    ///
    /// This method should return `None` if there are no frames left.
    ///
    /// Should the underlying allocator not support ranges, it should return single pages as ranges of size 1.
    ///
    /// If there are only ranges with less than `num` frames available, the largest range will be returned,
    /// even though it has less than `num` frames.
    #[allow(unused_variables)]
    fn allocate_range(&mut self, num: usize) -> Option<PhysFrameRange<S>> {
        self.allocate().map(|frame| {
            let end_frame =
                PhysFrame::from_start_address(frame.start_address() + frame.size()).unwrap();

            PhysFrameRange {
                start: frame,
                end: end_frame,
            }
        })
    }
}
