# 🦊 RSB VIOLATION YAP
**Date**: 2025-09-08  
**Target**: Multiple files missing RSB prelude imports
**Violation Type**: Missing `use rsb::prelude::*` imports

## VIOLATION DETECTED 🚨
**Files lacking RSB imports:**

1. `/home/xnull/repos/code/rust/oodx/xstream/src/bin/xstream-gen.rs` - Line 5
```rust
use clap::{Parser, Subcommand};  // ❌ Using clap, no RSB import
use std::collections::HashMap;
```

2. `/home/xnull/repos/code/rust/oodx/xstream/src/bin/xstream-color-gen.rs` - Line 5 
```rust  
use clap::{Parser, Subcommand};  // ❌ Using clap, no RSB import
use std::collections::HashMap;
```

## CANONICAL RSB PATTERN 📚
From RSB Architecture Framework Amendment A lines 915-932:
```rust
// main.rs - Single RSB entry point
use rsb::prelude::*;

// lib modules - use crate imports for RSB functionality
// src/myapp/config.rs
use crate::rsb;  // or similar crate-specific import pattern

pub fn do_load_config() -> String { 
    // RSB macros available via crate import
    param!("CONFIG_PATH", default: "config.toml")
}
```

**From lines 943-952**: "Single Source of Truth: main.rs serves as the RSB gateway for the entire application"

## CORRECTIVE ACTION ⚡
Add proper RSB imports at the top of each binary file:

```rust
// src/bin/xstream-gen.rs - CORRECTED
use rsb::prelude::*;  // ✅ RSB GATEWAY IMPORT
// Remove clap imports - not needed with RSB patterns

fn main() {
    let args = bootstrap!();  // RSB initialization
    // ... RSB patterns
}
```

```rust
// src/bin/xstream-color-gen.rs - CORRECTED  
use rsb::prelude::*;  // ✅ RSB GATEWAY IMPORT
// Remove clap imports - not needed with RSB patterns

fn main() {
    let args = bootstrap!();  // RSB initialization
    // ... RSB patterns  
}
```

## REFERENCE 📖
RSB Architecture Framework Amendment A: "RSB Import Hierarchy Patterns"
- Line 920: "Single-entry-point pattern for RSB framework imports"
- Line 923-926: "main.rs - Single RSB entry point"
- Line 972-975: "Single Source of Truth" principle - main.rs serves as RSB gateway