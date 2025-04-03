use core::cell::UnsafeCell;
use spin::Mutex;
use x2apic::ioapic::IoApic;

pub unsafe fn io_apic_init(apic_base: u64) -> IoApic {
    let mut io_apic = IoApic::new(apic_base);
    io_apic.init(0x23);
    io_apic
}

pub struct GloballyAvailableIoApic(UnsafeCell<IoApic>);

unsafe impl Send for GloballyAvailableIoApic {}
unsafe impl Sync for GloballyAvailableIoApic {}

impl GloballyAvailableIoApic {
    pub const fn new(io_apic: IoApic) -> Self {
        GloballyAvailableIoApic(UnsafeCell::new(io_apic))
    }

    pub fn get_mut(&self) -> &mut IoApic {
        unsafe { &mut *self.0.get() }
    }
}

pub static IO_APIC: Mutex<Option<GloballyAvailableIoApic>> = Mutex::new(None);

pub unsafe fn init_globally_available_io_apic(apic_base: u64) {
    let io_apic = io_apic_init(apic_base);
    *IO_APIC.lock() = Some(GloballyAvailableIoApic::new(io_apic));
}