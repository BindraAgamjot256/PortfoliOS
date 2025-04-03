use acpi::fadt::Fadt;

/// Shutdown the system using ACPI, given the FADT address.
///
/// # Safety
/// This function performs raw memory access and I/O port writes, which are inherently unsafe.
pub unsafe fn acpi_shutdown(fadt_address: usize) {
    // Interpret the given address as a reference to the FADT table
    let fadt = &*(fadt_address as *const Fadt);

    // Enable ACPI mode if necessary
    if fadt.smi_cmd_port > 0 && fadt.acpi_enable > 0 {
        let mut port = x86_64::instructions::port::Port::new(fadt.smi_cmd_port as u16);
        port.write(fadt.acpi_enable);
    }

    // ACPI shutdown via PM1 control register
    const SLP_TYP5: u16 = 5 << 10; // SLP_TYP value for S5 (soft-off)
    const SLP_EN: u16 = 1 << 13; // SLP_EN bit

    // Write to PM1a_CNT register
    if fadt.pm1a_control_block().unwrap().address > 0 {
        let mut port = x86_64::instructions::port::Port::new(
            fadt.pm1a_control_block().unwrap().address as u16,
        );
        port.write(SLP_TYP5 | SLP_EN);
    }

    // Write to PM1b_CNT register if it exists
    if fadt.pm1b_control_block().unwrap().unwrap().address > 0 {
        let mut port = x86_64::instructions::port::Port::new(
            fadt.pm1b_control_block().unwrap().unwrap().address as u16,
        );
        port.write(SLP_TYP5 | SLP_EN);
    }
}
