//! Foreign Function Interface for C++ GPU simulator
//! 
//! Provides safe Rust wrappers around unsafe C++ calls.

extern "C" {
    fn debug_gpu(id: libc::c_int) -> libc::c_int;
    fn simulate_trace(instructions: *const u32, length: libc::c_int) -> u32;
}

/// Safe wrapper around the C++ GPU simulator.
/// 
/// Implements Drop to ensure proper cleanup (future-proofing for 
/// when we allocate dynamic C++ memory).
pub struct Simulator {
    _initialized: bool,
}

impl Simulator {
    /// Create a new simulator instance.
    pub fn new() -> Self {
        Simulator { _initialized: true }
    }

    /// Run the legacy debug function (for testing FFI).
    pub fn debug(&self, id: i32) -> i32 {
        unsafe { debug_gpu(id) }
    }

    /// Simulate a GPU trace and return total latency in cycles.
    /// 
    /// # Arguments
    /// * `instructions` - Slice of opcodes (1=LD.SHARED, 2=MMA.SYNC, 0=ALU)
    /// 
    /// # Returns
    /// Total simulated latency in cycles.
    pub fn simulate(&self, instructions: &[u32]) -> u32 {
        unsafe { 
            simulate_trace(instructions.as_ptr(), instructions.len() as libc::c_int) 
        }
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        // Future: Call C++ cleanup functions here if we allocate memory
        // For now, this is a placeholder for good practice.
        self._initialized = false;
    }
}
