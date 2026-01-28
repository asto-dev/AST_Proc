# ASTProc

A fast, **pretty** and minimal **process tree viewer** written in Rust using a terminal UI.
This project was done for learning/entertainment purposes but feel free if you'd like to use it for yourself!!! :P 


ASTProc reads the Linux `/proc` filesystem and displays running processes in a hierarchical parent → child tree, similar to `pstree` or `htop`, but inside a clean interactive TUI powered by **ratatui**.

---

## Features

- Real-time process tree
- Parent → child hierarchy
- Shows:
  - PID
  - process name
  - parent PID
  - executable path
- Auto refresh every second
- Scrollable (vertical + horizontal)
- Lightweight (no external dependencies beyond `/proc`)
- Pure Rust

---

## 📸 Preview

---

![demo](assets/demo.gif)


---

## Installation

### Requirements

- Linux
- Rust (stable)
- Cargo

Install Rust: curl https://sh.rustup.rs -sSf | sh


---

### Build

git clone https://github.com/asto-dev/AST_Proc
cd astproc
cargo build --release

I recommend changing the project title to anything you want in the cargo.toml file

---

## Controls

| Key | Action |
|-----|---------|
| q | Quit |
| j / ↓ | Scroll down |
| k / ↑ | Scroll up |
| h / ← | Scroll left |
| l / → | Scroll right |

---

## How it works?

1. Reads `/proc`
2. Parses numeric directories as PIDs
3. Extracts:
   - `/proc/<pid>/comm`
   - `/proc/<pid>/status`
   - `/proc/<pid>/exe`
4. Builds a `HashMap<u32, Process>`
5. Constructs a parent → children tree
6. Renders the tree using ratatui

Refresh happens every second.

---

## Dependencies

- ratatui
- crossterm
- std

---