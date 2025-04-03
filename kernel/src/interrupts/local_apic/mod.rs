use spin::Mutex;
use x2apic::lapic::{LocalApic, LocalApicBuilder};
use x86_64::instructions::port::Port;
use core::cell::UnsafeCell;



/// Initialize the Local APIC using a given APIC base address.
/// This function disables the legacy PIC and configures the x2APIC.
pub fn local_apic_init(apic_base: usize) -> LocalApic {
    // Mask all IRQs on the legacy PICs.
    unsafe {
        let mut pic1_data = Port::<u8>::new(0xA1);
        let mut pic2_data = Port::<u8>::new(0x21);
        pic1_data.write(0xFF);
        pic2_data.write(0xFF);
    }

    let mut builder = LocalApicBuilder::new();
    builder
        .timer_vector(0x20)
        .spurious_vector(0x2e)
        .error_vector(0x2f)
        .timer_initial(1234567890 / 2)
        .set_xapic_base(apic_base as u64);


    builder.build().expect("APIC should work")
}


/// A wrapper type that allows a global mutable reference to the Local APIC.
///
/// Since LocalApic is not Send or Sync, we wrap it in an UnsafeCell
/// and manually implement Send/Sync. **Ensure that this is safe in your context!**
pub struct GloballyAvailableLocalApic(UnsafeCell<LocalApic>);

unsafe impl Send for GloballyAvailableLocalApic {}
unsafe impl Sync for GloballyAvailableLocalApic {}

impl GloballyAvailableLocalApic {
    /// Create a new GlobalApic from an initialized LocalApic.
    pub const fn new(apic: LocalApic) -> Self {
        GloballyAvailableLocalApic(UnsafeCell::new(apic))
    }

    /// Obtain a mutable reference to the LocalApic.
    ///
    /// # Safety
    ///
    /// Caller must ensure that this reference is not aliased and that
    /// concurrent accesses do not occur.
    pub unsafe fn get_mut(&self) -> &mut LocalApic {
        &mut *self.0.get()
    }
}

/// A global static for the Local APIC.
///
/// You must initialize this once early in your boot process.
pub static LOCAL_APIC: Mutex<Option<GloballyAvailableLocalApic>> = Mutex::new(None);

/// Initialize the global APIC variable with a given APIC base address.
///
/// # Safety
///
/// This function is unsafe because it writes to a mutable static variable.
/// Caller must ensure that this initialization happens exactly once
/// and that no data races occur.
pub unsafe fn init_globally_available_local_apic(apic_base: usize) {
    let lapic = local_apic_init(apic_base);
    *LOCAL_APIC.lock() = Some(GloballyAvailableLocalApic::new(lapic));
}
