# PortfoliOS
![rustfmt](https://github.com/BindraAgamjot256/PortfoliOS/actions/workflows/cargo-fmt.yml/badge.svg)
![testing](https://github.com/BindraAgamjot256/PortfoliOS/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project is a simple operating system written in Rust. It is a text environment based mirror for my portfolio
project which is simultaneously being developed (currently private, for those interested). It includes a shell(AgamShell or Ash), and a simple programming language, for user scripts.  


## Features

| Feature      | Description                                             |
|--------------|---------------------------------------------------------|
| VGA Buffer   | A simple VGA buffer for text output                     |
| Unit Testing | Full Unit testing for the kernel (Partially completed)  |
| CI/CD        | Continuous Integration and Deployment                   |
| Keyboard     | Keyboard input for the CLI                              |
| CLI          | Command Line Interface       (TODO)                     |
| Interpreter  | Interpreter for programming language     (TODO)         |

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
   # for Linux
   sudo apt-get install -y qemu-system-x86
   ```

   OR
   ```shell
   # for MacOS
   brew install qemu
   ```
   OR  
   [for Windows](https://qemu.weilnetz.de/w64/)

### Running the Kernel

To run the kernel, run:

```sh
cargo run --target x86_64-PortfoliOS.json
```

> [!NOTE]
> The `--target` flag is unnecessary, as the default target is defined [here](./.cargo/config.toml) as PortfoliOS.

## TODO:
- [x] VGA Buffer (for text output)
- [x] Unit Testing
- [ ] Full Documentation coverage
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

## Other cool features I added:

- A python script, that creates a GUI, for easier debugging and testing, as well as building the project

## Licence.
This project is licenced under the MIT Licence. See the [License](./LICENSE) for more details
