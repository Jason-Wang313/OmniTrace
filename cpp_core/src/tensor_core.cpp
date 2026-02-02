#include "omni_api.h"

// Simulates H100 Tensor Core Matrix Multiply-Accumulate latency
// Returns 16 cycles (typical MMA.SYNC latency on H100)
int simulate_mma_sync() {
    return 16;
}
