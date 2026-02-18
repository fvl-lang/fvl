use fvl_parser::submitter::Submitter;

fn main() {
    println!("=== FVL State Root Submitter ===");

    let mut submitter = match Submitter::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize submitter: {}", e);
            eprintln!("   Make sure data/contract.json exists");
            eprintln!("   Run: bash contracts/deploy.sh");
            std::process::exit(1);
        }
    };

    submitter.run();
}