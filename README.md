# PortfoliOS

This project is a simple operating system written in Rust. It is a text environment based mirror for my portfolio
project which is simultaneously being developed.

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

2. Install `bootimage` crate:

    ```sh
    cargo install bootimage
    ```

3. Install QEMU:

    ```sh
    sudo apt-get update
    sudo apt-get install -y qemu-system-x86
    ```

### Running the Kernel

To run the kernel, run:

```sh
cargo run --target x86_64-PortfoliOS.json
```


## TODO:
- [x] VGA Buffer (for text output)
- [x] Unit Testing
- [ ] Full Doc. coverage
- [ ] CLI
   - [x] Keyboard Input
   - [ ] Commands
   - [ ] Command History
   - [ ] Interpreter for programming language
- [ ] Full Kernel
- [ ] File System
- [x] CI/CD
- [ ] Customisable Printing
- [ ] Customisable VGA Buffer
- [ ] Sandboxed programming language for user scripts 
