use cmake::Config;

fn main() {
    // Re-run if C++ files change
    println!("cargo:rerun-if-changed=../cpp_core/library.cpp");
    println!("cargo:rerun-if-changed=../cpp_core/CMakeLists.txt");

    // Build the C++ library using cmake
    let dst = Config::new("../cpp_core").build();

    // Tell cargo to link the static library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=omni");

    // Link C++ standard library on Windows (MSVC)
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=msvcrt");
    }

    // Link C++ standard library on Unix
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
    }

    // Copy the DLL to the project root so the executable can find it
    let dll_path = dst.join("bin/omni.dll");
    let target_path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/omni.dll";

    println!("cargo:warning=Copying DLL from {:?} to {:?}", dll_path, target_path);
    std::fs::copy(dll_path, target_path).expect("Failed to copy DLL to root!");
}
