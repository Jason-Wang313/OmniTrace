//! OmniTrace GPU Simulator Library
//!
//! Provides PTX parsing and GPU trace simulation through C++ FFI.

pub mod ffi;
pub mod cost_model;

pub use ffi::Simulator;
pub use cost_model::parse_ptx;

/// Run a complete simulation from PTX text to latency result.
///
/// This is the main entry point for the library. It parses the PTX,
/// runs the C++ simulation, and returns the total latency.
///
/// # Arguments
/// * `ptx` - PTX assembly text (one instruction per line)
///
/// # Returns
/// Total simulated latency in cycles.
pub fn run_simulation(ptx: &str) -> u64 {
    let sim = Simulator::new();
    let opcodes = parse_ptx(ptx);
    sim.simulate(&opcodes) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        // Two ALU operations = 2 cycles
        let latency = run_simulation("ADD\nADD");
        assert_eq!(latency, 2, "Two ADD instructions should take 2 cycles");
    }

    #[test]
    fn test_tensor_core() {
        // MMA.SYNC = 16 cycles (H100 tensor core latency)
        let latency = run_simulation("MMA.SYNC");
        assert_eq!(latency, 16, "MMA.SYNC should take 16 cycles");
    }

    #[test]
    fn test_mixed() {
        // LD.SHARED (stride=1, no conflicts = 1 cycle) + MMA.SYNC (16 cycles) = 17 cycles
        let latency = run_simulation("LD.SHARED\nMMA.SYNC");
        assert_eq!(latency, 17, "LD.SHARED + MMA.SYNC should take 17 cycles");
    }
}
