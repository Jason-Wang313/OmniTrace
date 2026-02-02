
# OmniTrace: Hybrid C++/Rust GPU Simulator

**A cycle-accurate GPU simulator investigating the physics of H100 Streaming Multiprocessors.**

## 🏗 Architecture
This project uses a "Layer Cake" architecture to combine performance with safety:
* **Core (C++20):** Simulates warp physics, bank conflicts, and tensor core latency.
* **Interface (Rust):** Provides memory safety, FFI bindings, and parallelized CLI tooling.
* **Agent (Python):** An AI optimizer that generates PTX kernels to stress-test the micro-architecture.

## 🚀 Performance Proof
The simulator successfully models **Shared Memory Bank Conflicts**.
* **Unoptimized Kernel (Stride 32):** 1024 Cycles (Massive Stalls)
* **Optimized Kernel (Stride 1):** 32 Cycles (Perfect Parallelism)
* **Speedup:** 32.0x ⚡

## 🛠 Usage
1.  **Build:** \`cd rust_tooling && cargo build --release\`
2.  **Test:** \`cargo test\`
3.  **Run Agent:** \`python python_agent/agent.py\`

## 📂 Project Structure
* \`cpp_core/\`: The simulation engine (CMake).
* \`rust_tooling/\`: The CLI and parser (Cargo).
* \`python_agent/\`: The optimization logic.
"@ | Out-File -Encoding utf8 README.md
