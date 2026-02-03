# ⚡ OmniTrace

> **A cycle-accurate hybrid C++/Rust GPU Simulator investigating the physics of H100 Streaming Multiprocessors.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![C++20](https://img.shields.io/badge/C++-20-blue.svg)](https://en.cppreference.com/w/cpp/20)
[![Python](https://img.shields.io/badge/Python-3.10+-yellow.svg)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 📖 Overview

**OmniTrace** bridges the gap between high-level ML frameworks and low-level hardware execution. By simulating warp physics, SM bank conflicts, and tensor core latency, it provides a rigorous testbed for optimizing GPU kernels before they touch silicon.

The system utilizes a **"Layer Cake" Architecture** to balance raw performance with developer safety:
* **🚀 Core (C++20):** High-performance simulation engine handling warp state, shared memory physics, and execution pipelines.
* **🛡️ Interface (Rust):** Provides memory safety, robust FFI bindings, and a parallelized CLI for managing simulation tasks.
* **🧠 Agent (Python):** An AI-driven optimizer that generates PTX kernels, analyzes latency feedback, and iteratively tunes memory access patterns.

---

## 📊 Performance Proof: Bank Conflict Analysis

The simulator accurately models the massive latency penalties incurred by Shared Memory Bank Conflicts.

| Kernel Strategy | Stride | Latency | Outcome |
| :--- | :---: | :---: | :--- |
| **Unoptimized** | 32 | **1024 Cycles** | ❌ Massive Serialization Stalls |
| **Optimized** | 1 | **32 Cycles** | ✅ Perfect Parallelism |

> **Result:** The optimizer achieves a **32.0x speedup** by realigning memory access patterns to eliminate bank conflicts.

---

## 📂 Project Structure

```text
omnitrace/
├── cpp_core/           # The high-performance simulation engine (CMake)
│   ├── include/        # Public API headers (omni_api.h)
│   └── src/            # Core physics logic (sm_banks.cpp, tensor_core.cpp)
├── rust_tooling/       # The safe CLI wrapper and parser (Cargo)
│   ├── src/            # FFI bindings and command-line logic
│   └── Cargo.toml      # Rust dependency management
└── python_agent/       # The AI optimization logic
    └── agent.py        # Self-optimizing feedback loop script

```

---

## 🚀 Usage

### Prerequisites

* **Rust Toolchain:** `cargo` (for the CLI)
* **C++ Compiler:** `cmake`, `g++` or `clang++` (supporting C++20)
* **Python:** Python 3.10+ (for the agent)

### Quick Start

1. **Build the Simulator:**
Compile the C++ core and Rust bindings in release mode.
```bash
cd rust_tooling
cargo build --release

```


2. **Run the Test Suite:**
Verify the physics engine against known baselines.
```bash
cargo test

```


3. **Launch the AI Agent:**
Run the self-optimizing feedback loop to demonstrate automatic conflict resolution.
```bash
cd ..
python python_agent/agent.py

```



---

## 🧩 Technical Details

### Shared Memory Simulation

The `SharedMemory` class in the C++ core simulates the 32-bank architecture of modern GPUs. It detects when multiple threads within a warp attempt to access different addresses mapping to the same bank, calculating the resulting serialization penalty.

### Warp Physics

The `WarpState` struct maintains the program counter (PC), a simulated register file (32 threads × 64 registers), and an active mask to accurately model divergent execution paths.

---

## 🤝 Contributing

Contributions are welcome! Please focus on:

* **Tensor Core Modeling:** Enhancing the `simulate_mma_sync` logic.
* **Instruction Set:** Expanding the parser to support more PTX instructions.
* **Visualizations:** Improving the reporting of the Python agent.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

```

```
