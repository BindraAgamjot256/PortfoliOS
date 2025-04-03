pub mod management;

use acpi::{AcpiHandler, PhysicalMapping};
use core::ptr::NonNull;
use spin::Mutex;
use x86_64::VirtAddr;

// Define a simple handler that assumes physical memory is already mapped
pub struct KernelAcpiHandler {
    // The offset from physical to virtual addresses in your kernel's memory map
    phys_to_virt_offset: u64,
}

impl KernelAcpiHandler {
    pub fn new(phys_to_virt_offset: u64) -> Self {
        KernelAcpiHandler {
            phys_to_virt_offset,
        }
    }
}

impl Clone for KernelAcpiHandler {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for KernelAcpiHandler {}

impl AcpiHandler for KernelAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        // Calculate the virtual address by adding the offset
        let virt_addr = VirtAddr::new(physical_address as u64 + self.phys_to_virt_offset);

        // Create a pointer to the virtual address
        let virt_ptr = NonNull::new(virt_addr.as_mut_ptr::<T>())
            .expect("Failed to create pointer to mapped memory");

        PhysicalMapping::new(physical_address, virt_ptr, size, size, *self)
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // No need to unmap - we're using the kernel's existing mapping
    }
}

pub static FADT_ADDR: Mutex<Option<usize>> = Mutex::new(None);
