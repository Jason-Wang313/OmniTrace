use std::fs;
use std::env;

fn main() {
    // Get filename from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <ptx_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    
    // Read PTX file
    let ptx_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Run simulation via library
    let latency = rust_tooling::run_simulation(&ptx_code);

    // Output ONLY JSON
    println!("{}", serde_json::json!({ "latency": latency }));
}
