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