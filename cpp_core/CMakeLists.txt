cmake_minimum_required(VERSION 3.10)
project(OmniTrace)

# 1. Essential for Windows DLLs to export symbols automatically
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

# 2. Include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# 3. Define the library with all source files
add_library(omni SHARED 
    library.cpp
    src/gpu_state.cpp
    src/sm_banks.cpp
    src/tensor_core.cpp
)

# 4. The Install Target
# Windows DLLs go to 'bin', Import Libs go to 'lib'
install(TARGETS omni
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)
