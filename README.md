# திகழ் IDE (Thigazh IDE)

> **Desktop editor for the திகழ் (Thigazh) Tamil programming language**

[![Framework](https://img.shields.io/badge/framework-Tauri-blue)](https://tauri.app)
[![Backend](https://img.shields.io/badge/backend-Rust-orange)](https://rust-lang.org)
[![Status](https://img.shields.io/badge/status-Working%20Prototype-green)](https://github.com)

---

## 🌟 என்ன இது? (What is this?)

Thigazh IDE is a desktop editor, built with [Tauri](https://tauri.app), for writing and compiling code in **திகழ்** (Thigazh) — a Tamil programming language that compiles to Arduino C.

This IDE is a companion project to the [Thigazh compiler](https://github.com/vasikaran4454/Thigazh-), which does the actual lexing, parsing, type-checking, and code generation.

---

## ✅ What It Provides

- Tamil syntax highlighting (keywords, strings, numbers, functions, variables) in a code editor
- Open / Save `.தி` source files
- One-click **Compile** and **Flash** buttons that call the Thigazh compiler
- Live output panel showing compiler results, generated Arduino C, and flash status

---

## 📁 Project Structure

```
thigazh-ide/
├── dist/
│   ├── index.html       ← Editor UI with Tamil syntax highlighting
│   └── output.html       ← Output/results panel
└── src-tauri/
    ├── src/main.rs        ← Tauri desktop app backend (Rust)
    ├── build.rs
    ├── Cargo.toml
    ├── tauri.conf.json    ← Tauri app configuration
    └── icons/             ← App icons
```

---

## 🔗 Related Project

This IDE is the editor front-end for the **[Thigazh Compiler](https://github.com/vasikaran4454/Thigazh-)**, a Rust-based compiler that translates Tamil source code (`.தி` files) into Arduino C and flashes it to real hardware (Arduino Nano, Uno, Mega, ESP32, and more).

---

## 🚀 How to Run

**Requirements:**
- [Rust](https://rust-lang.org) 1.75+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)
- Node.js (for Tauri tooling)
- The [Thigazh compiler](https://github.com/vasikaran4454/Thigazh-) available on your system

**Setup:**
```bash
cd thigazh-ide
cargo install tauri-cli
cargo tauri dev
```

**Build a release binary:**
```bash
cargo tauri build
```

---

## ⚠️ Honest Status

- Working prototype — editor, syntax highlighting, and compile/flash buttons function
- Depends on the separate [Thigazh compiler](https://github.com/vasikaran4454/Thigazh-) being installed and available
- No autocomplete, debugger, or advanced IDE features yet
- Tested primarily on Windows

---

## 👨‍💻 Creator

**Vasikaran V (Joy)**
Tech Solutions Engineer
Founder — [Time Boundary](https://timeboundary.in)
Tamil Nadu, India 🇮🇳

---

> *"Tamil civilization's entry into the foundation of computing."*
