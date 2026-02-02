//! PTX Cost Model
//! 
//! Parses PTX assembly text and converts instructions to opcodes
//! for the GPU simulator.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Match LD.SHARED with optional stride: "LD.SHARED" or "LD.SHARED:32"
    static ref LD_SHARED_RE: Regex = Regex::new(r"(?i)LD\.SHARED(?::(\d+))?").unwrap();
    static ref MMA_SYNC_RE: Regex = Regex::new(r"(?i)MMA\.SYNC").unwrap();
}

/// Parse PTX assembly text and extract instruction opcodes.
/// 
/// # Opcode Encoding
/// Encoded as: `opcode | (stride << 8)`
/// - `LD.SHARED` -> 1 (shared memory load), stride encoded in upper bits
/// - `LD.SHARED:N` -> 1 with stride N
/// - `MMA.SYNC` -> 2 (tensor core matrix multiply)
/// - Others -> 0 (generic ALU operation)
/// 
/// # Arguments
/// * `ptx` - PTX assembly text (one instruction per line)
/// 
/// # Returns
/// Vector of encoded opcodes suitable for the simulator.
pub fn parse_ptx(ptx: &str) -> Vec<u32> {
    ptx.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            
            // Check for LD.SHARED with optional stride
            if let Some(caps) = LD_SHARED_RE.captures(line) {
                let stride: u32 = caps.get(1)
                    .map(|m| m.as_str().parse().unwrap_or(1))
                    .unwrap_or(1);
                // Encode: opcode=1, stride in upper bits
                1 | (stride << 8)
            } else if MMA_SYNC_RE.is_match(line) {
                2 // MMA.SYNC
            } else {
                0 // Default ALU
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ptx() {
        let ptx = "LD.SHARED\nMMA.SYNC\nADD\n";
        let opcodes = parse_ptx(ptx);
        assert_eq!(opcodes, vec![1 | (1 << 8), 2, 0]);
    }

    #[test]
    fn test_parse_with_stride() {
        let ptx = "LD.SHARED:32\n";
        let opcodes = parse_ptx(ptx);
        assert_eq!(opcodes, vec![1 | (32 << 8)]);
    }
}
