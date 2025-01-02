# PortfoliOS

This project is a simple operating system written in Rust. It is a text environment based mirror for my portfolio
project which is simultaneously being developed.

## Features

| Feature      | Description                                     |
|--------------|-------------------------------------------------|
| VGA Buffer   | A simple VGA buffer for text output             |
| Unit Testing | Unit tests for the kernel(Partially completed)  |
| CI/CD        | Continuous Integration and Deployment           |
| Keyboard     | Keyboard input for the CLI                      |
| CLI          | Command Line Interface       (TODO)             |
| Interpreter  | Interpreter for programming language     (TODO) |
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

        ```shell
        sudo apt-get update
        sudo apt-get install -y qemu-system-x86
        ```

       OR
   ```shell
    brew install qemu
    ```
   OR
   WINDOWS:https://qemu.weilnetz.de/w64/

### Running the Kernel

To run the kernel, run:

```sh
cargo run --target x86_64-PortfoliOS.json
```

> [!NOTE]
> The `--target` flag is unnecessary, as the default target is defined [here](./.cargo/config.toml).

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
- [x] Customisable Printing
- [ ] Customisable VGA Buffer
- [ ] Sandboxed programming language for user scripts 
