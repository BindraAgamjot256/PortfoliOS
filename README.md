# PortfoliOS
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project is a simple operating system written in Rust. It is a text environment based mirror for my portfolio
project which is simultaneously being developed (currently private, for those interested). It includes a shell(AgamShell or Ash), and a simple programming language, for user scripts.  


## Features

| Feature      | Description                                             |
|--------------|---------------------------------------------------------|
| Framebuffer   | A simple VGA buffer for text output                    |
| Unit Testing | Full Unit testing for the kernel (TODO)                 |
| CI/CD        | Continuous Integration and Deployment                   |
| Keyboard     | Keyboard input for the CLI                              |
| CLI          | Command Line Interface                                  |

## Getting Started

### Prerequisites

- Rust nightly toolchain
- QEMU for running the kernel

### Installation

1. Install Rust nightly toolchain and components:

    ```sh
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    rustup component add llvm-tools-preview --toolchain nightly
    ```

2. Install QEMU:

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
cargo run
```


## TODO:
- [x] Framebuffer (for text/image output)
- [ ] Unit Testing
- [ ] Full Documentation coverage
- [x] CLI
   - [x] Keyboard Input
   - [x] Commands
- [ ] Full Kernel
- [ ] File System
- [ ] CI/CD
- [x] Customisable Printing


## Licence.
This project is licenced under the MIT Licence. See the [License](./LICENSE) for more details
