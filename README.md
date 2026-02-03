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
