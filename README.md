# PortfoliOS

This project is a simple operating system written in Rust. It is a text enviorment based mirror for my portfolio project which is simultaneously being developed.

## Features

- VGA text buffer for text output
- Custom `print!` and `println!` macros
- Unit tests for kernel functionality

## Getting Started

### Prerequisites

- Rust nightly toolchain
- `bootimage` tool
- QEMU for running the kernel

### Installation

1. Install Rust nightly toolchain and components:

    ```sh
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    rustup component add llvm-tools-preview --toolchain nightly
    ```

2. Install `bootimage`:

    ```sh
    cargo install bootimage
    ```

3. Install QEMU:

    ```sh
    sudo apt-get update
    sudo apt-get install -y qemu-system-x86
    ```

### Building the Kernel

To build the kernel, run:

```sh
cargo build --target x86_64-PortfoliOS.json
```


## TODO:
- [x] VGA Buffer (for text output)
- [x] Unit Testing
- [x] Full Doc. coverage
- [ ] CLI
- [ ] File System
- [x] CI/CD
- [ ] Kernel rings (Ring 0, Ring 1, etc...)[^1]<br>![image](https://github.com/user-attachments/assets/eedc386a-46c3-4081-bee3-8efab6d014b5)



[^1]:Image Copyright: [https://www.baeldung.com/cs/os-rings](https://www.baeldung.com/cs/os-rings)
