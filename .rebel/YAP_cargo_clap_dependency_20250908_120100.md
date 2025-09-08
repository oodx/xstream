# 🦊 RSB VIOLATION YAP
**Date**: 2025-09-08 12:01:00
**Target**: /home/xnull/repos/code/rust/oodx/xstream/Cargo.toml
**Violation Type**: Including clap dependency contrary to RSB philosophy

## VIOLATION DETECTED 🚨
Cargo.toml includes clap as a dependency, which violates RSB's string-first command parsing philosophy:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

This dependency is only used by the violating binary targets (xstream-gen.rs and xstream-color-gen.rs), but contradicts RSB principles.

## CANONICAL RSB PATTERN 📚
From `/home/xnull/repos/code/rust/oodx/rebel/docs/ref/rsb-reference.md`, section 1.2:

> The `Args` struct, passed to every handler, provides a simple API for parsing arguments. It automatically tracks consumed arguments.

RSB provides built-in argument parsing that eliminates the need for external dependencies like clap:

```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    dispatch!(&args, {
        "command1" => do_command1,
        "command2" => do_command2
    });
}

fn do_command1(mut args: Args) -> i32 {
    let value = args.get_or(1, "default");
    let flag = args.has_pop("--flag");
    let option = args.has_val("--option");
    // ... logic
    0
}
```

## CORRECTIVE ACTION ⚡

**Remove clap dependency from Cargo.toml:**
```toml
[dependencies]
# Remove this line: clap = { version = "4.0", features = ["derive"] }
rsb = { git = "https://github.com/oodx/rebel", branch = "main" }
base64 = "0.22"
urlencoding = "2.1"
regex = "1.10"
```

**Refactor binaries to use RSB Args pattern** (as shown in previous YAP file):
- xstream-gen.rs needs complete refactor to use `bootstrap!()` and `dispatch!()`
- xstream-color-gen.rs needs complete refactor to use `bootstrap!()` and `dispatch!()`

## RSB PHILOSOPHY COMPLIANCE ⚡

RSB's string-first philosophy means:
1. **No complex derive macros** - Simple string handling instead
2. **No external parsing dependencies** - RSB provides all necessary parsing
3. **Consistent RSB patterns** - `bootstrap!()`, `dispatch!()`, `Args` struct
4. **String-biased interfaces** - Everything is a string until proven otherwise

The proper RSB approach eliminates clap entirely by using RSB's built-in argument handling.

## REFERENCE 📖
- RSB Reference Guide: `/home/xnull/repos/code/rust/oodx/rebel/docs/ref/rsb-reference.md` Section 1.2
- RSB Architecture Guide: `/home/xnull/repos/code/rust/oodx/rebel/docs/ref/rsb-architecture.md`
- RSB Framework Source: `/home/xnull/repos/code/rust/oodx/rebel/src/args.rs`

🦊 **SEVERITY**: MAJOR VIOLATION - Including external parsing library contradicts RSB string-first philosophy