# Streamable API Design Discussion

## Current Issue
The streamable structs (`Replace`, `UpperCase`, etc.) are **exposed** to users, but they should be **hidden implementation details**.

## Current API (Exposed Structs)
```rust
use xstream::{Replace, UpperCase, FilterTokens, Base64Encode};

transform(data)
    .stream_apply(Replace, ("old".to_string(), "new".to_string()))
    .stream_apply(UpperCase, ())
    .stream_apply(Base64Encode, ())
```

**Problems:**
- Users must import struct names
- Implementation details are exposed
- Zero-sized structs pollute namespace
- Not ergonomic

## Proposed Solutions

### Option 1: Method-based (Clean)
```rust
transform(data)
    .apply_replace("old", "new")
    .apply_uppercase() 
    .apply_base64_encode()
```

**Pros:** Clean, discoverable, no imports needed
**Cons:** Many methods on TokenStream, potential naming conflicts

### Option 2: Function-based 
```rust
transform(data)
    .stream_apply(replace_fn, ("old", "new"))
    .stream_apply(uppercase_fn, ())
```

**Pros:** Single `stream_apply` method, functions can be modular
**Cons:** Still need to import function names

### Option 3: String-based (Most Flexible)
```rust
transform(data)
    .apply("replace", ("old", "new"))
    .apply("uppercase", ())
    .apply("base64_encode", ())
```

**Pros:** No imports, runtime flexibility, extensible
**Cons:** No compile-time checking, string literals

### Option 4: Enum-based
```rust
use xstream::StreamOp;

transform(data)
    .apply(StreamOp::Replace("old", "new"))
    .apply(StreamOp::Uppercase)
    .apply(StreamOp::Base64Encode)
```

**Pros:** Type safety, single import, pattern matching
**Cons:** Enum can get large, boxing issues for different arg types

## Key Design Questions

1. **Ergonomics vs Type Safety**: String-based is most ergonomic but loses compile-time checking
2. **Discoverability**: Method-based is most discoverable via IDE autocomplete
3. **Extensibility**: How easy to add new streamable functions?
4. **Import Story**: What should users need to import?
5. **RSB Integration**: Which pattern works best when rolled back to RSB?

## Implementation Strategy

The structs should be:
- **Private** (`pub(crate)` or module-private)
- **Created internally** by the chosen API
- **Hidden from docs** and public interface

## Next Steps

- Decide on API pattern
- Refactor to hide implementation details
- Update driver examples
- Document the chosen pattern
- Prepare for RSB integration

## Questions for Discussion

- Which API feels most "unix-like"?
- How important is compile-time type checking vs runtime flexibility?
- Should we support custom/user-defined streamable functions?
- How does this integrate with RSB's existing patterns?