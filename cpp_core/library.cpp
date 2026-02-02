#include "omni_api.h"
#include <vector>

// Legacy debug function (keep for backwards compatibility)
extern "C" int debug_gpu(int id) {
    return id * 42;
}

// Main trace simulation function
// Decodes instruction opcodes and accumulates simulated latency
// Opcode format: opcode | (stride << 8)
//   - Lower 8 bits: opcode (1=LD.SHARED, 2=MMA.SYNC, 0=ALU)
//   - Upper bits: stride for LD.SHARED operations
// Opcode 1: LD.SHARED - uses bank conflict detection with stride
// Opcode 2: MMA.SYNC - tensor core operation (16 cycles)
// Default: ALU operation (1 cycle)
extern "C" uint32_t simulate_trace(const uint32_t* instructions, int length) {
    uint32_t total_latency = 0;
    SharedMemory shared_mem;
    
    for (int i = 0; i < length; i++) {
        uint32_t encoded = instructions[i];
        uint32_t opcode = encoded & 0xFF;        // Lower 8 bits
        uint32_t stride = (encoded >> 8) & 0xFF; // Next 8 bits
        if (stride == 0) stride = 1;             // Default stride
        
        switch (opcode) {
            case 1: { // LD.SHARED
                // Generate addresses for 32 threads with given stride
                // address = thread_id * stride * 4
                std::vector<uint32_t> addresses;
                for (int t = 0; t < 32; t++) {
                    addresses.push_back(t * stride * 4);
                }
                total_latency += shared_mem.check_conflicts(addresses);
                break;
            }
            case 2: // MMA.SYNC
                total_latency += simulate_mma_sync();
                break;
            default: // ALU or other
                total_latency += 1;
                break;
        }
    }
    
    return total_latency;
}
