// XStream Generator Binary
// Generate colored token streams for testing and demonstrations
// Usage: cargo run --bin xstream-gen colored --namespaces ui,db,api --tokens 5

use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "xstream-gen")]
#[command(about = "XStream token stream generator for testing and demonstrations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate colored token streams
    Colored {
        /// Comma-separated list of namespaces (e.g., ui,db,api)
        #[arg(long, default_value = "ui,db,api")]
        namespaces: String,
        
        /// Number of tokens per namespace
        #[arg(long, default_value = "3")]
        tokens: usize,
        
        /// Output format (stream|fork|merge)
        #[arg(long, default_value = "stream")]
        format: String,
        
        /// Use color block symbols
        #[arg(long, default_value = "false")]
        symbols: bool,
    },
    
    /// Generate pre-colored test streams
    PreColored {
        /// Total number of tokens to generate
        #[arg(long, default_value = "10")]
        count: usize,
        
        /// Color theme (rainbow|mono|warm|cool)
        #[arg(long, default_value = "rainbow")]
        theme: String,
    },
    
    /// Generate streams for specific patterns (fork-ready, merge-ready, etc.)
    Pattern {
        /// Pattern type (fork|merge|gate|pipeline)
        #[arg(long, required = true)]
        pattern: String,
        
        /// Complexity level (simple|medium|complex)
        #[arg(long, default_value = "medium")]
        complexity: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Colored { namespaces, tokens, format, symbols } => {
            let ns_list: Vec<&str> = namespaces.split(',').collect();
            let result = generate_colored_stream(&ns_list, *tokens, format, *symbols);
            println!("{}", result);
        }
        Commands::PreColored { count, theme } => {
            let result = generate_pre_colored_tokens(*count, theme);
            println!("{}", result);
        }
        Commands::Pattern { pattern, complexity } => {
            let result = generate_pattern_stream(pattern, complexity);
            println!("{}", result);
        }
    }
}

fn generate_colored_stream(namespaces: &[&str], tokens_per_ns: usize, format: &str, symbols: bool) -> String {
    let mut result = Vec::new();
    
    // Define color mappings for namespaces
    let color_map = get_namespace_colors();
    
    match format {
        "fork" => {
            // Generate stream suitable for fork testing
            for (i, &namespace) in namespaces.iter().enumerate() {
                let color_prefix = color_map.get(namespace).unwrap_or(&"def");
                let mut namespace_tokens = Vec::new();
                
                for token_num in 1..=tokens_per_ns {
                    let symbol = if symbols { get_block_symbol(i) } else { "" };
                    let token = format!(
                        "{}:{}{}=\"{}{}\"",
                        namespace,
                        symbol,
                        format!("item{:02}", token_num),
                        color_prefix,
                        format!("{:02}", token_num)
                    );
                    namespace_tokens.push(token);
                }
                result.push(namespace_tokens.join("; "));
            }
            result.join("\n")
        }
        "merge" => {
            // Generate separate streams for merge testing
            for (i, &namespace) in namespaces.iter().enumerate() {
                let color_prefix = color_map.get(namespace).unwrap_or(&"def");
                let mut namespace_tokens = Vec::new();
                
                for token_num in 1..=tokens_per_ns {
                    let symbol = if symbols { get_block_symbol(i) } else { "" };
                    let token = format!(
                        "{}:{}{}=\"{}{}\"",
                        namespace,
                        symbol,
                        format!("val{:02}", token_num),
                        color_prefix,
                        format!("{:02}", token_num)
                    );
                    namespace_tokens.push(token);
                }
                result.push(format!("{}: {}", namespace, namespace_tokens.join("; ")));
            }
            result.join("\n")
        }
        "stream" | _ => {
            // Generate single mixed stream
            for &namespace in namespaces {
                let color_prefix = color_map.get(namespace).unwrap_or(&"def");
                
                for token_num in 1..=tokens_per_ns {
                    let symbol = if symbols { get_block_symbol(namespaces.iter().position(|&x| x == namespace).unwrap_or(0)) } else { "" };
                    let token = format!(
                        "{}:{}{}=\"{}{}\"",
                        namespace,
                        symbol,
                        format!("data{:02}", token_num),
                        color_prefix,
                        format!("{:02}", token_num)
                    );
                    result.push(token);
                }
            }
            result.join("; ")
        }
    }
}

fn generate_pre_colored_tokens(count: usize, theme: &str) -> String {
    let colors = match theme {
        "rainbow" => vec!["red", "orange", "yellow", "green", "blue", "purple", "pink"],
        "mono" => vec!["grey", "black", "white"],
        "warm" => vec!["red", "orange", "yellow", "pink", "brown"],
        "cool" => vec!["blue", "cyan", "green", "purple", "teal"],
        _ => vec!["red", "blue", "green", "yellow"],
    };
    
    let mut result = Vec::new();
    
    for i in 0..count {
        let color = colors[i % colors.len()];
        let token = format!(
            "color:item{:02}=\"{}{}\"",
            i + 1,
            color,
            format!("{:02}", (i % 99) + 1)
        );
        result.push(token);
    }
    
    result.join("; ")
}

fn generate_pattern_stream(pattern: &str, complexity: &str) -> String {
    match pattern {
        "fork" => generate_fork_pattern(complexity),
        "merge" => generate_merge_pattern(complexity),
        "gate" => generate_gate_pattern(complexity),
        "pipeline" => generate_pipeline_pattern(complexity),
        _ => "Invalid pattern. Use: fork, merge, gate, or pipeline".to_string(),
    }
}

fn generate_fork_pattern(complexity: &str) -> String {
    match complexity {
        "simple" => {
            "ui:btn=\"click\"; ui:theme=\"dark\"; db:host=\"localhost\"; db:port=\"3306\"; api:status=\"ok\"".to_string()
        }
        "complex" => {
            let namespaces = vec!["ui.widgets", "ui.layout", "db.conn", "db.query", "api.auth", "api.data", "log.info", "log.error"];
            let mut tokens = Vec::new();
            
            for (i, ns) in namespaces.iter().enumerate() {
                for j in 1..=3 {
                    let token = format!("{}:item{}=\"val{:02}\"", ns, j, i * 10 + j);
                    tokens.push(token);
                }
            }
            tokens.join("; ")
        }
        _ => { // medium
            "ui:click=\"btn1\"; ui:hover=\"btn2\"; ui:focus=\"input1\"; db:query=\"users\"; db:conn=\"pool1\"; api:get=\"/data\"; api:post=\"/submit\"; log:level=\"info\"".to_string()
        }
    }
}

fn generate_merge_pattern(complexity: &str) -> String {
    match complexity {
        "simple" => {
            "ui: ui:btn=\"click\"; ui:theme=\"dark\"\ndb: db:host=\"localhost\"; db:port=\"3306\"\napi: api:status=\"ok\"; api:version=\"v1\"".to_string()
        }
        "complex" => {
            let streams = vec![
                "ui.primary: ui.primary:header=\"main\"; ui.primary:sidebar=\"left\"; ui.primary:content=\"body\"",
                "ui.secondary: ui.secondary:footer=\"info\"; ui.secondary:modal=\"popup\"; ui.secondary:tooltip=\"help\"", 
                "db.read: db.read:select=\"users\"; db.read:join=\"roles\"; db.read:where=\"active\"",
                "db.write: db.write:insert=\"user\"; db.write:update=\"profile\"; db.write:delete=\"session\"",
                "api.v1: api.v1:get=\"/users\"; api.v1:post=\"/auth\"; api.v1:put=\"/data\"",
                "api.v2: api.v2:graphql=\"/gql\"; api.v2:rest=\"/api\"; api.v2:websocket=\"/ws\"",
            ];
streams.join("\n")
        }
        _ => { // medium
            "ui: ui:click=\"btn1\"; ui:hover=\"btn2\"; ui:focus=\"input1\"\ndb: db:query=\"users\"; db:conn=\"pool1\"; db:cache=\"redis\"\napi: api:get=\"/data\"; api:post=\"/submit\"; api:auth=\"token\"".to_string()
        }
    }
}

fn generate_gate_pattern(complexity: &str) -> String {
    match complexity {
        "simple" => {
            "ui:ready=\"true\"; db:connected=\"true\"; api:auth=\"valid\"".to_string()
        }
        "complex" => {
            let mut tokens = Vec::new();
            
            // High priority tokens
            for i in 1..=5 {
                tokens.push(format!("priority:high=\"task{}\"", i));
            }
            
            // Medium priority tokens  
            for i in 1..=3 {
                tokens.push(format!("priority:medium=\"work{}\"", i));
            }
            
            // Low priority tokens
            for i in 1..=2 {
                tokens.push(format!("priority:low=\"job{}\"", i));
            }
            
            // Auth namespace tokens
            for i in 1..=4 {
                tokens.push(format!("auth:token{}=\"valid{}\"", i, i));
            }
            
tokens.join("; ")
        }
        _ => { // medium
            "ui:ready=\"true\"; ui:loading=\"false\"; db:connected=\"true\"; db:pool=\"available\"; api:auth=\"valid\"; api:rate=\"ok\"; priority:high=\"urgent\"".to_string()
        }
    }
}

fn generate_pipeline_pattern(complexity: &str) -> String {
    match complexity {
        "simple" => {
            "input:data=\"raw\"; transform:clean=\"processed\"; output:result=\"final\"".to_string()
        }
        "complex" => {
            let stages = vec![
                "input.raw:file=\"data.csv\"; input.raw:format=\"csv\"; input.raw:size=\"1024\"",
                "validate:schema=\"ok\"; validate:types=\"pass\"; validate:nulls=\"clean\"", 
                "transform:normalize=\"done\"; transform:enrich=\"added\"; transform:filter=\"applied\"",
                "aggregate:sum=\"total\"; aggregate:count=\"records\"; aggregate:avg=\"computed\"",
                "output.staging:write=\"temp\"; output.staging:index=\"created\"; output.staging:backup=\"saved\"",
                "output.prod:deploy=\"live\"; output.prod:monitor=\"active\"; output.prod:alert=\"ready\"",
            ];
stages.join("; ")
        }
        _ => { // medium
            "input:file=\"data.json\"; parse:json=\"object\"; validate:schema=\"pass\"; transform:map=\"array\"; filter:valid=\"items\"; output:write=\"result.json\"".to_string()
        }
    }
}

fn get_namespace_colors() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("ui", "blue");
    map.insert("db", "green"); 
    map.insert("api", "yellow");
    map.insert("auth", "red");
    map.insert("log", "grey");
    map.insert("cache", "cyan");
    map.insert("queue", "purple");
    map.insert("file", "brown");
    map.insert("net", "orange");
    map.insert("sys", "pink");
    map
}

fn get_block_symbol(index: usize) -> &'static str {
    let symbols = ["■", "▲", "●", "♦", "★", "▼", "◆", "♠"];
    symbols[index % symbols.len()]
}