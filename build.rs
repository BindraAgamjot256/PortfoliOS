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

    let mut disk_image = bootloader::DiskImageBuilder::new(kernel);  
    
    disk_image.set_file_contents(
        "a_file.txt".to_string(),
        "Hello, world!".as_bytes().to_vec(),
    );


    let uefi_path = out_dir.join("uefi.img");

    let bios_path = out_dir.join("bios.img");

    disk_image
        .create_uefi_image(uefi_path.as_path())
        .expect("Failed to build UEFI disk image");

    disk_image
        .create_bios_image(bios_path.as_path())
        .expect("Failed to build BIOS disk image");

    // Pass the disk image paths to the main runner.
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.to_str().unwrap());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.to_str().unwrap());

    // Rerun build if relevant sources change.
    println!("cargo:rerun-if-changed=kernel/src/*.rs");
    println!("cargo:rerun-if-changed=src/*.rs");
}

