# XStream Transformers

## Philosophy

**Terse API**: One-word methods where possible (`.upper()` not `.uppercase_values()`)  
**Dumb Markers**: `tx::` flags are just markers - each transformer decides what they mean  
**Fluent Chains**: Method chaining for complex transformations  
**RSB Integration**: Built on RSB's `stream!` macro and sed operations  

## tx Enum - Transform Markers

```rust
pub enum tx {
    // Operations
    ENCODE,    // Context-aware encoding
    DECODE,    // Context-aware decoding  
    ESCAPE,    // Context-aware escaping
    UNESCAPE,  // Context-aware unescaping
    
    // Targets/Categories
    QUOTES,    // Quote characters
    HTML,      // HTML entities
    UNICODE,   // Unicode sequences
    URL,       // URL encoding
    BASE64,    // Base64 encoding
    ALL,       // Everything applicable
    
    // Case
    UPPER,     // Uppercase
    LOWER,     // Lowercase
}
```

## Current Transformers

### Case Transforms
```rust
.upper()     // host="localhost" → host="LOCALHOST"
.lower()     // USER="JOHN" → user="john"
```

### Escape/Unescape
```rust
.esc(tx::QUOTES)    // Say "hello" → Say \"hello\"
.esc(tx::HTML)      // <div>&</div> → &lt;div&gt;&amp;&lt;/div&gt;
.esc(tx::ALL)       // All special chars (quotes, newlines, etc)
.unesc(tx::QUOTES)  // \"hello\" → "hello"
.unesc(tx::HTML)    // &lt;div&gt; → <div>
.unesc(tx::ALL)     // All escaped chars
```

### Encoding/Decoding
```rust
.base64(tx::ENCODE)   // secret → c2VjcmV0
.base64(tx::DECODE)   // c2VjcmV0 → secret
.url(tx::ENCODE)      // /api/users → %2Fapi%2Fusers
.url(tx::DECODE)      // %2Fapi%2Fusers → /api/users
.unicode(tx::ENCODE)  // 😀🔥 → \u{1f600}\u{1f525}
.unicode(tx::DECODE)  // \u{1f600} → 😀
```

## Legacy Transform Methods

### Basic String Operations
```rust
.translate(from, to)           // Simple find/replace
.translate_many(&[(a,b), ...]) // Multiple replacements
```

### Quote Management
```rust
.swap_quotes()      // "hello" ↔ 'hello'
.double_quotes()    // Convert all to double quotes
.single_quotes()    // Convert all to single quotes
.strip_quotes()     // Remove all quotes
```

### Namespace Operations  
```rust
.rename_namespace(old, new)   // Rename namespace globally
.rename_key(old, new)         // Rename key across all namespaces
```

### Security
```rust
.mask_sensitive()   // Mask passwords, secrets, tokens with ***
```

### Formatting
```rust
.compact()     // Remove spaces: a="1";b="2"
.expand()      // Add spaces: a="1"; b="2"  
.multiline()   // Each token on new line
.singleline()  // Convert back to single line
```

### Advanced
```rust
.custom(|stream| {...})          // Custom RSB stream operations
.regex(pattern, replacement)     // Direct regex substitution
.remove_matching(pattern)        // Remove tokens matching pattern
.keep_matching(pattern)          // Keep only tokens matching pattern
.sort()                          // Sort tokens alphabetically
```

### Validation & Parsing
```rust
.validate()                      // Check if still parseable
.parse(mode)                     // Parse into TokenBucket
```

## Usage Patterns

### Power Chains
```rust
transform(config)
    .translate("localhost", "prod-server-01")
    .rename_namespace("db", "database")
    .mask_sensitive()
    .esc(tx::QUOTES)
    .base64(tx::ENCODE)
    .expand()
    .to_string()
```

### Simple Operations
```rust
transform(tokens).upper().compact()
transform(data).esc(tx::HTML).validate()
```

### RSB Integration
```rust
transform(stream)
    .custom(|s| s.sed("old", "new").sed("=", " = "))
    .unicode(tx::DECODE)
```

## Design Principles

1. **Each transformer interprets tx flags independently**
2. **Transformers ignore irrelevant flags** 
3. **Method chaining preserves token stream validity**
4. **Terse names for common operations**
5. **RSB stream operations as foundation**
6. **Validation available at any step**

## Future Extensions

The `tx` enum can grow as needed:
- `tx::JSON`, `tx::XML` for new escape contexts
- `tx::HASH_MD5`, `tx::HASH_SHA256` for hashing
- `tx::REVERSE`, `tx::SHUFFLE` for manipulations
- Each transformer decides what to implement

The beauty is in the flexibility - add new `tx` variants without breaking existing code!