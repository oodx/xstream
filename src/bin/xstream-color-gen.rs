// XStream Color Generator Binary
// Specialized tool for generating pre-colored test streams
// Usage: cargo run --bin xstream-color-gen theme --theme=rainbow --count=10

use rsb::prelude::*;
use std::collections::HashMap;

fn main() {
    let args = bootstrap!();
    
    if args.len() < 2 {
        let empty_args = vec![];
        show_help(Args::new(&empty_args));
        return;
    }
    
    let result = match args[1].as_str() {
        "theme" => handle_theme_command(Args::new(&args)),
        "namespace" => handle_namespace_command(Args::new(&args)),
        "gradient" => handle_gradient_command(Args::new(&args)),
        "palette" => handle_palette_command(Args::new(&args)),
        "help" => show_help(Args::new(&args)),
        _ => {
            println!("Unknown command: {}. Use 'help' for usage information.", args[1]);
            let empty_args = vec![];
            show_help(Args::new(&empty_args))
        }
    };
    std::process::exit(result);
}

fn show_help(_args: Args) -> i32 {
    println!("Specialized color stream generator for XStream testing");
    println!("");
    println!("USAGE:");
    println!("    xstream-color-gen <COMMAND> [OPTIONS]");
    println!("");
    println!("COMMANDS:");
    println!("    theme       Generate themed color streams");
    println!("    namespace   Generate namespace-colored streams");
    println!("    gradient    Generate color gradients for pipeline visualization");
    println!("    palette     Generate color palettes for testing");
    println!("    help        Show this help message");
    println!("");
    println!("THEME OPTIONS:");
    println!("    --theme=<THEME>    Color theme: rainbow|warm|cool|mono|neon|pastel|earth (default: rainbow)");
    println!("    --count=<N>        Number of colored tokens to generate (default: 8)");
    println!("    --format=<FORMAT>  Output format: tokens|blocks|gradient (default: tokens)");
    println!("");
    println!("NAMESPACE OPTIONS:");
    println!("    --namespaces=<ns1,ns2>  Comma-separated namespaces to color (default: ui,db,api)");
    println!("    --tokens=<N>           Tokens per namespace (default: 3)");
    println!("    --symbols=<BOOL>       Include visual symbols (default: false)");
    println!("");
    println!("GRADIENT OPTIONS:");
    println!("    --start=<COLOR>       Starting color (required)");
    println!("    --end=<COLOR>         Ending color (required)");
    println!("    --steps=<N>           Number of gradient steps (default: 5)");
    println!("");
    println!("PALETTE OPTIONS:");
    println!("    --palette=<TYPE>      Palette type: web|terminal|ansi|rgb (default: terminal)");
    println!("    --codes=<BOOL>        Include color codes in output (default: false)");
    0
}

fn handle_theme_command(mut args: Args) -> i32 {
    let theme = args.get_kv("theme").unwrap_or("rainbow".to_string());
    let count = args.get_kv("count").unwrap_or("8".to_string()).parse::<usize>().unwrap_or(8);
    let format = args.get_kv("format").unwrap_or("tokens".to_string());
    
    let result = generate_themed_stream(&theme, count, &format);
    println!("{}", result);
    0
}

fn handle_namespace_command(mut args: Args) -> i32 {
    let namespaces = args.get_kv("namespaces").unwrap_or("ui,db,api".to_string());
    let tokens = args.get_kv("tokens").unwrap_or("3".to_string()).parse::<usize>().unwrap_or(3);
    let symbols = args.get_kv("symbols").unwrap_or("false".to_string()) == "true";
    
    let ns_list: Vec<&str> = namespaces.split(',').collect();
    let result = if symbols {
        generate_symbol_colored_stream(&ns_list, tokens)
    } else {
        generate_namespace_colored_stream(&ns_list, tokens)
    };
    println!("{}", result);
    0
}

fn handle_gradient_command(mut args: Args) -> i32 {
    let start = match args.get_kv("start") {
        Some(s) => s,
        None => {
            println!("Error: --start is required for gradient command");
            return 1;
        }
    };
    let end = match args.get_kv("end") {
        Some(e) => e,
        None => {
            println!("Error: --end is required for gradient command");
            return 1;
        }
    };
    let steps = args.get_kv("steps").unwrap_or("5".to_string()).parse::<usize>().unwrap_or(5);
    
    let result = generate_color_gradient(&start, &end, steps);
    println!("{}", result);
    0
}

fn handle_palette_command(mut args: Args) -> i32 {
    let palette = args.get_kv("palette").unwrap_or("terminal".to_string());
    let codes = args.get_kv("codes").unwrap_or("false".to_string()) == "true";
    
    let result = generate_color_palette(&palette, codes);
    println!("{}", result);
    0
}

// Removed parse_arg function - using RSB Args methods instead

fn generate_themed_stream(theme: &str, count: usize, format: &str) -> String {
    let color_set = get_theme_colors(theme);
    
    match format {
        "blocks" => {
            let mut blocks = Vec::new();
            for i in 0..count {
                let color = &color_set[i % color_set.len()];
                let block = format!("block:{}=\"{}■\"", i + 1, color);
                blocks.push(block);
            }
            blocks.join("; ")
        }
        "gradient" => {
            let mut gradient = Vec::new();
            for i in 0..count {
                let color = &color_set[i % color_set.len()];
                let step = format!("gradient:step{:02}=\"{}{}\"", 
                    i + 1, color, "█".repeat((i % 5) + 1));
                gradient.push(step);
            }
            gradient.join("; ")
        }
        "tokens" | _ => {
            let mut tokens = Vec::new();
            for i in 0..count {
                let color = &color_set[i % color_set.len()];
                let token = format!("theme:color{:02}=\"{}{:02}\"", i + 1, color, i + 1);
                tokens.push(token);
            }
            tokens.join("; ")
        }
    }
}

fn generate_namespace_colored_stream(namespaces: &[&str], tokens_per_ns: usize) -> String {
    let color_map = get_namespace_color_map();
    let mut all_tokens = Vec::new();
    
    for namespace in namespaces {
        let color_prefix = color_map.get(namespace).unwrap_or(&"default");
        
        for token_num in 1..=tokens_per_ns {
            let token = format!(
                "{}:item{:02}=\"{}{:02}\"",
                namespace,
                token_num,
                color_prefix,
                token_num
            );
            all_tokens.push(token);
        }
    }
    
    all_tokens.join("; ")
}

fn generate_symbol_colored_stream(namespaces: &[&str], tokens_per_ns: usize) -> String {
    let symbols = ["■", "▲", "●", "♦", "★", "▼", "◆", "♠"];
    let color_map = get_namespace_color_map();
    let mut all_tokens = Vec::new();
    
    for (ns_idx, namespace) in namespaces.iter().enumerate() {
        let color_prefix = color_map.get(namespace).unwrap_or(&"default");
        let symbol = symbols[ns_idx % symbols.len()];
        
        for token_num in 1..=tokens_per_ns {
            let token = format!(
                "{}:{}item{:02}=\"{}{:02}\"",
                namespace,
                symbol,
                token_num,
                color_prefix,
                token_num
            );
            all_tokens.push(token);
        }
    }
    
    all_tokens.join("; ")
}

fn generate_color_gradient(start_color: &str, end_color: &str, steps: usize) -> String {
    let mut gradient_tokens = Vec::new();
    
    // Simple gradient simulation using color names
    let start_intensity = get_color_intensity(start_color);
    let end_intensity = get_color_intensity(end_color);
    
    for step in 0..steps {
        let progress = step as f32 / (steps - 1) as f32;
        let intensity = start_intensity + (end_intensity - start_intensity) * progress;
        
        let gradient_color = if intensity < 0.33 {
            start_color
        } else if intensity < 0.67 {
            "mixed"
        } else {
            end_color
        };
        
        let token = format!(
            "gradient:step{:02}=\"{}{:02}\"",
            step + 1,
            gradient_color,
            ((progress * 99.0) as usize).min(99)
        );
        gradient_tokens.push(token);
    }
    
    gradient_tokens.join("; ")
}

fn generate_color_palette(palette_type: &str, include_codes: bool) -> String {
    let colors = match palette_type {
        "web" => vec!["red", "green", "blue", "yellow", "magenta", "cyan", "orange", "purple"],
        "terminal" => vec!["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"],
        "ansi" => vec!["bright_black", "bright_red", "bright_green", "bright_yellow", 
                     "bright_blue", "bright_magenta", "bright_cyan", "bright_white"],
        "rgb" => vec!["rgb_red", "rgb_green", "rgb_blue", "rgb_cyan", "rgb_magenta", "rgb_yellow"],
        _ => vec!["red", "green", "blue", "yellow"], // default
    };
    
    let mut palette_tokens = Vec::new();
    
    for (i, color) in colors.iter().enumerate() {
        let token = if include_codes {
            let code = get_color_code(color);
            format!("palette:{}=\"{}[{}]\"", color, color, code)
        } else {
            format!("palette:{}=\"{}{:02}\"", color, color, i + 1)
        };
        palette_tokens.push(token);
    }
    
    palette_tokens.join("; ")
}

fn get_theme_colors(theme: &str) -> Vec<&'static str> {
    match theme {
        "rainbow" => vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"],
        "warm" => vec!["red", "orange", "yellow", "pink", "brown", "coral"],
        "cool" => vec!["blue", "cyan", "green", "purple", "teal", "navy"],
        "mono" => vec!["black", "grey", "white", "silver"],
        "neon" => vec!["lime", "cyan", "magenta", "yellow", "pink"],
        "pastel" => vec!["light_pink", "light_blue", "light_green", "light_yellow", "lavender"],
        "earth" => vec!["brown", "tan", "forest_green", "gold", "rust"],
        _ => vec!["red", "blue", "green", "yellow"], // default
    }
}

fn get_namespace_color_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("ui", "blue");
    map.insert("db", "green");
    map.insert("api", "yellow");
    map.insert("auth", "red");
    map.insert("cache", "cyan");
    map.insert("log", "grey");
    map.insert("queue", "purple");
    map.insert("file", "brown");
    map.insert("net", "orange");
    map.insert("sys", "pink");
    map.insert("input", "light_grey");
    map.insert("validate", "blue");
    map.insert("transform", "yellow");
    map.insert("aggregate", "green");
    map.insert("output", "purple");
    map
}

fn get_color_intensity(color: &str) -> f32 {
    match color {
        "black" | "dark_grey" => 0.0,
        "grey" | "silver" => 0.3,
        "red" | "green" | "blue" => 0.5,
        "yellow" | "cyan" | "magenta" => 0.7,
        "white" | "bright_white" => 1.0,
        _ => 0.5, // default
    }
}

fn get_color_code(color: &str) -> &'static str {
    match color {
        "black" => "30",
        "red" => "31", 
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "bright_black" => "90",
        "bright_red" => "91",
        "bright_green" => "92",
        "bright_yellow" => "93",
        "bright_blue" => "94",
        "bright_magenta" => "95",
        "bright_cyan" => "96",
        "bright_white" => "97",
        _ => "39", // default
    }
}