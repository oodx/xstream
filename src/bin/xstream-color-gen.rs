// XStream Color Generator Binary
// Specialized tool for generating pre-colored test streams
// Usage: cargo run --bin xstream-color-gen --theme rainbow --count 10

use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "xstream-color-gen")]
#[command(about = "Specialized color stream generator for XStream testing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate themed color streams
    Theme {
        /// Color theme (rainbow|warm|cool|mono|neon|pastel|earth)
        #[arg(long, default_value = "rainbow")]
        theme: String,
        
        /// Number of colored tokens to generate
        #[arg(long, default_value = "8")]
        count: usize,
        
        /// Output format (tokens|blocks|gradient)
        #[arg(long, default_value = "tokens")]
        format: String,
    },
    
    /// Generate namespace-colored streams
    Namespace {
        /// Comma-separated namespaces to color
        #[arg(long, default_value = "ui,db,api")]
        namespaces: String,
        
        /// Tokens per namespace
        #[arg(long, default_value = "3")]
        tokens: usize,
        
        /// Include visual symbols
        #[arg(long, default_value = "false")]
        symbols: bool,
    },
    
    /// Generate color gradients for pipeline visualization
    Gradient {
        /// Starting color
        #[arg(long, required = true)]
        start: String,
        
        /// Ending color
        #[arg(long, required = true)]
        end: String,
        
        /// Number of gradient steps
        #[arg(long, default_value = "5")]
        steps: usize,
    },
    
    /// Generate color palettes for testing
    Palette {
        /// Palette type (web|terminal|ansi|rgb)
        #[arg(long, default_value = "terminal")]
        palette: String,
        
        /// Include color codes in output
        #[arg(long, default_value = "false")]
        codes: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Theme { theme, count, format } => {
            let result = generate_themed_stream(theme, *count, format);
            println!("{}", result);
        }
        Commands::Namespace { namespaces, tokens, symbols } => {
            let ns_list: Vec<&str> = namespaces.split(',').collect();
            let result = if *symbols {
                generate_symbol_colored_stream(&ns_list, *tokens)
            } else {
                generate_namespace_colored_stream(&ns_list, *tokens)
            };
            println!("{}", result);
        }
        Commands::Gradient { start, end, steps } => {
            let result = generate_color_gradient(start, end, *steps);
            println!("{}", result);
        }
        Commands::Palette { palette, codes } => {
            let result = generate_color_palette(palette, *codes);
            println!("{}", result);
        }
    }
}

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