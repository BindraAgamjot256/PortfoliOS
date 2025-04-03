use bootloader;
use std::path::PathBuf;

fn main() {
    // Cargo provides the OUT_DIR for build artifacts.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // Cargo's artifact dependency feature provides the kernel binary.
    // This binary is the kernel ELF file built with debug symbols.
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap());

    // Pass the kernel path as environment variables for use in the main runner.
    println!("cargo:rustc-env=KERNEL_PATH={}", kernel.to_str().unwrap());
    // Set a separate variable for clarity; KERNEL_ELF is used by LLDB.
    println!("cargo:rustc-env=KERNEL_ELF={}", kernel.to_str().unwrap());

    // Create a UEFI disk image.
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    // Create a BIOS disk image.
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // Pass the disk image paths to the main runner.
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.to_str().unwrap());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.to_str().unwrap());

    // Rerun build if relevant sources change.
    println!("cargo:rerun-if-changed=kernel/src/*.rs");
    println!("cargo:rerun-if-changed=src/*.rs");
}

