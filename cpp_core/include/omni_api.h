#ifndef OMNI_API_H
#define OMNI_API_H

#include <cstdint>
#include <vector>

// WarpState: Simulates the state of a GPU warp (32 threads)
struct WarpState {
    uint32_t pc;                          // Program Counter
    std::vector<uint32_t> registers;      // Simulated Register File (32 threads * 64 registers)
    bool active_mask[32];                 // Which threads are active

    WarpState() : pc(0), registers(32 * 64, 0) {
        for (int i = 0; i < 32; i++) active_mask[i] = true;
    }
};

// SharedMemory: Simulates shared memory with bank conflict detection
class SharedMemory {
public:
    int check_conflicts(const std::vector<uint32_t>& addresses);
};

// Tensor Core simulation
int simulate_mma_sync();

// Engine initialization
void init_omni_engine();

// Trace simulation (exported to Rust)
extern "C" uint32_t simulate_trace(const uint32_t* instructions, int length);

#endif // OMNI_API_H
