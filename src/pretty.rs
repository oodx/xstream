// Your pretty XStream visualizer!
use rsb::prelude::*;

fn main() {
    println!("🎨 Welcome to your pretty XStream visualizer!");
    echo!("Loading configuration...");
    
    let args = bootstrap!();
    
    if args.len() < 2 {
        do_default();
        return;
    }
    
    match args[1].as_str() {
        "pretty" => { do_pretty(); }
        _ => {
            println!("Unknown command: {}", args[1]);
            do_default();
        }
    }
}

fn do_default() {
    println!("Ready to make some pretty streams!");
    println!("Usage: cargo run --bin pretty pretty");
}

fn do_pretty() {
    info!("Pretty!");
    println!("Running pretty visualization...");
}
