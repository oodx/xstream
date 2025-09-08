# XStream

**Advanced Stream Processing with Visual Flow Tracking**

XStream is a sophisticated Rust library and toolkit for processing token streams with colorful visual tracking, namespace-aware operations, and robust pipeline management. Built on the RSB (Rebel) framework, it provides powerful stream manipulation capabilities with beautiful, color-coded visual feedback.

**NEW**: XStream now features user-friendly adapters and macros that make integration simple and intuitive. Convert JSON/CSV to token streams, use fluent APIs for complex operations, and leverage convenient macros for common patterns.

## 🌟 Features

- **🍴 Fork Operations**: Split streams by namespace with intelligent discovery
- **🔀 Merge Strategies**: Concatenate, interleave, and deduplicate streams  
- **🚪 Gate Controls**: Filter streams by token count, namespace requirements, and synchronization
- **🔄 Pipeline Chains**: Multi-stage stream transformations with error recovery
- **🎨 Visual Tracking**: Color-coded token visualization preserves origin through operations
- **⚡ RSB Integration**: Built on the proven Rebel framework for reliability
- **📱 Easy Integration**: XStreamAdapter for JSON/CSV conversion and external system integration
- **🏗️ User-Friendly Macros**: Intuitive macros for common operations and testing patterns
- **🔗 Fluent API**: Method chaining for readable and maintainable stream processing
- **📊 Test Utilities**: Built-in helpers for generating test data and validating operations

## 📦 Installation & Setup

### Prerequisites

- Rust 1.70+ with Cargo
- Git access to RSB framework repository

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd xstream

# Build the project
cargo build --release

# Run tests to verify installation (all 65 tests should pass)
cargo test

# Try the new integration tools
cargo run --example integration_demo

# Try a quick demonstration
cargo run --bin xstream-driver help
```

### Available Binaries

XStream provides several specialized tools:

- **`xstream-driver`**: Visual ceremony demonstrations
- **`xstream-gen`**: Token stream generator for testing
- **`xstream-color-gen`**: Specialized color stream generator
- **`pretty`**: Stream pretty-printing utility

## 🚀 Quick Start

### Easy Integration with XStreamAdapter

```rust
use xstream::{XStreamAdapter, MergeStrategy};

// Create an adapter for easy operations
let mut adapter = XStreamAdapter::new();

// Convert JSON to token stream
let json = r#"{"host": "localhost", "port": 8080}"#;
let stream = adapter.from_json(json).unwrap();
// Result: host="localhost"; port="8080"

// Process with fluent API
let input = "ui:theme=\"dark\"; ui:size=\"large\"; db:host=\"localhost\"";
let result = adapter.process_stream(input)
    .fork_by(&["ui", "db"])
    .gate_min_tokens(1)
    .merge_with(MergeStrategy::Concat)
    .collect();
```

### User-Friendly Macros

```rust
use xstream::{fork_colored, pipeline, generate_stream};

// Fork with automatic colors
let input = "ui:btn=\"click\"; db:host=\"localhost\"";
let colored_streams = fork_colored!(input, "ui", "db");

// Pipeline operations
let result = pipeline!(input => fork(["ui", "db"]) => gate(min_tokens: 1));

// Generate test data
let test_streams = generate_stream!(namespaces: ["ui", "db"], tokens_per_ns: 3);
```

### Basic Stream Operations (Advanced)

```rust
use xstream::xstream::{
    fork::{Fork, ForkAll},
    merge::{Merge, MergeStrategy},
    gate::{Gate, GateCondition}
};
use rsb::prelude::*;

// Create a mixed stream
let input = "ui:btn=\"click\"; db:host=\"localhost\"; ui:theme=\"dark\"; api:status=\"ok\"";

// Fork by namespace
let forked = input.stream_apply(Fork, vec!["ui".to_string(), "db".to_string()]);

// Gate by minimum tokens
let gated = forked.stream_apply(Gate, GateCondition::MinTokens(2));

// Merge results
let merged = gated.stream_apply(Merge, MergeStrategy::Concat);
```

### Visual Stream Processing

The heart of XStream is its visual ceremony system that demonstrates operations with color-coded tokens:

```bash
# Run all visual demonstrations
cargo run --bin xstream-driver all

# Specific operation demos
cargo run --bin xstream-driver fork
cargo run --bin xstream-driver merge
cargo run --bin xstream-driver gate
cargo run --bin xstream-driver pipeline
```

## 🎭 Visual Testing System

### Driver Ceremonies

XStream includes sophisticated visual demonstrations called "ceremonies" that showcase operations with color-coded output:

**Fork Ceremonies**: Stream splitting demonstrations
- Basic namespace separation
- Automatic discovery with `ForkAll`
- Selective namespace filtering
- Deep nested namespace handling
- Empty namespace graceful handling

**Merge Ceremonies**: Stream combination strategies
- Concatenation preserving origin colors
- Interleaved round-robin merging
- Duplicate detection and removal
- Weighted priority merging
- Empty stream graceful handling

**Gate Ceremonies**: Flow control demonstrations
- Minimum/maximum token filtering
- Namespace requirement validation
- Multi-stream synchronization
- Value-based content filtering
- Rate limiting and throughput control

**Pipeline Ceremonies**: Multi-stage transformations
- Fork → Gate → Merge chains
- Error recovery with backup streams
- Conditional routing by priority
- Feedback loop processing

### Color System

XStream uses a sophisticated color system for visual tracking:

```rust
// Colors travel with tokens through operations
let red_ui_token = create_colored_token("ui", "btn", "click01", "red");
let blue_db_token = create_colored_token("db", "host", "local01", "blue");

// After merge, you can still see origin by color:
// ui tokens remain red ■, db tokens remain blue ■
```

## 🧪 Stream Generation

### XStream Generator (`xstream-gen`)

Generate test streams for development and testing:

```bash
# Generate colored streams by namespace
cargo run --bin xstream-gen colored --namespaces ui,db,api --tokens 3

# Create fork-ready streams
cargo run --bin xstream-gen pattern --pattern fork --complexity medium

# Generate merge-ready streams  
cargo run --bin xstream-gen pattern --pattern merge --complexity simple
```

### Color Generator (`xstream-color-gen`)

Specialized color stream generation:

```bash
# Rainbow theme with 8 tokens
cargo run --bin xstream-color-gen theme --theme rainbow --count 8

# Namespace-colored streams with symbols
cargo run --bin xstream-color-gen namespace --namespaces ui,db --symbols true

# Color gradients for pipeline visualization
cargo run --bin xstream-color-gen gradient --start red --end blue --steps 5
```

## 🧪 Testing Suite

### Automated Test Scripts

XStream includes comprehensive testing scripts in the `bin/` directory:

```bash
# Run complete test suite
./bin/test.sh all

# Test specific ceremonies
./bin/test.sh fork
./bin/test.sh merge --verbose

# Individual showcase scripts
./bin/showcase-fork.sh
./bin/showcase-merge.sh
./bin/showcase-pipeline.sh
```

### Test Script Features

- **Colored Output**: Status indicators with visual feedback
- **Unit Test Integration**: Cargo test execution with filtering
- **Ceremony Validation**: Visual demonstration verification  
- **Verbose Mode**: Detailed error reporting for debugging
- **Pass/Fail Tracking**: Comprehensive result summaries

## ⚡ RSB Integration

XStream is built on the RSB (Rebel) framework, providing:

- **Stream Apply Pattern**: Consistent `.stream_apply(Operation, Parameters)` interface
- **Streamable Trait**: Universal stream processing capabilities
- **Error Handling**: Robust error management through RSB patterns
- **Macro System**: Simplified operation definitions

```rust
// RSB integration examples
use rsb::prelude::*;

// The streamable trait enables consistent operations
let result = "data".stream_apply(Transform, params);

// Fork macro for easy namespace splitting  
let forked = fork!(input => "ui", "db", "api");

// Merge macro with strategy selection
let merged = merge!(stream1, stream2; strategy = MergeStrategy::Interleave);
```

## 🔗 Easy Integration

### JSON/CSV Integration

XStream now provides seamless integration with common data formats:

```rust
use xstream::XStreamAdapter;

let mut adapter = XStreamAdapter::new();

// JSON to token stream
let json = r#"{
  "host": "localhost",
  "port": 8080,
  "db": {
    "user": "admin",
    "pass": "secret"
  }
}"#;

let stream = adapter.from_json(json).unwrap();
// Result: host="localhost"; port="8080"; db:user="admin"; db:pass="secret"

// Convert back to JSON
let json_back = adapter.to_json(&stream).unwrap();

// CSV to token stream
let csv = "name,age,city\nAlice,25,NYC\nBob,30,SF";
let csv_stream = adapter.from_csv(csv).unwrap();
// Result: row0:name="Alice"; row0:age="25"; row0:city="NYC"; row1:name="Bob"; row1:age="30"; row1:city="SF"
```

### XStreamAdapter Features

```rust
// Split streams by namespace with automatic colors
let processed = adapter.split_and_process(input, &["ui", "db"]);

// Merge with filtering
let streams = &["a=\"1\"", "b=\"2\"; c=\"3\"", "d=\"4\"; e=\"5\"; f=\"6\""];
let merged = adapter.merge_and_filter(streams, 2); // Only streams with 2+ tokens

// Pipeline builder
let pipeline = XStreamAdapter::pipeline()
    .fork(&["ui", "db"])
    .gate(1)
    .merge(MergeStrategy::Concat);

let result = pipeline.execute(input).unwrap();
```

## 🏗️ User-Friendly Macros

### Stream Processing Macros

```rust
use xstream::{xstream, fork_colored, pipeline, generate_stream};

let input = "ui:theme=\"dark\"; db:host=\"localhost\"";

// Fluent API macro
let result = xstream!(input)
    .fork_by(&["ui", "db"])
    .merge_with(MergeStrategy::Concat)
    .collect();

// Fork with automatic colors
let colored = fork_colored!(input, "ui", "db");

// Pipeline operations
let pipeline_result = pipeline!(
    input => fork(["ui", "db"]) 
          => gate(min_tokens: 1) 
          => merge(MergeStrategy::Interleave)
);
```

### Testing Utilities

```rust
use xstream::{test_stream, generate_stream, validate_stream};

// Generate test data
let test_data = generate_stream!(
    namespaces: ["ui", "db", "api"], 
    tokens_per_ns: 3, 
    colors: true
);

// Test stream operations
let result = test_stream!(input, "expected_content", |s| {
    s.fork_by(&["ui"]).merge_with(MergeStrategy::Sort)
});
result.assert_passed();

// Validate stream format
validate_stream!("key=\"value\"; other=\"data\"").unwrap();
```

### Pipeline Builder Macro

```rust
// Build reusable pipelines
let pipeline = build_pipeline!(
    fork => ["ui", "db", "log"];
    gate => 1;
    merge => MergeStrategy::Concat
);

let result = pipeline.execute(input).unwrap();

// Conditional processing
let result = process_if!(
    stream, 
    has_namespace("ui") => fork(["ui", "ux"]) => merge(MergeStrategy::Sort)
);
```

## 📊 Testing Your Code

### Built-in Test Helpers

```rust
use xstream::{XStreamAdapter, test_stream, generate_stream};

// Generate test streams for different scenarios
let test_streams = XStreamAdapter::test_streams(&["ui", "db"], 3);
for stream in test_streams {
    println!("Test stream: {}", stream);
}

// Test with expected results
let test_result = test_stream!("ui:theme=\"dark\"", "theme", |processor| {
    processor.fork_by(&["ui"]).collect()
});

if test_result.passed {
    println!("✓ Test passed!");
} else {
    test_result.print_result(); // Colored output
}

// Validate stream format
match validate_stream!("invalid_format") {
    Ok(()) => println!("Valid stream"),
    Err(e) => println!("Invalid: {}", e),
}
```

### Integration Testing

```rust
// Test JSON roundtrip
let mut adapter = XStreamAdapter::new();
let original_json = r#"{"test": "value"}"#;
let stream = adapter.from_json(original_json).unwrap();
let converted_back = adapter.to_json(&stream).unwrap();

// Test CSV processing
let csv = "id,name\n1,Alice\n2,Bob";
let csv_stream = adapter.from_csv(csv).unwrap();
assert!(csv_stream.contains("row0:id=\"1\""));

// Test pipeline with real data
let pipeline = XStreamAdapter::pipeline()
    .fork(&["row0", "row1"])
    .gate(2)  // Only rows with 2+ fields
    .merge(MergeStrategy::Concat);

let result = pipeline.execute(&csv_stream).unwrap();
```

## 📖 Examples

### 1. Web Application Stream Processing

```rust
// Modern approach with adapter
let mut adapter = XStreamAdapter::new();
let requests = "ui:click=\"btn1\"; api:req=\"/data\"; ui:hover=\"menu\"; db:query=\"users\"";

// Process with fluent API
let result = adapter.process_stream(requests)
    .fork_by(&["ui", "api", "db"])
    .gate_min_tokens(2)  // Only namespaces with high activity
    .merge_with(MergeStrategy::Concat)
    .collect();

// Or use macros
let quick_result = pipeline!(
    requests => fork(["ui", "api", "db"]) 
             => gate(min_tokens: 2) 
             => merge(MergeStrategy::Concat)
);

// Traditional approach (still available)
let by_component = requests.stream_apply(ForkAll, ());
let filtered = by_component.stream_apply(Gate, GateCondition::MinTokens(2));
let ready = filtered.stream_apply(Merge, MergeStrategy::Concat);
```

### 2. Data Pipeline with Error Recovery

```rust
// JSON data pipeline
let mut adapter = XStreamAdapter::new();
let json_data = r#"{
  "input": {"file": "data.csv", "size": "1MB"},
  "validate": {"schema": "pass", "rows": "1000"},
  "transform": {"clean": "ok", "format": "json"}
}"#;

let stream = adapter.from_json(json_data).unwrap();

// Pipeline with error handling
let pipeline = XStreamAdapter::pipeline()
    .fork(&["input", "validate", "transform"])
    .gate(2)  // Ensure each stage has required data
    .merge(MergeStrategy::Dedupe);

match pipeline.execute(&stream) {
    Ok(result) => println!("Pipeline success: {}", result),
    Err(e) => println!("Pipeline error: {}", e),
}

// Traditional approach
let validated = stream.stream_apply(Gate, GateCondition::RequireNamespace("validate".to_string()));
let stage1 = validated.stream_apply(Fork, vec!["transform".to_string()]);
let final_result = stage1.stream_apply(Merge, MergeStrategy::Dedupe);
```

### 3. Real-time System Monitoring

```rust
// CSV metrics from monitoring system
let csv_metrics = "component,metric,value\ncpu,usage,75\nmem,free,2GB\ndisk,io,high\ncpu,temp,65C";

let mut adapter = XStreamAdapter::new();
let stream = adapter.from_csv(csv_metrics).unwrap();

// Group and filter critical metrics
let critical = adapter.split_and_process(&stream, &["row0", "row1", "row2", "row3"]);
let filtered = adapter.merge_and_filter(&critical.iter().map(|s| s.as_str()).collect::<Vec<_>>(), 2);

// Or use pipeline macro
let alerts = pipeline!(
    stream => fork(["row0", "row1", "row2", "row3"]) 
           => gate(min_tokens: 2) 
           => merge(MergeStrategy::Interleave)
);

println!("Alert stream: {}", alerts);
```

## 🛠 Development

### Project Structure

```
xstream/
├── src/
│   ├── lib.rs              # Library root with re-exports
│   ├── adapter.rs          # XStreamAdapter for easy integration
│   ├── macros.rs           # User-friendly macros
│   ├── driver.rs           # Visual ceremony driver
│   ├── pretty.rs           # Stream formatting utility
│   ├── colors.rs           # Color system implementation
│   ├── bin/                # Binary tools
│   │   ├── xstream-gen.rs     # Stream generator
│   │   └── xstream-color-gen.rs # Color generator
│   └── xstream/            # Core operations
│       ├── mod.rs             # Module definitions
│       ├── types/             # Type definitions
│       ├── fork.rs           # Fork operations
│       ├── merge.rs          # Merge strategies
│       ├── gate.rs           # Gate controls
│       ├── real_fork.rs      # Advanced fork implementations
│       ├── real_gate.rs      # Advanced gate implementations
│       └── real_merge.rs     # Advanced merge implementations
├── examples/
│   └── integration_demo.rs # Integration tools demonstration
├── bin/                    # Test and showcase scripts
│   ├── test.sh              # Main test suite
│   ├── run-all-tests.sh     # Complete test runner
│   ├── showcase-*.sh        # Individual demos
│   └── test_clean.sh        # Cleanup utilities
└── Cargo.toml              # Dependencies and binary definitions
```

### Contributing

1. **Fork the repository** and create your feature branch
2. **Add tests** for new functionality using the ceremony pattern
3. **Run the test suite**: `./bin/test.sh all --verbose`
4. **Add visual demonstrations** to the driver if applicable
5. **Update documentation** and include examples
6. **Submit a pull request** with clear description

### Adding New Operations

```rust
// 1. Define your operation struct
pub struct CustomOperation;

// 2. Implement StreamApply trait
impl StreamApply<CustomParams> for CustomOperation {
    fn stream_apply(&self, input: &str, params: CustomParams) -> String {
        // Your operation logic here
    }
}

// 3. Add ceremony demonstrations in driver.rs
fn ceremony_custom_operations() {
    print_section_header("CUSTOM OPERATIONS - Your Description");
    // Visual test cases here
}

// 4. Add unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_custom_operation() {
        // Test cases here
    }
}
```

### Running Development Tools

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run specific test categories
cargo test fork_tests
cargo test merge_tests  
cargo test gate_tests

# Generate documentation
cargo doc --open

# Build all binaries
cargo build --bins

# Profile performance
cargo build --release
cargo run --release --bin xstream-driver all
```

## 📊 Performance

XStream is designed for efficiency:

- **Zero-copy** string operations where possible
- **Lazy evaluation** of stream transformations  
- **Memory-efficient** token parsing and manipulation
- **Parallel-ready** operation design (futures compatibility)
- **Minimal allocations** in hot paths

Performance testing can be done with:

```bash
# Release build for performance testing
cargo build --release

# Time complex ceremonies
time cargo run --release --bin xstream-driver all

# Memory usage profiling
valgrind cargo run --bin xstream-driver pipeline
```

## 🎨 Visual Design Philosophy

XStream follows a "visual-first" approach to stream processing:

- **Colors preserve identity**: Tokens maintain origin colors through operations
- **Symbols add meaning**: Block symbols (■▲●♦) enhance visual tracking  
- **Ceremonies demonstrate**: Each operation has comprehensive visual tests
- **ASCII art flows**: Clear visual representation of data transformation
- **Status colors**: Success ✓, Error ✗, Warning ⚠, Info ℹ indicators

This makes XStream excellent for:
- **Learning stream processing concepts**
- **Debugging complex data flows**  
- **Demonstrating system behavior**
- **Teaching functional programming patterns**

## 🔗 Dependencies

- **RSB Framework**: Core stream processing capabilities via Git
- **Serde JSON**: JSON serialization/deserialization for adapter integration
- **Clap 4.0**: Command-line argument parsing for generators
- **Base64**: Encoding utilities for token serialization
- **URL Encoding**: Safe string handling for web contexts  
- **Regex**: Pattern matching for advanced stream operations

## 📄 License

XStream is built on the RSB framework. Please refer to the RSB repository for licensing terms and conditions.

---

**Ready to start streaming?** Begin with `cargo run --bin xstream-driver help` to explore the visual ceremonies and see XStream in action! 🎉