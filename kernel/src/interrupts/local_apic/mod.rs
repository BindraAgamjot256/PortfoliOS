use spin::Mutex;
use x2apic::lapic::{LocalApic, LocalApicBuilder, TimerMode};
use x86_64::instructions::port::Port;
use core::cell::UnsafeCell;


pub fn local_apic_init(apic_base: usize) -> LocalApic {
    // Mask all IRQs on the legacy PICs
    unsafe {
        let mut pic1_data = Port::<u8>::new(0xA1);
        let mut pic2_data = Port::<u8>::new(0x21);
        pic1_data.write(0xFF);
        pic2_data.write(0xFF);
    }

    // Calibrate APIC timer using PIT
    let ticks_in_10ms = unsafe {
        let apic = apic_base as *mut u32;

        // Configure APIC timer divider (16)
        let timer_div = apic.add(0x3E0 / 4) as *mut u32;
        timer_div.write_volatile(0x3);

        // Prepare PIT for 10ms sleep (mode 0)
        let divisor = 11931;
        let mut pit_cmd = Port::<u8>::new(0x43);
        let mut pit_data = Port::<u8>::new(0x40);

        pit_cmd.write(0x30); // Channel 0, mode 0, lo/hi byte
        pit_data.write((divisor & 0xFF) as u8);
        pit_data.write((divisor >> 8) as u8);

        // Start APIC timer with max initial count
        let timer_init = apic.add(0x380 / 4);
        timer_init.write_volatile(0xFFFFFFFF);

        // Wait for PIT to complete countdown
        loop {
            pit_cmd.write(0x00); // Latch count command
            let low = pit_data.read();
            let high = pit_data.read();
            if ((high as u16) << 8) | (low as u16) == 0 {
                break;
            }
        }

        // Stop APIC timer and read remaining count
        let lvt_timer = apic.add(0x320 / 4) as *mut u32;
        lvt_timer.write_volatile(0x10000); // Mask interrupt

        let curr_cnt = apic.add(0x390 / 4) as *mut u32;
        let current = curr_cnt.read_volatile();

        0xFFFFFFFFu32 - current
    };

    // Configure Local APIC with calibrated values
    let mut builder = LocalApicBuilder::new();
    builder
        .timer_vector(0x20)
        .spurious_vector(0x2e)
        .error_vector(0x2f)
        .timer_mode(TimerMode::Periodic)
        .timer_divide(x2apic::lapic::TimerDivide::Div16) // Divider 16 (0x3)
        .timer_initial(ticks_in_10ms)
        .set_xapic_base(apic_base as u64);

    let apic = builder.build().expect("APIC initialization failed");
    apic
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
/// # The name.
/// yes.
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
