# XStream 🚀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](#)

> **Revolutionary Token Stream Processing** - Fluent, composable, and powerful stream transformations built on the RSB framework.

XStream is a next-generation token processing library that provides three distinct API tiers for maximum flexibility: from fluent transformation chains to composable streamables to raw RSB power. Whether you're processing configuration tokens, transforming data streams, or building complex token pipelines, XStream adapts to your needs.

## ✨ Key Features

- 🔥 **Three-Tier API Architecture** - Choose your abstraction level
- ⚡ **Built on RSB/REBEL** - Industrial-strength stream processing foundation
- 🎯 **Token-Aware Processing** - Native understanding of `key="value"` token formats
- 🔗 **Fluent Transformation Chains** - Intuitive method chaining for complex operations
- 🧩 **Composable Streamables** - Reusable, combinable processing components
- 🚀 **Terse Transform API** - Compact operations with `tx::` markers
- 🛡️ **Type-Safe Operations** - Rust's safety guarantees throughout
- 📦 **Namespace Support** - Handle complex token hierarchies with ease

## 🔧 Quick Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xstream = { git = "https://github.com/oodx/xstream", branch = "main" }
```

Or clone and build locally:

```bash
git clone https://github.com/oodx/xstream.git
cd xstream
cargo build --release
```

## 📖 Basic Usage

### Tier 1: Fluent Transform API (Recommended for most users)

The fluent API provides intuitive method chaining for common token operations:

```rust
use xstream::prelude::*;

// Transform a token stream with method chaining
let tokens = "user=\"alice\"; pass=\"secret123\"; role=\"admin\"";

let result = transform(tokens)
    .mask_sensitive()           // Hide sensitive values
    .swap_quotes()              // Change quote style
    .upper()                    // Uppercase values
    .compact()                  // Remove extra spaces
    .to_string();

// Output: "user='ALICE';pass='***';role='ADMIN'"
```

### Advanced Transformations

```rust
// Complex token manipulation
let config_stream = transform("app:name=\"MyApp\"; app:version=\"1.0\"; debug=\"true\"")
    .rename_namespace("app", "application")
    .add_quotes('"')
    .multiline()
    .to_string();

// Validation and parsing
let parsed = transform(tokens)
    .validate()                 // Check if still parseable
    .parse(BucketMode::Strict)?; // Parse into TokenBucket
```

### Tier 2: Composable Streamables

Perfect for building reusable processing pipelines:

```rust
use xstream::prelude::*;

// Use token-specific streamables
let token_count = stream!(string: tokens)
    .streamable!(TokenCount)
    .to_string();

let keys_only = stream!(string: tokens)
    .streamable!(ExtractKeys)
    .to_string();

let filtered = stream!(string: tokens)
    .streamable!(FilterTokens, key_contains: "user")
    .to_string();
```

### Tier 3: Raw RSB Power

For maximum control and custom operations:

```rust
use rsb::prelude::*;

// Direct RSB stream operations
let result = stream!(string: tokens)
    .sed(r#"pass="[^"]*""#, r#"pass="***""#)
    .sed(";", ";\n")
    .to_string();

// Custom streamable integration
let custom_result = stream!(string: tokens)
    .custom(|s| s.sed("old", "new").map_lines(|line| line.to_uppercase()))
    .to_string();
```

## 🎯 Core Concepts

### Token Format

XStream understands structured token formats:

```
key="value"; ns:subkey="data"; flag="true"
```

- **Simple tokens**: `key="value"`
- **Namespaced tokens**: `namespace:key="value"`
- **Namespace switches**: `ns="namespace"`

### The Three-Tier Architecture

1. **Fluent API (`transform()`)** - High-level, intuitive operations
2. **Composable Streamables** - Reusable, type-safe components
3. **Raw RSB** - Full control, custom operations

### Terse Transform API

Use `tx::` markers for compact operations:

```rust
use xstream::{transform, tx};

let encoded = transform(data)
    .base64(tx::ENCODE)
    .esc(tx::QUOTES)
    .url(tx::ENCODE);
```

Available `tx` markers:
- **Operations**: `ENCODE`, `DECODE`, `ESCAPE`, `UNESCAPE`
- **Targets**: `QUOTES`, `HTML`, `UNICODE`, `URL`, `BASE64`, `ALL`
- **Case**: `UPPER`, `LOWER`

## 🛠️ Common Operations

### Quote Management

```rust
let tokens = transform(data)
    .double_quotes()    // Convert to double quotes
    .single_quotes()    // Convert to single quotes
    .swap_quotes()      // Toggle quote style
    .strip_quotes()     // Remove all quotes
    .add_quotes('"');   // Add quotes to unquoted values
```

### Security & Masking

```rust
let safe_tokens = transform(sensitive_data)
    .mask_sensitive()   // Hide passwords, secrets, keys, tokens
    .esc(tx::HTML)      // Escape HTML entities
    .esc(tx::ALL);      // Escape all special characters
```

### Namespace Operations

```rust
let updated = transform(tokens)
    .rename_namespace("old_ns", "new_ns")
    .rename_key("old_key", "new_key")
    .prefix_namespaces("prefix");
```

### Format Control

```rust
let formatted = transform(data)
    .compact()      // Remove spaces: "a=1;b=2"
    .expand()       // Add spaces: "a=1; b=2"
    .multiline()    // Split to multiple lines
    .singleline()   // Merge to single line
    .sort();        // Sort tokens alphabetically
```

## 📚 API Documentation

### TokenStream Methods

| Method | Purpose | Example |
|--------|---------|---------|
| `translate(from, to)` | Replace all occurrences | `.translate("=", ":")` |
| `regex(pattern, replacement)` | Regex substitution | `.regex(r"\d+", "NUM")` |
| `mask_sensitive()` | Hide sensitive values | Masks passwords, tokens, keys |
| `validate()` | Check parseability | Returns `bool` |
| `parse(mode)` | Parse to TokenBucket | `.parse(BucketMode::Strict)` |
| `custom(fn)` | Custom RSB operation | `.custom(\|s\| s.sed("a", "b"))` |

### Built-in Streamables

| Streamable | Input | Output | Purpose |
|------------|-------|--------|---------|
| `TokenCount` | Token stream | Count | Number of tokens |
| `ExtractKeys` | Token stream | Key list | All token keys |
| `ExtractValues` | Token stream | Value list | All token values |
| `FilterTokens` | Token stream + pattern | Filtered stream | Tokens matching pattern |
| `ExtractNamespaces` | Token stream | Namespace list | All unique namespaces |

## 🚀 Advanced Examples

### Configuration Processing Pipeline

```rust
use xstream::prelude::*;

fn process_config(raw_config: &str) -> Result<String, Box<dyn std::error::Error>> {
    let processed = transform(raw_config)
        // Security first
        .mask_sensitive()
        // Standardize format
        .double_quotes()
        .expand()
        // Organize structure
        .rename_namespace("app", "application")
        .sort()
        // Validate result
        .validate()
        .then(|valid| if valid { Ok(()) } else { Err("Invalid token format") })?
        .to_string();
    
    Ok(processed)
}
```

### Multi-Stage Data Transformation

```rust
// Stage 1: Clean and structure
let stage1 = transform(messy_data)
    .strip_quotes()
    .translate_many(&[("\\n", ""), ("\\t", "")])
    .add_quotes('"');

// Stage 2: Enhance and validate
let stage2 = stage1
    .prefix_namespaces("cleaned")
    .upper()
    .compact();

// Stage 3: Export formats
let json_ready = stage2.clone().esc(tx::QUOTES);
let xml_ready = stage2.esc(tx::HTML);
```

### Custom Streamable Integration

```rust
// Combine built-in and custom operations
let analysis = stream!(string: data)
    .streamable!(TokenCount)            // Count tokens
    .map_lines(|count| format!("Found {} tokens", count))
    .chain(
        stream!(string: data)
        .streamable!(ExtractKeys)       // Get keys
        .map_lines(|keys| format!("Keys: {}", keys))
    )
    .to_string();
```

## 🔍 RSB Integration

XStream is built on the powerful [RSB (REBEL Stream Builder)](https://github.com/oodx/rebel) framework, providing:

- **Industrial-strength streaming**: Battle-tested performance
- **Composable operations**: Mix and match processing components
- **Memory efficiency**: Process large streams without loading everything into memory
- **Error handling**: Robust error propagation and recovery
- **Extensibility**: Easy to add custom streamables and operations

### Direct RSB Access

```rust
use rsb::prelude::*;
use xstream::prelude::*;

// Combine XStream tokens with RSB power
let result = stream!(string: token_data)
    // XStream tokenization
    .streamable!(ExtractValues)
    // Raw RSB processing
    .map_lines(|line| line.to_uppercase())
    .filter_lines(|line| line.len() > 3)
    .take_lines(10)
    // Back to XStream
    .pipe_to(|values| transform(values).add_quotes('"').to_string())
    .to_string();
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature-amazing-transform`
3. **Make your changes** and add tests
4. **Run the test suite**: `cargo test`
5. **Check formatting**: `cargo fmt`
6. **Submit a pull request**

### Development Setup

```bash
# Clone the repository
git clone https://github.com/oodx/xstream.git
cd xstream

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_usage
```

### Code Guidelines

- Follow Rust conventions and idioms
- Add tests for new functionality
- Update documentation for API changes
- Use meaningful commit messages
- Ensure all tests pass before submitting

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Documentation tests
cargo test --doc

# Run with coverage
cargo tarpaulin --out html
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on the powerful [RSB framework](https://github.com/oodx/rebel)
- Inspired by the need for better token stream processing
- Thanks to all contributors and the Rust community

---

**Ready to revolutionize your token processing?** Get started with XStream today! 🚀

*For questions, issues, or feature requests, please [open an issue](https://github.com/oodx/xstream/issues) on GitHub.*