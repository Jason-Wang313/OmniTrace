#!/usr/bin/env python3
"""
OmniTrace Self-Optimizing Agent
Demonstrates GPU bank conflict optimization through the Rust/C++ simulator.
"""

import subprocess
import json
import os
import sys

# Try to import rich for professional visuals
try:
    from rich.console import Console
    from rich.theme import Theme
    custom_theme = Theme({
        "info": "cyan",
        "warning": "yellow",
        "error": "bold red",
        "success": "bold green",
        "hacker": "bold green",
    })
    console = Console(theme=custom_theme)
    print_function = console.print
except ImportError:
    # Fallback if rich is not installed
    class MockConsole:
        def print(self, text, style=None):
            if style == "bold green" or style == "hacker":
                print(f"\033[92m{text}\033[0m")
            elif style == "bold red" or style == "error":
                print(f"\033[91m{text}\033[0m")
            elif style == "yellow" or style == "warning":
                print(f"\033[93m{text}\033[0m")
            elif style == "cyan" or style == "info":
                print(f"\033[96m{text}\033[0m")
            else:
                print(text)
    
    console = MockConsole()
    print_function = console.print

# Path configuration
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
RUST_TOOLING_DIR = os.path.join(SCRIPT_DIR, "..", "rust_tooling")
TEMP_PTX = os.path.join(RUST_TOOLING_DIR, "temp.ptx")

def generate_ptx(stride: int) -> None:
    """
    Generate a PTX file with 32 LD.SHARED instructions.
    
    Args:
        stride: Memory access stride (1=Fast, 32=Slow/Conflict)
    """
    with open(TEMP_PTX, "w") as f:
        for _ in range(32):
            f.write(f"LD.SHARED:{stride}\n")

def run_simulation(ptx_file: str) -> int:
    """
    Run the Rust simulator and return the latency.
    
    Args:
        ptx_file: Path to the PTX file (relative to rust_tooling dir)
        
    Returns:
        Simulated latency in cycles.
    """
    # Using 'cargo run' handles finding the binary (debug or release) automatically
    # We use --quiet to suppress cargo build output so we just get the JSON
    cmd = ["cargo", "run", "--quiet", "--", ptx_file]
    
    try:
        result = subprocess.run(
            cmd,
            cwd=RUST_TOOLING_DIR,
            capture_output=True,
            text=True,
            check=True
        )
        output = json.loads(result.stdout.strip())
        return output["latency"]
    except subprocess.CalledProcessError as e:
        console.print(f"[error]Simulator failed: {e.stderr}[/error]")
        sys.exit(1)
    except json.JSONDecodeError as e:
        console.print(f"[error]Failed to parse JSON output: {e}[/error]")
        console.print(f"[info]Raw output: {result.stdout}[/info]")
        sys.exit(1)

def main():
    console.print("=" * 60, style="hacker")
    console.print(" OmniTrace: Self-Optimizing Feedback Loop Initialized ", style="hacker")
    console.print("=" * 60, style="hacker")
    print()

    # Step 1: Generate a 'Bad Kernel' with stride=32
    console.print("[info]Step 1: Generating 'Bad Kernel' (Stride=32)...[/info]")
    generate_ptx(stride=32)
    
    # Step 2: Run the simulation
    console.print("[info]Step 2: Profiling kernel...[/info]")
    initial_latency = run_simulation("temp.ptx")
    console.print(f"Initial Latency: [bold]{initial_latency}[/bold] cycles")
    print()
    
    # Step 3: Optimization decision
    if initial_latency > 100:
        console.print(f"[warning]⚠ High Latency Detected ({initial_latency} > 100). Bank Conflicts Likely.[/warning]")
        console.print("[hacker]⚡ Optimizing memory access pattern...[/hacker]")
        print()
        
        # Step 4: Regenerate with stride=1
        console.print("[info]Step 4: Regenerating optimized kernel (Stride=1)...[/info]")
        generate_ptx(stride=1)
        
        # Step 5: Run simulation again
        console.print("[info]Step 5: Profiling optimized kernel...[/info]")
        new_latency = run_simulation("temp.ptx")
        
        # Step 6: Report results
        speedup = initial_latency / new_latency
        console.print(f"New Latency: [hacker]{new_latency}[/hacker] cycles.")
        console.print(f"Speedup: [hacker]{speedup:.1f}x[/hacker] 🚀")
        
        if new_latency < initial_latency:
             console.print("\n[success]✔ Optimization Successful![/success]")
    else:
        console.print("[success]✔ Kernel is already optimized.[/success]")
        
    print()
    console.print("=" * 60, style="hacker")

if __name__ == "__main__":
    main()
