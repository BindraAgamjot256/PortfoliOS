#![allow(static_mut_refs)]

use spin::Lazy;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    unsafe {
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(STACK.as_ptr());
            let stack_end = stack_start + STACK_SIZE as u64;
            // Align stack_end down to a 16-byte boundary.
            stack_end.align_down(16u64)
        };
    }
    tss
});

pub(crate) static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.append(Descriptor::kernel_code_segment());
    let data_selector = gdt.append(Descriptor::kernel_data_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
    let usr_code_selector = gdt.append(Descriptor::user_code_segment());
    let usr_data_selector = gdt.append(Descriptor::user_data_segment());
    (gdt, Selectors {
        code_selector,
        data_selector,
        tss_selector,
        usr_code_selector,
        usr_data_selector,
    })
});

#[derive(Clone, Copy)]
pub(crate) struct Selectors {
    pub(crate) code_selector: SegmentSelector,
    pub(crate) data_selector: SegmentSelector,
    pub(crate) tss_selector: SegmentSelector,
    pub(crate) usr_code_selector: SegmentSelector,
    pub(crate) usr_data_selector: SegmentSelector,
}
