#include "omni_api.h"
#include <algorithm>

// SharedMemory bank conflict detection
// GPU shared memory has 32 banks. Address 'addr' maps to bank (addr / 4) % 32.
// If multiple threads access the same bank in one cycle, they serialize.
// Returns the maximum number of threads accessing the same bank (stall cycles).
int SharedMemory::check_conflicts(const std::vector<uint32_t>& addresses) {
    int bank_access_count[32] = {0};
    
    for (const auto& addr : addresses) {
        int bank = (addr / 4) % 32;
        bank_access_count[bank]++;
    }
    
    // Find the maximum collision count
    int max_collisions = 0;
    for (int i = 0; i < 32; i++) {
        if (bank_access_count[i] > max_collisions) {
            max_collisions = bank_access_count[i];
        }
    }
    
    return max_collisions;
}
