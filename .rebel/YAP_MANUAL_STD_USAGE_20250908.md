# 🦊 RSB VIOLATION YAP
**Date**: 2025-09-08
**Target**: /home/xnull/repos/code/rust/oodx/xstream/src/xstream/gen.rs
**Violation Type**: Manual std:: usage instead of RSB macros

## VIOLATION DETECTED 🚨
```rust
// gen.rs lines 188-192
let start_time = std::time::SystemTime::now()  // ❌ Manual std::time usage
    .duration_since(std::time::UNIX_EPOCH)     // ❌ Manual UNIX_EPOCH access
    .unwrap()
    .as_secs();
```

```rust
// gen.rs lines 53-55  
let mut rng = rand::rng();  // ❌ Direct rand usage instead of RSB deps
rng.random_range(min..=max).to_string()  // ❌ Manual conversion instead of RSB string-first
```

Multiple instances of direct `std::collections::HashMap::new()` usage throughout gen.rs

## CANONICAL RSB PATTERN 📚
From RSB Reference lines 394-409:
```rust
// Access third-party crates via rsb
use rsb::deps::rand::{Rng, distributions::Alphanumeric};
use rsb::deps::uuid::Uuid;
use rsb::deps::lazy_static::lazy_static;

let token: String = rsb::deps::rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(8)
    .map(char::from)
    .collect();
```

From RSB Architecture lines 77-83 - String-first operations:
```rust
// RSB: Unix pipe-like operations (familiar mental model)
let results = cat!("access.log")
    .grep("ERROR")
    .cut(4, " ")
    .sort()
    .uniq()
    .to_string();
```

## CORRECTIVE ACTION ⚡
Replace manual std:: usage with RSB patterns:

```rust
// ✅ RSB COMPLIANT VERSION
use rsb::prelude::*;

pub fn gen_timed_stream(line_count: usize) -> String {
    // Use RSB time handling instead of std::time
    let start_time = get_var("TIMESTAMP").parse::<u64>()
        .unwrap_or_else(|| {
            // RSB approach: shell-style timestamp generation
            let timestamp = shell!("date +%s");
            timestamp.trim().parse::<u64>().unwrap_or(0)
        });
    
    let mut lines = Vec::new();
    
    for i in 0..line_count {
        let timestamp = start_time + (i as u64);
        
        // Use RSB string-first patterns
        let tokens = vec![
            format!("time:epoch=\"{}\"", timestamp),
            format!("time:seq=\"{}\"", i),
            gen_token(Some("sensor"), Some("temp"), ValueType::RandomNumber(18, 35)),
            gen_token(Some("sensor"), Some("humidity"), ValueType::RandomNumber(30, 80)),
            gen_token(Some("status"), None, ValueType::FromList),
        ];
        
        lines.push(tokens.join("; "));
    }
    
    lines.join("\n")
}

// Replace rand usage with RSB deps
pub fn gen_token(prefix: Option<&str>, key_name: Option<&str>, value_type: ValueType) -> String {
    // Use RSB deps for randomization
    let mut rng = rsb::deps::rand::thread_rng();
    
    // ... rest of function using RSB patterns
}
```

## REFERENCE 📖
RSB Reference Section "Dependency Re-exports"
- Lines 394-409: Access third-party crates via rsb::deps
- RSB Architecture lines 27-55: String-biased philosophy - avoid complex std:: patterns
- Lines 77-83: Unix pipe-like operations preferred over manual std:: calls