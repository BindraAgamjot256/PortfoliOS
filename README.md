# 🚀 PortfoliOS  
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**PortfoliOS** is a tiny operating system written in Rust — not meant for general use, but built entirely to flex my portfolio in the most extra way possible.

It runs a custom shell (AgamShell aka `ash`) and will eventually feature a custom scripting language, AI integrations, Rick Astley at boot, and zero POSIX compliance because that’s boring.

> It’s a portfolio... that *boots*.  
> LinkedIn could never.

> [!WARNING]  
> **DO NOT RUN THIS ON ACTUAL HARDWARE.**  
> It doesn’t even possess proper shutdown functionality yet. You’ve been warned.

---

## ✨ Features

| Feature       | Description                                                |
|---------------|------------------------------------------------------------|
| 🖼️ Framebuffer | Custom framebuffer with gradient/Rickroll boot visuals     |
| 🎹 Keyboard    | PS/2 keyboard input + input buffering                     |
| 💬 CLI         | Command Line Interface with custom shell + commands       |
| 🧠 AI Cmds     | (Planned) Local chatbot using TinyLlama or Mixtral        |
| 🧪 Unit Tests  | Kernel-level unit testing (WIP)                            |
| 🚀 CI/CD       | Planned auto-build pipeline                               |

---

## 🧰 Getting Started

### 🔧 Prerequisites

- Rust nightly toolchain  
- QEMU (for running your kernel)

### 🛠 Installation

```sh
rustup install nightly
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
```

#### QEMU Setup

- **Linux**: `sudo apt install qemu-system-x86`
- **macOS**: `brew install qemu`
- **Windows**: [Download here](https://qemu.weilnetz.de/w64/)

---

## 🏃 Running the Kernel

```sh
cargo run
```

(And prepare for vibes.)

---

## 📝 TODO

- [x] Framebuffer support  
- [x] Basic CLI with custom shell  
- [x] Keyboard input  
- [x] `portfoliofetch` (like neofetch, but better 😎)  
- [ ] Proper scrolling + shell history  
- [ ] File system (eventually)  
- [ ] Scripting language support  
- [ ] Shutdown command that Rickrolls and hangs  
- [ ] Unit testing framework  
- [ ] CI/CD  
- [ ] Fully documented code (lol maybe)

---

## 📜 License

MIT. Do whatever you want, just don’t claim you built it unless your name is Agamjot Singh Bindra.
