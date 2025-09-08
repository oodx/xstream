// XStream Token Generator using RSB random capabilities

use rsb::prelude::*;
use rand::Rng;

// Word lists for generating realistic tokens
pub const PREFIXES: &[&str] = &[
    "meta", "sec", "admin", "data", "tmpl", "config", "db", "auth", "user", "sys", 
    "app", "api", "cache", "log", "debug", "prod", "dev", "test", "temp", "local"
];

pub const KEY_NAMES: &[&str] = &[
    "key", "user", "colors", "slot", "host", "port", "name", "value", "id", "token",
    "pass", "secret", "url", "path", "file", "dir", "mode", "type", "format", "size",
    "count", "max", "min", "limit", "timeout", "retry", "version", "status", "state"
];

pub const VALUE_WORDS: &[&str] = &[
    "localhost", "admin", "enabled", "disabled", "active", "inactive", "primary", "secondary",
    "production", "development", "staging", "test", "default", "custom", "auto", "manual",
    "true", "false", "yes", "no", "on", "off", "high", "medium", "low", "normal"
];

#[derive(Debug, Clone)]
pub enum ValueType {
    RandomAlnum(usize),    // Random alphanumeric string
    RandomAlpha(usize),    // Random alphabetic string  
    RandomHex(usize),      // Random hex string
    RandomNumber(i32, i32), // Random number in range
    FromList,              // Random from VALUE_WORDS
    Literal(String),       // Exact literal value
}

/// Generate a single random token
pub fn gen_token(prefix: Option<&str>, key_name: Option<&str>, value_type: ValueType) -> String {
    let prefix_str = match prefix {
        Some(p) => p.to_string(),
        None => get_rand_from_slice(&PREFIXES.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "gen".to_string()),
    };
    
    let key_str = match key_name {
        Some(k) => k.to_string(), 
        None => get_rand_from_slice(&KEY_NAMES.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "key".to_string()),
    };
    
    let value_str = match value_type {
        ValueType::RandomAlnum(n) => get_rand_alnum(n),
        ValueType::RandomAlpha(n) => get_rand_alpha(n),
        ValueType::RandomHex(n) => get_rand_hex(n),
        ValueType::RandomNumber(min, max) => {
            let mut rng = rand::rng();
            rng.random_range(min..=max).to_string()
        },
        ValueType::FromList => get_rand_from_slice(&VALUE_WORDS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "default".to_string()),
        ValueType::Literal(s) => s,
    };
    
    format!("{}:{}=\"{}\"", prefix_str, key_str, value_str)
}

/// Generate a flat token (no prefix)
pub fn gen_flat_token(key_name: Option<&str>, value_type: ValueType) -> String {
    let key_str = match key_name {
        Some(k) => k.to_string(),
        None => get_rand_from_slice(&KEY_NAMES.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "key".to_string()),
    };
    
    let value_str = match value_type {
        ValueType::RandomAlnum(n) => get_rand_alnum(n),
        ValueType::RandomAlpha(n) => get_rand_alpha(n),
        ValueType::RandomHex(n) => get_rand_hex(n),
        ValueType::RandomNumber(min, max) => {
            let mut rng = rand::rng();
            rng.random_range(min..=max).to_string()
        },
        ValueType::FromList => get_rand_from_slice(&VALUE_WORDS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "default".to_string()),
        ValueType::Literal(s) => s,
    };
    
    format!("{}=\"{}\"", key_str, value_str)
}

/// Generate a namespace switch token
pub fn gen_ns_token(namespace: Option<&str>) -> String {
    let ns_name = match namespace {
        Some(ns) => ns.to_string(),
        None => get_rand_from_slice(&PREFIXES.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| "config".to_string()),
    };
    
    format!("ns={}", ns_name)
}

/// Generate a pseudo token stream with mixed prefixed/flat tokens
pub fn gen_token_stream(count: usize, flat_ratio: f32) -> String {
    let mut rng = rand::rng();
    let mut tokens = Vec::new();
    
    for _ in 0..count {
        let token = if rng.random::<f32>() < flat_ratio {
            // Generate flat token
            let value_type = match rng.random_range(0..4) {
                0 => ValueType::RandomAlnum(rng.random_range(6..16)),
                1 => ValueType::RandomHex(rng.random_range(8..24)),
                2 => ValueType::FromList,
                _ => ValueType::RandomNumber(1, 9999),
            };
            gen_flat_token(None, value_type)
        } else {
            // Generate prefixed token
            let value_type = match rng.random_range(0..4) {
                0 => ValueType::RandomAlnum(rng.random_range(6..16)),
                1 => ValueType::RandomHex(rng.random_range(8..24)), 
                2 => ValueType::FromList,
                _ => ValueType::RandomNumber(1, 9999),
            };
            gen_token(None, None, value_type)
        };
        tokens.push(token);
    }
    
    tokens.join("; ")
}

/// Generate a realistic config-style stream with namespace switching
pub fn gen_config_stream() -> String {
    let mut tokens = Vec::new();
    
    // Global config
    tokens.push(gen_flat_token(Some("host"), ValueType::Literal("localhost".to_string())));
    tokens.push(gen_flat_token(Some("port"), ValueType::RandomNumber(8000, 9000)));
    tokens.push(gen_flat_token(Some("debug"), ValueType::FromList));
    
    // Database config
    tokens.push(gen_ns_token(Some("db")));
    tokens.push(gen_flat_token(Some("host"), ValueType::Literal("db.example.com".to_string())));
    tokens.push(gen_flat_token(Some("user"), ValueType::RandomAlpha(8)));
    tokens.push(gen_flat_token(Some("pass"), ValueType::RandomHex(32)));
    
    // Auth config  
    tokens.push(gen_ns_token(Some("auth")));
    tokens.push(gen_flat_token(Some("secret"), ValueType::RandomHex(64)));
    tokens.push(gen_flat_token(Some("timeout"), ValueType::RandomNumber(300, 3600)));
    
    tokens.join("; ")
}

/// Generate a streaming dataset - like `for i in {1..N}; do printf "data\n"; done`
/// Each line is a separate token stream that can be piped through XStream commands
pub fn gen_stream_lines(line_count: usize, tokens_per_line: usize) -> String {
    let mut lines = Vec::new();
    
    for i in 1..=line_count {
        // Generate a line of tokens with sequence numbers
        let mut line_tokens = Vec::new();
        
        // Add sequence tracking
        line_tokens.push(format!("seq:line=\"{}\"", i));
        line_tokens.push(format!("seq:total=\"{}\"", line_count));
        
        // Add random tokens for the rest
        for _ in 2..tokens_per_line {
            let mut rng = rand::rng();
            let value_type = match rng.random_range(0..5) {
                0 => ValueType::RandomAlnum(2), // Short for visual blocks
                1 => ValueType::Literal(format!("{}■", get_rand_alpha(1))), // Color blocks
                2 => ValueType::FromList,
                3 => ValueType::RandomNumber(1, 99),
                _ => ValueType::RandomHex(4),
            };
            line_tokens.push(gen_token(None, None, value_type));
        }
        
        lines.push(line_tokens.join("; "));
    }
    
    lines.join("\n")
}

/// Generate timed stream data - simulates data arriving over time
/// Like `for i in {1..N}; do echo "timestamp=$(date) data"; sleep 0.1; done`
pub fn gen_timed_stream(line_count: usize) -> String {
    let mut lines = Vec::new();
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    for i in 0..line_count {
        let timestamp = start_time + (i as u64);
        
        let tokens = vec![
            format!("time:epoch=\"{}\"", timestamp),
            format!("time:seq=\"{}\"", i),
            gen_token(Some("sensor"), Some("temp"), ValueType::RandomNumber(18, 35)),
            gen_token(Some("sensor"), Some("humidity"), ValueType::RandomNumber(30, 80)),
            gen_token(Some("status"), None, ValueType::FromList),
        ];
        
        lines.push(tokens.join("; "));
    }
    
    lines.join("\n")
}

/// Generate log-style streaming data
/// Like tailing a log file with structured data
pub fn gen_log_stream(line_count: usize) -> String {
    let log_levels = ["DEBUG", "INFO", "WARN", "ERROR"];
    let components = ["auth", "db", "api", "cache", "worker"];
    let mut lines = Vec::new();
    
    for i in 0..line_count {
        let mut rng = rand::rng();
        let level = log_levels[rng.random_range(0..log_levels.len())];
        let component = components[rng.random_range(0..components.len())];
        
        let tokens = vec![
            format!("log:level=\"{}\"", level),
            format!("log:component=\"{}\"", component),
            format!("log:seq=\"{}\"", i),
            gen_token(Some("req"), Some("id"), ValueType::RandomHex(8)),
            gen_token(Some("perf"), Some("ms"), ValueType::RandomNumber(1, 500)),
        ];
        
        lines.push(tokens.join("; "));
    }
    
    lines.join("\n")
}

// Enhanced colored stream generation functions for Wave 3

/// Generate a colored token stream with namespace-based coloring
pub fn gen_colored_stream(namespaces: Vec<&str>, tokens_per_ns: usize) -> String {
    let color_map = get_namespace_color_map();
    let mut all_tokens = Vec::new();
    
    for namespace in namespaces {
        let color_prefix = color_map.get(namespace).unwrap_or(&"def");
        
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

/// Generate pre-colored tokens with color themes
pub fn gen_pre_colored_tokens(count: usize) -> Vec<String> {
    let colors = ["red", "blue", "green", "yellow", "purple", "cyan", "orange", "pink"];
    let mut tokens = Vec::new();
    
    for i in 0..count {
        let color = colors[i % colors.len()];
        let token = format!(
            "color:block{:02}=\"{}{:02}\"",
            i + 1,
            color,
            (i % 99) + 1
        );
        tokens.push(token);
    }
    
    tokens
}

/// Generate fork-ready colored streams
pub fn gen_fork_ready_stream(namespaces: Vec<&str>, tokens_per_ns: usize) -> String {
    let color_map = get_namespace_color_map();
    let mut all_tokens = Vec::new();
    
    for namespace in namespaces {
        let color_prefix = color_map.get(namespace).unwrap_or(&"def");
        
        for token_num in 1..=tokens_per_ns {
            let token = format!(
                "{}:data{:02}=\"{}{:02}\"",
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

/// Generate merge-ready colored streams (pre-forked format)
pub fn gen_merge_ready_streams(namespaces: Vec<&str>, tokens_per_ns: usize) -> String {
    let color_map = get_namespace_color_map();
    let mut streams = Vec::new();
    
    for namespace in namespaces {
        let color_prefix = color_map.get(namespace).unwrap_or(&"def");
        let mut namespace_tokens = Vec::new();
        
        for token_num in 1..=tokens_per_ns {
            let token = format!(
                "{}:val{:02}=\"{}{:02}\"",
                namespace,
                token_num,
                color_prefix,
                token_num
            );
            namespace_tokens.push(token);
        }
        
        streams.push(format!("{}: {}", namespace, namespace_tokens.join("; ")));
    }
    
    streams.join("\n")
}

/// Generate gate-ready streams with filtering conditions
pub fn gen_gate_ready_stream(include_auth: bool, token_count: usize) -> String {
    let mut tokens = Vec::new();
    
    // Add auth tokens if requested
    if include_auth {
        tokens.push("auth:token=\"valid123\"".to_string());
        tokens.push("auth:role=\"admin\"".to_string());
    }
    
    // Add regular tokens with color coding
    let colors = ["blue", "green", "yellow", "red"];
    let namespaces = ["ui", "db", "api", "cache"];
    
    for i in 0..token_count {
        let ns = namespaces[i % namespaces.len()];
        let color = colors[i % colors.len()];
        let token = format!(
            "{}:item{:02}=\"{}{:02}\"",
            ns,
            i + 1,
            color,
            i + 1
        );
        tokens.push(token);
    }
    
    tokens.join("; ")
}

/// Generate pipeline-ready streams with stages
pub fn gen_pipeline_ready_stream(stage_count: usize, tokens_per_stage: usize) -> String {
    let stages = ["input", "validate", "transform", "aggregate", "output"];
    let colors = ["grey", "blue", "yellow", "green", "purple"];
    let mut all_tokens = Vec::new();
    
    for stage_idx in 0..stage_count.min(stages.len()) {
        let stage = stages[stage_idx];
        let color = colors[stage_idx];
        
        for token_num in 1..=tokens_per_stage {
            let token = format!(
                "{}:step{:02}=\"{}{:02}\"",
                stage,
                token_num,
                color,
                token_num
            );
            all_tokens.push(token);
        }
    }
    
    all_tokens.join("; ")
}

/// Generate themed colored streams
pub fn gen_themed_stream(theme: &str, token_count: usize) -> String {
    let color_set = match theme {
        "rainbow" => vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"],
        "warm" => vec!["red", "orange", "yellow", "pink", "brown"],
        "cool" => vec!["blue", "cyan", "green", "purple", "teal"],
        "mono" => vec!["black", "grey", "white"],
        "neon" => vec!["lime", "cyan", "magenta", "yellow"],
        _ => vec!["red", "blue", "green", "yellow"], // default
    };
    
    let mut tokens = Vec::new();
    
    for i in 0..token_count {
        let color = color_set[i % color_set.len()];
        let token = format!(
            "theme:item{:02}=\"{}{:02}\"",
            i + 1,
            color,
            (i % 99) + 1
        );
        tokens.push(token);
    }
    
    tokens.join("; ")
}

/// Generate streams with block symbols for visual distinction
pub fn gen_symbol_stream(namespaces: Vec<&str>, tokens_per_ns: usize) -> String {
    let symbols = ["■", "▲", "●", "♦", "★", "▼", "◆", "♠"];
    let color_map = get_namespace_color_map();
    let mut all_tokens = Vec::new();
    
    for (ns_idx, namespace) in namespaces.iter().enumerate() {
        let color_prefix = color_map.get(namespace).unwrap_or(&"def");
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

/// Get namespace to color mapping
fn get_namespace_color_map() -> std::collections::HashMap<&'static str, &'static str> {
    let mut map = std::collections::HashMap::new();
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
    map.insert("input", "grey");
    map.insert("validate", "blue");
    map.insert("transform", "yellow");
    map.insert("aggregate", "green");
    map.insert("output", "purple");
    map
}