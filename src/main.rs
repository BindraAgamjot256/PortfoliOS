use ovmf_prebuilt;
use std::process::Command;

fn main() {
    // Retrieve environment variables set by build.rs.
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    let kernel_elf = env!("KERNEL_ELF");

    println!("Kernel ELF: {}", kernel_elf);
    println!("Using UEFI image at: {}", uefi_path);

    // Choose boot mode: set to true for UEFI, false for BIOS.
    let use_uefi = false;

    // Build the QEMU arguments.
    let mut qemu_args = Vec::new();
    if use_uefi {
        // For UEFI, supply firmware and UEFI disk image.
        qemu_args.push("-bios".to_string());
        qemu_args.push(ovmf_prebuilt::ovmf_pure_efi().to_str().unwrap().to_string());
        qemu_args.push("-drive".to_string());
        qemu_args.push(format!("format=raw,file={}", uefi_path));
    } else {
        // Otherwise, use the BIOS disk image.
        qemu_args.push("-drive".to_string());
        qemu_args.push(format!("format=raw,file={}", bios_path));
    }
    // Additional QEMU options.
    qemu_args.extend_from_slice(&[
        "-vga".to_string(),
        "qxl".to_string(),
        "-serial".to_string(),
        "stdio".to_string(),
        "-audiodev".to_string(),
        "dsound,id=snd0".to_string(),
        "-machine".to_string(),
        "pcspk-audiodev=snd0".to_string(),
        // Start the GDB server on port 1234 and halt execution.
        //"-s".to_string(),
        //"-S".to_string(),
        "-cpu".to_string(),
        "qemu64,+x2apic".to_string(),
    ]);

    
    let mut qemu_cmd = Command::new("qemu-system-x86_64");
    // Append QEMU's arguments.
    for arg in qemu_args {
        qemu_cmd.arg(arg);
    }
    println!("Launching QEMU with command: {:?}", qemu_cmd);
    // Spawn QEMU as an independent process.
    qemu_cmd.spawn().expect("Failed to launch QEMU").wait().unwrap();

}
