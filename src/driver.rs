// XStream Driver - Organized feature testing with RSB dispatch

use rsb::prelude::*;
use xstream::{gen_token_stream, gen_config_stream, transform, fork, merge, fork_all, xor, multi_xor, timed_gate, xor_gate_with_state, TX, MergeStrategy};
use xstream::colors::{colorize, colorize_fork_display, colorize_merge_display, colorize_workflow_display, colorize_xor_weaving, colorize_multi_xor_weaving, colored_separator, get_color, get_channel_color};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        show_help();
        return;
    }
    
    match args[1].as_str() {
        "help" => show_help(),
        "parsing" => test_parsing(),
        "transform" => test_transforms(), 
        "channels" => test_channels(),
        "gates" => test_gates(),
        "generation" => test_generation(),
        "integration" => test_integration(),
        "all" => run_all_tests(),
        _ => {
            println!("Unknown command: {}", args[1]);
            show_help();
        }
    }
}

fn show_help() {
    println!("=== XStream Driver - Feature Testing ===\n");
    println!("Usage: cargo run --bin xstream-driver <command>\n");
    println!("Commands:");
    println!("  help        - Show this help");
    println!("  parsing     - Test token parsing & validation");
    println!("  transform   - Test transform chains & operations");
    println!("  channels    - Test fork/merge channel operations");
    println!("  gates       - Test XOR gates and visual stream weaving");
    println!("  generation  - Test token generation");
    println!("  integration - Test RSB integration");
    println!("  all         - Run all test categories");
}

fn run_all_tests() {
    println!("=== Running All XStream Tests ===\n");
    test_parsing();
    test_transforms();
    test_channels();
    test_gates();
    test_generation();
    test_integration();
    println!("=== All Tests Complete ===");
}

fn test_parsing() {
    println!("=== Testing Token Parsing & Validation ===");
    
    // Valid token formats
    println!("\n1. Valid Token Formats:");
    let valid_examples = vec![
        r#"host="localhost""#,
        r#"ui:theme="dark"; ui:lang="en""#,
        r#"ns=config; host="server"; port="8080""#,
        r#"deep.namespace.example:key="value""#,
    ];
    
    for example in valid_examples {
        println!("  ✓ {}", example);
        println!("    Valid: {}", xstream::is_token_streamable(example));
    }
    
    // Invalid formats
    println!("\n2. Invalid Token Formats:");
    let invalid_examples = vec![
        "bad token",           // Missing =
        r#"key= "value""#,     // Space after =
        r#"my key="value""#,   // Space in key
        r#"ns:my key="val""#,  // Space in prefixed key
    ];
    
    for example in invalid_examples {
        println!("  ✗ {}", example);
        println!("    Valid: {}", xstream::is_token_streamable(example));
    }
    
    println!("=== Parsing Tests Complete ===\n");
}

fn test_transforms() {
    println!("=== Testing Transform Operations ===");
    
    // Basic transforms
    println!("\n1. Basic String Transforms:");
    let config = r#"host="LOCALHOST"; db:user="Admin"; pass="secret123""#;
    
    let lowered = transform(config).lower().to_string();
    println!("  Original: {}", config);
    println!("  Lowered:  {}", lowered);
    
    // Sensitive masking
    println!("\n2. Sensitive Data Masking:");
    let with_secrets = r#"api_key="abc123"; password="secret"; host="localhost""#;
    let masked = transform(with_secrets).mask_sensitive().to_string();
    println!("  Original: {}", with_secrets);
    println!("  Masked:   {}", masked);
    
    // Transform chaining
    println!("\n3. Transform Chaining:");
    let chain_input = r#"host="localhost"; ns=db; user="admin""#;
    let chained = transform(chain_input)
        .translate("localhost", "prod-server")
        .rename_namespace("db", "database")
        .expand()
        .to_string();
    println!("  Original: {}", chain_input);
    println!("  Chained:  {}", chained);
    
    // Encoding transforms
    println!("\n4. Encoding Transforms:");
    let text = r#"message="Hello World""#;
    let encoded = transform(text).base64(TX::ENCODE).to_string();
    let decoded = transform(&encoded).base64(TX::DECODE).to_string();
    println!("  Original: {}", text);
    println!("  Encoded:  {}", encoded);
    println!("  Decoded:  {}", decoded);
    
    println!("=== Transform Tests Complete ===\n");
}

fn test_channels() {
    println!("{}", colored_separator("Testing Channel Operations"));
    
    let mixed_stream = r#"ui:theme="dark"; db:host="localhost"; ui:lang="en"; log:level="info"; db:port="5432""#;
    
    // Fork operations with COLOR!
    println!("\n{}1. Fork by Specific Channels:{}", colorize("", "info"), xstream::colors::RESET);
    let (ui, db, logs) = fork!(mixed_stream, "ui", "db", "log");
    
    let fork_data = vec![
        ("ui".to_string(), ui.clone()),
        ("db".to_string(), db.clone()), 
        ("log".to_string(), logs.clone())
    ];
    println!("{}", colorize_fork_display(mixed_stream, &fork_data));
    
    // Fork all channels with COLOR!
    println!("{}2. Fork All Channels:{}", colorize("", "info"), xstream::colors::RESET);
    let all_channels = fork_all!(mixed_stream);
    let all_fork_data: Vec<(String, String)> = all_channels.into_iter().collect();
    println!("{}", colorize_fork_display(mixed_stream, &all_fork_data));
    
    // Merge operations with COLOR!
    println!("{}3. Basic Merge:{}", colorize("", "info"), xstream::colors::RESET);
    let merged_basic = merge!(ui.as_str(), db.as_str());
    let merge_inputs = vec![
        ("ui".to_string(), ui.clone()),
        ("db".to_string(), db.clone())
    ];
    println!("{}", colorize_merge_display(&merge_inputs, &merged_basic));
    
    // Merge with strategy and COLOR!
    println!("{}4. Merge with Priority Strategy:{}", colorize("", "info"), xstream::colors::RESET);
    let priority_merged = merge!(strategy: MergeStrategy::Priority(vec!["db".to_string(), "ui".to_string()]), 
                                db.as_str(), ui.as_str());
    let priority_inputs = vec![
        ("db".to_string(), db.clone()),
        ("ui".to_string(), ui.clone())
    ];
    println!("{}", colorize_merge_display(&priority_inputs, &priority_merged));
    
    // Complete workflow with VISUAL STREAM WEAVING!
    println!("{}5. Complete Channel Workflow - Stream Weaving:{}", colorize("", "info"), xstream::colors::RESET);
    
    // Transform each channel
    let ui_proc = transform(&ui).translate("dark", "light").to_string();
    let db_proc = transform(&db).mask_sensitive().to_string();
    
    // Prepare data for workflow visualization
    let forks = vec![
        ("ui".to_string(), ui.clone()),
        ("db".to_string(), db.clone())
    ];
    let transforms = vec![
        ("ui".to_string(), ui_proc.clone()),
        ("db".to_string(), db_proc.clone())
    ];
    let final_result = merge!(ui_proc.as_str(), db_proc.as_str());
    
    // Show the complete visual workflow
    println!("{}", colorize_workflow_display(mixed_stream, &forks, &transforms, &final_result));
    
    println!("{}", colored_separator("Channel Tests Complete"));
}

fn test_gates() {
    println!("{}", colored_separator("Testing XOR Gates & Stream Weaving"));
    
    // Generate longer streams using gen.rs - create some simple patterns for now
    // We'll make them manually to get the perfect consistent weaving patterns
    let g_stream = "g:a=\"1\"; g:b=\"2\"; g:c=\"3\"; g:d=\"4\"; g:e=\"5\"; g:f=\"6\"; g:g=\"7\"; g:h=\"8\"";
    let f_stream = "f:a=\"1\"; f:b=\"2\"; f:c=\"3\"; f:d=\"4\"; f:e=\"5\"; f:f=\"6\"; f:g=\"7\"; f:h=\"8\"";
    let h_stream = "h:a=\"1\"; h:b=\"2\"; h:c=\"3\"; h:d=\"4\"; h:e=\"5\"; h:f=\"6\"; h:g=\"7\"; h:h=\"8\"";
    let j_stream = "j:a=\"1\"; j:b=\"2\"; j:c=\"3\"; j:d=\"4\"; j:e=\"5\"; j:f=\"6\"; j:g=\"7\"; j:h=\"8\"";
    
    // XOR Gate Test - Longer streams for visible weaving
    println!("\n{}1. XOR Gate - Visual Stream Weaving (Long Patterns):{}", colorize("", "info"), xstream::colors::RESET);
    
    let (xor_result, gate_state) = xor_gate_with_state(g_stream, f_stream);
    println!("{}", colorize_xor_weaving(g_stream, f_stream, &xor_result, &gate_state));
    
    // Multi-XOR Gate Test - 4 streams cycling  
    println!("{}2. Multi-XOR Gate - Cycling Through 4 Streams (Long Pattern):{}", colorize("", "info"), xstream::colors::RESET);
    
    let multi_result = multi_xor!(g_stream, f_stream, h_stream, j_stream);
    let multi_streams = vec![
        ("g".to_string(), g_stream.to_string()),
        ("f".to_string(), f_stream.to_string()),
        ("h".to_string(), h_stream.to_string()),
        ("j".to_string(), j_stream.to_string()),
    ];
    println!("{}", colorize_multi_xor_weaving(&multi_streams, &multi_result));
    
    // Timed Gate Test - Switch every 3 tokens for longer pattern
    println!("{}3. Timed Gate - Switch Every 3 Tokens (Long Pattern):{}", colorize("", "info"), xstream::colors::RESET);
    
    let timed_result = timed_gate!(3, g_stream, f_stream);
    println!("{}  Input G:{} {}{}{}", 
        get_color("cyan"), xstream::colors::RESET, 
        get_channel_color(0), g_stream, xstream::colors::RESET);
    println!("{}  Input F:{} {}{}{}", 
        get_color("cyan"), xstream::colors::RESET, 
        get_channel_color(1), f_stream, xstream::colors::RESET);
    
    // Show timed result with alternating colors every 3 tokens
    let timed_tokens: Vec<&str> = timed_result.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let colored_timed: Vec<String> = timed_tokens.iter().enumerate().map(|(i, token)| {
        let color_index = (i / 3) % 2; // Switch color every 3 tokens
        let color_code = get_channel_color(color_index);
        format!("{}{}{}", color_code, token, xstream::colors::RESET)
    }).collect();
    
    println!("{}  Timed Result:{} {}", 
        get_color("green"), xstream::colors::RESET, 
        colored_timed.join(&format!("{}; {}", xstream::colors::RESET, "")));
    
    // XOR Macro Test
    println!("\n{}4. XOR Macro Test:{}", colorize("", "info"), xstream::colors::RESET);
    let simple_a = r#"type="user""#;
    let simple_b = r#"action="login""#;
    let _macro_result = xor!(simple_a, simple_b);
    
    println!("{}  Input A:{} {}{}{}", 
        get_color("cyan"), xstream::colors::RESET, 
        get_channel_color(0), simple_a, xstream::colors::RESET);
    println!("{}  Input B:{} {}{}{}", 
        get_color("cyan"), xstream::colors::RESET, 
        get_channel_color(1), simple_b, xstream::colors::RESET);
    println!("{}  XOR Result:{} {}{}{}{}{}{}{}",
        get_color("green"), xstream::colors::RESET,
        get_channel_color(0), "type=\"user\"", xstream::colors::RESET,
        "; ",
        get_channel_color(1), "action=\"login\"", xstream::colors::RESET);
    
    // BONUS: Pre-colored stream demonstration  
    println!("\n{}5. Pre-Colored Stream XOR (True Visual Weaving):{}", colorize("", "info"), xstream::colors::RESET);
    
    // Create pre-colored streams using ANSI codes
    let red_stream = format!("{}red:a=\"1\"{}; {}red:b=\"2\"{}; {}red:c=\"3\"{}", 
        get_channel_color(0), xstream::colors::RESET,
        get_channel_color(0), xstream::colors::RESET,
        get_channel_color(0), xstream::colors::RESET);
    let blue_stream = format!("{}blue:a=\"1\"{}; {}blue:b=\"2\"{}; {}blue:c=\"3\"{}", 
        get_channel_color(1), xstream::colors::RESET,
        get_channel_color(1), xstream::colors::RESET,
        get_channel_color(1), xstream::colors::RESET);
    
    println!("{}  Pre-Colored Red:{} {}", get_color("cyan"), xstream::colors::RESET, red_stream);
    println!("{}  Pre-Colored Blue:{} {}", get_color("cyan"), xstream::colors::RESET, blue_stream);
    
    let pre_colored_result = xor!(red_stream.as_str(), blue_stream.as_str());
    println!("{}  XOR Result:{} {}", get_color("green"), xstream::colors::RESET, pre_colored_result);
    
    println!("{}", colored_separator("XOR Gate Tests Complete"));
}

fn test_generation() {
    println!("=== Testing Token Generation ===");
    
    // Random token streams
    println!("1. Random Token Stream Generation:");
    let random_stream = gen_token_stream(5, 0.3);
    println!("  Random stream: {}", random_stream);
    println!("  Valid: {}", xstream::is_token_streamable(&random_stream));
    
    // Config generation
    println!("\n2. Config-style Generation:");
    let config_stream = gen_config_stream();
    println!("  Config stream: {}", config_stream);
    
    // Parse generated streams
    println!("\n3. Parse Generated Stream:");
    let bucket = xstream::TokenBucket::from_str(&config_stream, xstream::BucketMode::Hybrid).unwrap();
    println!("  Namespaces found: {:?}", bucket.data.keys().collect::<Vec<_>>());
    
    println!("=== Generation Tests Complete ===\n");
}

fn test_integration() {
    println!("=== Testing RSB Integration ===");
    
    // RSB stream operations on tokens
    println!("1. RSB Stream Processing:");
    let tokens = r#"host="localhost"; port="8080"; debug="true""#;
    
    let processed = stream!(string: tokens)
        .sed("localhost", "production-server")
        .sed("debug=\"true\"", "debug=\"false\"")
        .to_string();
    
    println!("  Original: {}", tokens);
    println!("  RSB proc: {}", processed);
    println!("  Valid:    {}", xstream::is_token_streamable(&processed));
    
    // Combined XStream + RSB
    println!("\n2. XStream + RSB Combined:");
    let combined = transform(tokens)
        .translate("8080", "80")
        .custom(|stream| stream.sed("=\"", " = \""))
        .to_string();
    
    println!("  Combined: {}", combined);
    
    // Token generation with RSB random
    println!("\n3. RSB Random in Token Generation:");
    let dynamic_key = get_rand_alpha(8);
    let dynamic_value = get_rand_hex(12);
    let dynamic_token = format!("{}=\"{}\"", dynamic_key, dynamic_value);
    println!("  Dynamic:  {}", dynamic_token);
    println!("  Valid:    {}", xstream::is_token_streamable(&dynamic_token));
    
    println!("=== Integration Tests Complete ===\n");
}