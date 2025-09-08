# XStream Integration Tools

This document describes the user-friendly integration tools for xstream, designed to make complex stream operations simple and accessible.

## Features

### 1. XStreamAdapter - Integration Adapter

The `XStreamAdapter` provides easy integration patterns for xstream with other systems, focusing on user ergonomics and common use cases.

```rust
use xstream::{XStreamAdapter, MergeStrategy};

// Create adapter
let mut adapter = XStreamAdapter::new();

// JSON integration
let json = r#"{"host": "localhost", "port": 8080}"#;
let stream = adapter.from_json(json)?;
let json_back = adapter.to_json(&stream)?;

// CSV integration
let csv = "name,age\nAlice,25\nBob,30";
let stream = adapter.from_csv(csv)?;

// Fluent processing
let result = adapter.process_stream(input)
    .fork_by(&["ui", "db"])
    .gate_min_tokens(2)
    .merge_with(MergeStrategy::Sort)
    .collect();
```

### 2. User-Friendly Macros

#### `xstream!` - Fluent API Creation
Creates a stream processor with fluent API:

```rust
let result = xstream!(input)
    .fork_by(&["ui", "db"])
    .merge_with(MergeStrategy::Concat)
    .collect();
```

#### `fork_colored!` - Quick Fork with Colors
Forks stream by namespaces with automatic color assignment:

```rust
let streams = fork_colored!(input, "ui", "db", "api");
```

#### `pipeline!` - Pipeline Processing
Creates processing pipelines with multiple steps:

```rust
let result = pipeline!(input => fork(["ui", "db"]) => gate(min_tokens: 2) => merge(MergeStrategy::Sort));
```

#### `generate_stream!` - Test Stream Generation
Generates test streams for development:

```rust
let streams = generate_stream!(namespaces: ["ui", "db"], tokens_per_ns: 3, colors: true);
```

#### `test_stream!` - Stream Testing
Provides convenient testing for stream operations:

```rust
let result = test_stream!(input, "expected", |s| s.fork_by(&["ui"]).merge_with(MergeStrategy::Concat));
result.assert_passed();
```

#### `build_pipeline!` - Pipeline Builder
Creates reusable pipeline configurations:

```rust
let pipeline = build_pipeline!(fork => ["ui", "db"]; gate => 2; merge => MergeStrategy::Concat);
let result = pipeline.execute(input)?;
```

#### `validate_stream!` - Stream Validation
Quick validation of token stream format:

```rust
validate_stream!(stream)?;
```

#### `process_if!` - Conditional Processing
Allows conditional processing based on stream content:

```rust
let result = process_if!(stream, has_namespace("ui") => fork(["ui", "ux"]));
```

### 3. Pipeline Builder

The pipeline builder provides a fluent interface for creating complex processing pipelines:

```rust
let pipeline = XStreamAdapter::pipeline()
    .fork(&["ui", "db"])
    .gate(2)  // minimum 2 tokens
    .merge(MergeStrategy::Sort);

let result = pipeline.execute(input)?;
```

### 4. Error Handling

All integration tools use consistent error handling:

```rust
use xstream::AdapterError;

match adapter.from_json(invalid_json) {
    Ok(stream) => println!("Success: {}", stream),
    Err(AdapterError::ParseError(msg)) => eprintln!("Parse error: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

See `examples/integration_demo.rs` for a complete demonstration of all integration features:

```bash
cargo run --example integration_demo
```

## Key Benefits

1. **User Ergonomics**: Simple, intuitive APIs for common operations
2. **No RSB Collision**: Macros designed to not interfere with existing RSB patterns
3. **Integration Focus**: Easy plugging into other systems (JSON, CSV, etc.)
4. **Comprehensive Coverage**: All main use cases covered (fork, merge, gate, pipeline)
5. **Error Handling**: Proper error types and meaningful messages
6. **Color Support**: Visual feedback and debugging support
7. **Testing Support**: Built-in testing utilities

## Integration Patterns

### JSON Workflow
```rust
let mut adapter = XStreamAdapter::new();
let stream = adapter.from_json(json_input)?;
let processed = adapter.process_stream(&stream)
    .fork_by(&["config", "data"])
    .gate_min_tokens(1)
    .merge_with(MergeStrategy::Sort)
    .collect();
let json_output = adapter.to_json(&processed)?;
```

### CSV Processing
```rust
let csv_stream = adapter.from_csv(csv_data)?;
let split_streams = adapter.split_and_process(&csv_stream, &["row0", "row1"]);
let merged = adapter.merge_and_filter(&split_streams, 2);
```

### Pipeline Chaining
```rust
let pipeline1 = XStreamAdapter::pipeline().fork(&["ui"]).gate(1);
let pipeline2 = XStreamAdapter::pipeline().merge(MergeStrategy::Concat);

let intermediate = pipeline1.execute(input)?;
let final_result = pipeline2.execute(&intermediate)?;
```

The integration tools make xstream accessible to users who want simple, ergonomic APIs without needing to understand the low-level Streamable implementations.