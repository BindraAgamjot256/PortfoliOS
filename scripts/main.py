#!/usr/bin/env python3
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import sys
import threading
import queue
import argparse
import os
import toml
from typing import Optional

class RustProjectManager:
    def __init__(self, root):
        self.root = root
        self.root.title("Rust Project Manager")
        self.root.geometry("800x600")

        self.message_queue = queue.Queue()
        self.detect_project_toolchain()
        self.setup_ui()
        self.check_cli_args()
        self.root.after(100, self.check_queue)

    def detect_project_toolchain(self):
        self.project_toolchain = "stable"  # default

        # Check for rust-toolchain.toml
        if os.path.exists("rust-toolchain.toml"):
            try:
                toolchain_config = toml.load("rust-toolchain.toml")
                if 'toolchain' in toolchain_config:
                    self.project_toolchain = toolchain_config['toolchain'].get('channel', 'stable')
            except Exception:
                pass

        # Check for rust-toolchain file
        elif os.path.exists("rust-toolchain"):
            try:
                with open("rust-toolchain", 'r') as f:
                    content = f.read().strip()
                    if content:
                        self.project_toolchain = content
            except Exception:
                pass

        # Check Cargo.toml for nightly features
        if os.path.exists("Cargo.toml"):
            try:
                cargo_config = toml.load("Cargo.toml")

                # Check for explicit feature flags that require nightly
                features = []
                if 'features' in cargo_config:
                    features.extend(cargo_config['features'])
                if 'package' in cargo_config and 'features' in cargo_config['package']:
                    features.extend(cargo_config['package']['features'])

                # Check for #![feature(...)] in lib.rs or main.rs
                source_files = ['src/lib.rs', 'src/main.rs']
                for file_path in source_files:
                    if os.path.exists(file_path):
                        with open(file_path, 'r') as f:
                            content = f.read()
                            if '#![feature(' in content:
                                self.project_toolchain = 'nightly'
                                break

            except Exception:
                pass

    def setup_ui(self):
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))

        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_columnconfigure(0, weight=1)
        main_frame.grid_rowconfigure(1, weight=1)
        main_frame.grid_columnconfigure(0, weight=1)

        # Toolchain selector
        toolchain_frame = ttk.Frame(main_frame)
        toolchain_frame.grid(row=0, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 10))

        self.toolchain_var = tk.StringVar(value=self.project_toolchain)
        self.toolchain_combo = ttk.Combobox(toolchain_frame, textvariable=self.toolchain_var,
                                          values=["stable", "nightly"], state="readonly", width=20)
        self.toolchain_combo.grid(row=0, column=0)
        self.toolchain_combo.bind('<<ComboboxSelected>>', self.on_toolchain_change)

        # Test output display
        self.test_text = tk.Text(main_frame, height=20, width=90, wrap=tk.NONE)
        self.test_text.grid(row=1, column=0, pady=10, sticky=(tk.W, tk.E, tk.N, tk.S))
        self.test_text.tag_configure('pass', foreground='green')
        self.test_text.tag_configure('fail', foreground='red')
        self.test_text.tag_configure('summary', font=('TkDefaultFont', 10, 'bold'))
        self.test_text.tag_configure('warning', foreground='yellow')

        # Scrollbars
        h_scrollbar = ttk.Scrollbar(main_frame, orient=tk.HORIZONTAL, command=self.test_text.xview)
        h_scrollbar.grid(row=2, column=0, sticky=(tk.W, tk.E))

        v_scrollbar = ttk.Scrollbar(main_frame, orient=tk.VERTICAL, command=self.test_text.yview)
        v_scrollbar.grid(row=1, column=1, sticky=(tk.N, tk.S))

        self.test_text['xscrollcommand'] = h_scrollbar.set
        self.test_text['yscrollcommand'] = v_scrollbar.set

        # Buttons
        button_frame = ttk.Frame(main_frame)
        button_frame.grid(row=3, column=0, columnspan=2, pady=10)

        self.buttons = {
            'build': ttk.Button(button_frame, text="Build", command=self.build_project),
            'test': ttk.Button(button_frame, text="Test", command=lambda: self.run_cargo_command("test")),
            'dev': ttk.Button(button_frame, text="Dev", command=lambda: self.run_cargo_command("run")),
            'doc': ttk.Button(button_frame, text="Docs", command=lambda: self.run_cargo_command("doc", ["--no-deps", "--open"]))
        }

        for i, (_, button) in enumerate(self.buttons.items()):
            button.grid(row=0, column=i, padx=5)

    def check_cli_args(self):
        parser = argparse.ArgumentParser()
        parser.add_argument('-d', '--dev', action='store_true', help='Run in dev mode')
        parser.add_argument('-t', '--test', action='store_true', help='Run tests')
        parser.add_argument('--doc', action='store_true', help='Generate documentation')
        parser.add_argument('--nightly', action='store_true', help='Use nightly toolchain')

        args = parser.parse_args()

        if args.nightly:
            self.toolchain_var.set("nightly")
            self.on_toolchain_change(None)

        if args.test:
            self.root.after(100, lambda: self.run_cargo_command("test"))
        elif args.dev:
            self.run_cargo_command("run")
        elif args.doc:
            self.run_cargo_command("doc", ["--no-deps", "--open"])

    def on_toolchain_change(self, event):
        subprocess.run(["rustup", "default", self.toolchain_var.get()],
                     stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    def parse_test_output(self, line: str) -> tuple[tuple, str]:
        if not line.strip():
            return ((), "")

        if "test result:" in line.lower():
            return (('summary',), line)
        elif "test" in line.lower() and "..." in line:
            if "ok" in line.lower():
                return (('pass',), line)
            elif "failed" in line.lower():
                return (('fail',), line)
        elif "running " in line.lower() and "tests" in line.lower():
            return ((), line)
        elif "warning:" in line.lower():
            return (('warning',), line)
        elif "panicked at" in line.lower():
            return (('fail',), line)
        elif "stack overflow" in line.lower():
            return (('fail',), line)
        return None

    def check_queue(self):
        while True:
            try:
                message = self.message_queue.get_nowait()
                parsed = self.parse_test_output(message)
                if parsed is not None:
                    tags, line = parsed
                    if line:  # Only display if there's actual content
                        self.test_text.insert(tk.END, f"{line}\n", tags)
                        self.test_text.see(tk.END)
            except queue.Empty:
                break
        self.root.after(100, self.check_queue)

    def run_command(self, command: list[str]) -> None:
        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
            universal_newlines=True
        )

        def read_output(pipe, queue):
            for line in pipe:
                queue.put(line.strip())
            pipe.close()

        stdout_thread = threading.Thread(target=read_output, args=(process.stdout, self.message_queue))
        stderr_thread = threading.Thread(target=read_output, args=(process.stderr, self.message_queue))

        stdout_thread.daemon = True
        stderr_thread.daemon = True

        stdout_thread.start()
        stderr_thread.start()

        process.wait()
        stdout_thread.join()
        stderr_thread.join()

    def build_project(self):
        self.run_command(["cargo", "build"])

    def run_cargo_command(self, command: str, additional_args: list[str] = None):
        cmd = ["cargo", command]
        if additional_args:
            cmd.extend(additional_args)

        if command == "test":
            self.test_text.delete(1.0, tk.END)  # Clear previous test output

        thread = threading.Thread(target=lambda: self.run_command(cmd))
        thread.daemon = True
        thread.start()

def main():
    root = tk.Tk()
    app = RustProjectManager(root)
    root.mainloop()

if __name__ == "__main__":
    main()