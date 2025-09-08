//! XStream Integration Tools Demo
//! 
//! This example demonstrates the user-friendly integration tools
//! for xstream, including the adapter and macros.

use xstream::{XStreamAdapter, MergeStrategy, fork_colored, pipeline, generate_stream};

fn main() {
    println!("🌊 XStream Integration Tools Demo\n");

    // 1. Adapter for JSON conversion
    println!("1. JSON Integration:");
    let mut adapter = XStreamAdapter::new();
    
    let json = r#"{"host": "localhost", "port": 8080, "db": {"user": "admin", "pass": "secret"}}"#;
    let stream = adapter.from_json(json).unwrap();
    println!("   JSON -> Stream: {}", stream);
    
    let json_back = adapter.to_json(&stream).unwrap();
    println!("   Stream -> JSON:\n{}", json_back);
    
    // 2. Fluent API
    println!("\n2. Fluent Stream Processing:");
    adapter.clear();
    let input = "ui:theme=\"dark\"; ui:size=\"large\"; db:host=\"localhost\"; db:port=\"5432\"";
    
    let result = adapter.process_stream(input)
        .fork_by(&["ui", "db"])
        .gate_min_tokens(2)
        .merge_with(MergeStrategy::Sort)
        .collect();
    
    println!("   Input: {}", input);
    println!("   Processed: {}", result);
    
    // 3. Macros for easy operations
    println!("\n3. User-Friendly Macros:");
    
    // Fork with colors
    let streams = fork_colored!(input, "ui", "db");
    println!("   Fork colored streams:");
    for (i, stream) in streams.iter().enumerate() {
        println!("     Stream {}: {}", i, stream);
    }
    
    // Pipeline macro
    let pipeline_result = pipeline!(input => fork(["ui", "db"]));
    println!("   Pipeline result:\n{}", pipeline_result);
    
    // Generate test streams
    let test_streams = generate_stream!(namespaces: ["test", "demo"], tokens_per_ns: 2, colors: false);
    println!("   Generated test streams:");
    for (i, stream) in test_streams.iter().enumerate() {
        println!("     Test {}: {}", i, stream);
    }
    
    // 4. Pipeline Builder
    println!("\n4. Pipeline Builder:");
    let pipeline = XStreamAdapter::pipeline()
        .fork(&["ui", "db"])
        .gate(1)
        .merge(MergeStrategy::Concat);
    
    let builder_result = pipeline.execute(input).unwrap();
    println!("   Builder result: {}", builder_result);
    
    // 5. CSV Integration
    println!("\n5. CSV Integration:");
    adapter.clear();
    let csv = "name,age,city\nAlice,25,NYC\nBob,30,SF";
    let csv_stream = adapter.from_csv(csv).unwrap();
    println!("   CSV -> Stream: {}", csv_stream);
    
    println!("\n✨ Integration tools demo complete!");
}