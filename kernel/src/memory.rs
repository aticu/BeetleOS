//! This module handles architecture independent memory management.

use crate::arch;

/// Represents a physical address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct PhysicalAddress(pub arch::PhysicalAddressType);

impl Into<arch::PhysicalAddressType> for PhysicalAddress {
    fn into(self) -> arch::PhysicalAddressType {
        self.0
    }
}

/// Represents a virtual address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct VirtualAddress(pub arch::VirtualAddressType);

impl Into<arch::VirtualAddressType> for VirtualAddress {
    fn into(self) -> arch::VirtualAddressType {
        self.0
    }
}
