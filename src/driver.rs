// XStream Driver - Bash-like token stream generation with RSB

use rsb::prelude::*;
use xstream::{gen_token_stream, gen_flat_token, gen_ns_token, gen_token, gen_config_stream, 
              ValueType, TokenBucket, BucketMode, is_token_streamable, transform};

fn main() {
    println!("=== XStream Driver with RSB Streams ===\n");

    // Example 1: Basic token generation pipeline
    println!("1. Basic Pipeline - Generate and transform tokens:");
    let tokens = gen_token_stream(5, 0.3);
    
    // Create a stream from the token string and apply transformations
    let result = stream!(string: &tokens)
        .sed("=\"", "='")  // Change quotes style
        .sed("\"", "'")
        .to_string();
    
    println!("Original: {}", tokens);
    println!("Modified: {}\n", result);

    // Example 2: Generate tokens and filter by namespace
    println!("2. Namespace Filtering Pipeline:");
    let config_tokens = vec![
        gen_flat_token(Some("host"), ValueType::Literal("localhost".to_string())),
        gen_ns_token(Some("auth")),
        gen_flat_token(Some("secret"), ValueType::RandomHex(16)),
        gen_flat_token(Some("timeout"), ValueType::RandomNumber(300, 3600)),
        gen_ns_token(Some("db")),
        gen_flat_token(Some("host"), ValueType::Literal("db.example.com".to_string())),
        gen_flat_token(Some("port"), ValueType::RandomNumber(5432, 5440)),
    ];
    
    let stream_text = config_tokens.join("; ");
    println!("Full stream: {}", stream_text);
    
    // Parse and filter tokens
    let bucket = TokenBucket::from_str(&stream_text, BucketMode::Hybrid).unwrap();
    println!("Auth tokens: {:?}", bucket.get_namespace("auth"));
    println!("DB tokens: {:?}", bucket.get_namespace("db"));
    println!("Global tokens: {:?}\n", bucket.get_namespace("global"));

    // Example 3: Bash-like generation with chained operations
    println!("3. Advanced Pipeline - Generate, transform, and format:");
    
    // Generate 10 random tokens
    let mut random_tokens = Vec::new();
    for i in 0..10 {
        let token = if i % 3 == 0 {
            // Every 3rd token is a namespace switch
            gen_ns_token(Some(&format!("section{}", i/3)))
        } else {
            gen_token(
                Some(&format!("prefix{}", i)),
                Some(&format!("key{}", i)),
                ValueType::RandomAlnum(8)
            )
        };
        random_tokens.push(token);
    }
    
    // Join and process through stream pipeline
    let pipeline_result = stream!(string: &random_tokens.join("; "))
        .sed("prefix", "pfx")  // Shorten prefix
        .sed("section", "sec")  // Shorten section
        .to_string();
    
    println!("Generated tokens:");
    println!("{}\n", pipeline_result);
    
    // Example 4: Config generation with RSB stream macros
    println!("4. Config Generation with Stream Processing:");
    
    // Generate a realistic config
    let config = gen_config_stream();
    
    // Process through stream to add comments
    let annotated = stream!(string: &config)
        .sed("ns=db", "ns=db # Database configuration")
        .sed("ns=auth", "ns=auth # Authentication settings")
        .sed("debug=", "debug= # Debug mode: ")
        .to_string();
    
    println!("Original config:");
    println!("{}\n", config);
    println!("Annotated config:");
    println!("{}\n", annotated);
    
    // Example 5: Stream splitting and parallel processing
    println!("5. Split and Process Streams:");
    
    let multi_ns_stream = "ns=api; port=\"8080\"; ns=cache; ttl=\"3600\"; ns=log; level=\"info\"";
    
    // Split by namespace markers
    let parts: Vec<String> = multi_ns_stream
        .split("ns=")
        .filter(|s| !s.is_empty())
        .map(|part| {
            let ns_part = format!("ns={}", part);
            stream!(string: &ns_part)
                .sed(";", ";\n  ")  // Add newlines for readability
                .to_string()
        })
        .collect();
    
    println!("Split namespaces:");
    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {}", i, part.trim());
    }
    println!();
    
    // Example 6: Token validation pipeline
    println!("6. Validation Pipeline:");
    
    let test_streams = vec![
        "key=\"valid\"; other=\"good\"",
        "bad key=\"invalid\"",  // Space in key
        "key= \"bad\"",         // Space after =
        "good=\"token\";ns=switch;more=\"data\"",
    ];
    
    for stream in test_streams {
        let validation = if is_token_streamable(stream) {
            stream!(string: stream)
                .sed("$", " ")  // Add checkmark at end
                .to_string()
        } else {
            format!("{}  INVALID", stream)
        };
        println!("Stream: {}", validation);
    }
    println!();
    
    // Example 7: Dynamic token generation with RSB
    println!("7. Dynamic Generation with RSB:");
    
    // Use RSB's random functions directly in a pipeline
    let dynamic_tokens = (0..5)
        .map(|i| {
            let key = get_rand_alpha(6);
            let value = match i % 3 {
                0 => get_rand_hex(12),
                1 => get_rand_alnum(8),
                _ => get_rand_from_slice(&vec!["active".to_string(), "inactive".to_string(), "pending".to_string()])
                    .unwrap_or("unknown".to_string()),
            };
            format!("{}=\"{}\"", key, value)
        })
        .collect::<Vec<_>>()
        .join("; ");
    
    println!("Dynamic tokens: {}\n", dynamic_tokens);
    
    // Example 8: Stream-based token aggregation
    println!("8. Token Aggregation Pipeline:");
    
    let streams = vec![
        gen_token_stream(3, 0.0),  // All prefixed
        gen_token_stream(3, 1.0),  // All flat
        gen_config_stream(),        // Mixed config
    ];
    
    let aggregated = streams
        .into_iter()
        .map(|s| stream!(string: &s).to_string())
        .collect::<Vec<_>>()
        .join(" | ");  // Use pipe as separator
    
    println!("Aggregated streams (pipe-separated):");
    println!("{}\n", aggregated);
    
    // Example 9: Token stream statistics
    println!("9. Stream Statistics:");
    
    let stats_stream = gen_token_stream(20, 0.5);
    let bucket = TokenBucket::from_str(&stats_stream, BucketMode::Hybrid).unwrap_or_else(|e| {
        println!("Error parsing: {}", e);
        TokenBucket::new(BucketMode::Hybrid)
    });
    
    let token_count = bucket.data.values().map(|ns| ns.len()).sum::<usize>();
    let namespace_count = bucket.data.len();
    
    println!("Stream: {} chars", stats_stream.len());
    println!("Tokens: {}", token_count);
    println!("Namespaces: {}", namespace_count);
    println!("Sample: {}...\n", &stats_stream[..stats_stream.len().min(50)]);
    
    // Example 10: Power chains with transform!
    println!("10. Transform Power Chains:");
    
    let raw_config = "host=\"localhost\"; pass=\"secret123\"; db:user=\"admin\"; db:pass=\"dbpass\"";
    
    // Chain multiple transformations
    let transformed = transform(raw_config)
        .translate("localhost", "127.0.0.1")
        .rename_namespace("db", "database")
        .mask_sensitive()
        .expand()
        .to_string();
    
    println!("Original: {}", raw_config);
    println!("Transformed: {}\n", transformed);
    
    // Example 11: Quote manipulation
    println!("11. Quote Style Transformations:");
    
    let mixed_quotes = "key=\"value\"; other='data'; unquoted=text";
    
    let single = transform(mixed_quotes).single_quotes().to_string();
    let double = transform(mixed_quotes).double_quotes().to_string();
    let stripped = transform(mixed_quotes).strip_quotes().to_string();
    
    println!("Original: {}", mixed_quotes);
    println!("Single quotes: {}", single);
    println!("Double quotes: {}", double);
    println!("No quotes: {}\n", stripped);
    
    // Example 12: Advanced pattern matching
    println!("12. Pattern-based Transformations:");
    
    let tokens = gen_token_stream(10, 0.3);
    
    // Remove all tokens with 'secret' in them
    let safe = transform(tokens.clone())
        .remove_matching("secret")
        .compact()
        .to_string();
    
    // Keep only tokens with 'host' or 'port'
    let network_only = transform(tokens.clone())
        .keep_matching("host")
        .to_string();
    
    println!("Original: {}", &tokens[..tokens.len().min(100)]);
    println!("No secrets: {}", &safe[..safe.len().min(100)]);
    println!("Network only: {}\n", network_only);
    
    // Example 13: Multi-step transformation with validation
    println!("13. Complex Transform Chain with Validation:");
    
    let config = gen_config_stream();
    
    let result = transform(config.clone())
        .rename_namespace("db", "database")
        .rename_namespace("auth", "security")
        .translate("localhost", "prod-server-01")
        .rename_key("pass", "password")
        .rename_key("secret", "api_key")
        .single_quotes()
        .expand();
    
    println!("Valid before: {}", is_token_streamable(&config));
    println!("Valid after: {}", result.validate());
    let result_str = result.to_string();
    println!("Sample: {}...\n", &result_str[..result_str.len().min(100)]);
    
    // Example 14: Custom RSB stream operations
    println!("14. Custom RSB Stream Integration:");
    
    let data = "prefix:key=\"value\"; other=\"data\"";
    
    let custom = transform(data)
        .custom(|stream| {
            // Use any RSB stream operations
            stream
                .sed("prefix", "PREFIX")
                .sed("=\"", " = \"")
                .sed("\";", "\" ;")
        })
        .to_string();
    
    println!("Original: {}", data);
    println!("Custom RSB: {}\n", custom);
    
    // Example 15: Format transformations
    println!("15. Format Transformations:");
    
    let compact_stream = "a=\"1\";b=\"2\";c=\"3\";d=\"4\"";
    
    let expanded = transform(compact_stream).expand().to_string();
    let multiline = transform(compact_stream).multiline().to_string();
    
    println!("Compact: {}", compact_stream);
    println!("Expanded: {}", expanded);
    println!("Multiline:\n{}\n", multiline);
    
    println!("=== XStream Driver Complete ===");
}