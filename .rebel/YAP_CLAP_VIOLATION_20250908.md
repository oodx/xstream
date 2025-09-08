# 🦊 RSB VIOLATION YAP
**Date**: 2025-09-08
**Target**: /home/xnull/repos/code/rust/oodx/xstream/src/bin/xstream-gen.rs, xstream-color-gen.rs
**Violation Type**: clap/arg parsing instead of RSB patterns

## VIOLATION DETECTED 🚨
```rust
// xstream-gen.rs - Lines 5-77
use clap::{Parser, Subcommand};

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
        // ... more clap derivatives
    }
}

fn main() {
    let cli = Cli::parse();  // ❌ CLAP PARSING - NOT RSB!
    // ...
}
```

**Same violation in xstream-color-gen.rs lines 5-100**

## CANONICAL RSB PATTERN 📚
From /home/xnull/repos/code/rust/oodx/rebel/docs/ref/rsb-reference.md lines 15-51:
```rust
use rsb::prelude::*;

fn main() {
    // 1. Initialize RSB, load environment, and get command-line args.
    let args = bootstrap!();

    // 2. (Optional) Run "pre-config" commands like `install` or `init`.
    if pre_dispatch!(&args, { "install" => do_install }) {
        return; // Exit if a pre_dispatch command was run.
    }

    // 3. Load configuration files. Values in later files override earlier ones.
    info!("Loading configuration...");
    src!("~/.config/myapp/config.conf", "./app.conf");

    // 4. Route the main command to its handler function.
    dispatch!(&args, {
        "build"   => do_build,
        "deploy"  => do_deploy,
        "config"  => do_manage_config
    });
}

// Handler functions receive an `Args` struct and return an exit code (0 for success).
fn do_build(mut args: Args) -> i32 {
    let target = args.get_or(1, "debug");
    info!("Building target: {}", target);
    // ... logic ...
    okay!("Build successful.");
    0
}
```

## CORRECTIVE ACTION ⚡
Replace clap-based parsers with RSB standard interface:

```rust
// src/bin/xstream-gen.rs - RSB COMPLIANT VERSION
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    
    dispatch!(&args, {
        "colored" => do_generate_colored,
        "precolored" => do_generate_precolored, 
        "pattern" => do_generate_pattern
    });
}

fn do_generate_colored(mut args: Args) -> i32 {
    let namespaces = args.get_or(1, "ui,db,api");
    let tokens = args.get_or(2, "3").parse::<usize>().unwrap_or(3);
    let format = args.get_or(3, "stream"); 
    let symbols = args.has_pop("--symbols");
    
    let ns_list: Vec<&str> = namespaces.split(',').collect();
    let result = generate_colored_stream(&ns_list, tokens, &format, symbols);
    echo!("{}", result);
    0
}

fn do_generate_precolored(mut args: Args) -> i32 {
    let count = args.get_or(1, "10").parse::<usize>().unwrap_or(10);
    let theme = args.get_or(2, "rainbow");
    
    let result = generate_pre_colored_tokens(count, theme);
    echo!("{}", result);
    0
}

fn do_generate_pattern(mut args: Args) -> i32 {
    require_var!("PATTERN_TYPE");
    let pattern = param!("PATTERN_TYPE");
    let complexity = args.get_or(1, "medium");
    
    let result = generate_pattern_stream(&pattern, complexity);
    echo!("{}", result);
    0
}
```

## REFERENCE 📖
RSB Architecture Framework Section 1.2: "The Args Parser"
- Use `bootstrap!()` instead of clap::Parser
- Use `dispatch!()` macro for command routing
- Use `Args` methods: `.get_or()`, `.has_pop()`, `.get_kv()` etc.
- Functions must return i32 exit codes, not void

**RSB Reference Line 57-66**: Args struct provides simple API for parsing without complex clap derivatives