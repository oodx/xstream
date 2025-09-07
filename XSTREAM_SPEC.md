# ProntoDB XStream Specification

## Overview

XStream is a token stream processing library for parsing, collecting, and organizing structured key-value data with hierarchical namespace support.

---

## Token Data Structure

```rust
struct Token {
    namespace: Option<String>,  // None for bare keys
    key: String,
    value: String
}

struct TokenStream {
    tokens: Vec<Token>
}
```

The TokenStream can be:
- Parsed from string format
- Generated to string format  
- Serialized to JSON
- Deserialized from JSON

---

## Stream Parser

### Purpose
Convert semicolon-delimited string format to TokenStream structure.

### Input Format
```
"key1=value1; namespace:key2=value2; key3=value3;"
```

### Parsing Algorithm
1. Split input on ';' to get token strings
2. For each token string:
   - Trim whitespace
   - Skip if empty
   - Split on first '=' to get key_part and value
   - If key_part contains ':' 
     - Split on first ':' to get namespace and key
   - Otherwise namespace is None, key is key_part
3. Create Token and add to TokenStream

### Error Handling
- Missing '=' in token: Skip token
- Empty key: Skip token
- Multiple ':' in key: First is namespace separator, rest part of key
- Multiple '=' in token: First is separator, rest part of value

### Parser API
- `parse(input: &str) -> Result<TokenStream>`
- `Token::from_str(s: &str) -> Option<Token>`

### Example
```
Input: "user=bob; sec:pass=123; meta:ns=todo.work;"
Output: TokenStream {
    tokens: [
        Token { namespace: None, key: "user", value: "bob" },
        Token { namespace: Some("sec"), key: "pass", value: "123" },
        Token { namespace: Some("meta"), key: "ns", value: "todo.work" }
    ]
}
```

---

## Stream Generator

### Purpose
Convert TokenStream structure back to string format.

### Generation Algorithm
1. For each Token in stream:
   - If namespace exists: format as "namespace:key=value"
   - If no namespace: format as "key=value"
2. Join all formatted strings with "; "
3. Optionally append final ";"

### Generator API
- `TokenStream::to_string() -> String`
- `Token::to_string() -> String`

### Example
```
Input: TokenStream {
    tokens: [
        Token { namespace: Some("sec"), key: "user", value: "alice" },
        Token { namespace: None, key: "status", value: "active" }
    ]
}
Output: "sec:user=alice; status=active;"
```

---

## Stream Collector

### Purpose
Group tokens by namespace into bucket structure for easy access.

### Output Structure
```rust
type TokenBucket = HashMap<String, HashMap<String, String>>
// Or more explicitly:
struct TokenBucket {
    namespaces: HashMap<String, NamespaceData>
}
type NamespaceData = HashMap<String, String>
```

### Collection Rules

1. **Prefixed tokens** (`namespace:key=value`):
   - Extract namespace part
   - Add key=value to that namespace's bucket

2. **Plain tokens** (`key=value`):
   - Use current active namespace from `ns=` tokens
   - Default namespace is `"global"` when not specified

3. **Namespace switching** with `ns=` tokens:
   - `ns=colors` switches active namespace to `"colors"`
   - `ns=global` switches back to global namespace
   - All subsequent plain tokens go to active namespace
   - `ns=` tokens themselves are not stored in buckets

4. **All namespaces treated equally**:
   - `"sec"` → just another namespace
   - `"colors"` → just another namespace
   - `"global"` → just another namespace
   - Any string → just another namespace

### Collection Algorithm
```
1. Create empty HashMap for namespaces
2. Initialize active_namespace = "global"
3. For each Token:
   - If key == "ns": Update active_namespace = value, continue
   - Get namespace from token.namespace OR active_namespace
   - Get or create inner HashMap for that namespace
   - Insert key=value into inner HashMap
4. Return namespace buckets
```

### Collector API
- `new() -> Collector`
- `add_token(token: Token)`
- `get_data() -> HashMap<String, HashMap<String, String>>`
- `reset()`
- `collect(stream: TokenStream) -> HashMap<String, HashMap<String, String>>`

---

## Examples

### Basic Namespaced Tokens
```
Input: "sec:user=bob; sec:pass=123; dog:lucy=hound; dog:sally=pitbull; cat:tom=tabby;"

Output: {
  "sec": {"user": "bob", "pass": "123"},
  "dog": {"lucy": "hound", "sally": "pitbull"},
  "cat": {"tom": "tabby"}
}
```

### Namespace Switching with ns=
```
Input: "ns=animals; dog=fido; cat=fluffy; ns=colors; red=#FF0000; blue=#0000FF;"

Output: {
  "animals": {"dog": "fido", "cat": "fluffy"},
  "colors": {"red": "#FF0000", "blue": "#0000FF"}
}
```

### Mixed Tokens (Prefixed vs Plain)
```
Input: "item=value1; work:task=urgent; ns=personal; note=reminder; ns=global; final=done;"

Output: {
  "global": {"item": "value1", "final": "done"},
  "work": {"task": "urgent"},
  "personal": {"note": "reminder"}
}
```

### Prefixed Tokens Ignore Active Namespace
```
Input: "tok1=val1; ns=color; tok4=val4; meta:p=q; sec:user=bob;"

Output: {
  "global": {"tok1": "val1"},
  "color": {"tok4": "val4"},
  "meta": {"p": "q"},        // Ignores ns=color
  "sec": {"user": "bob"}     // Ignores ns=color
}
```

---

## Serialization Formats

### TokenStream as JSON
```json
{
  "tokens": [
    {"namespace": "sec", "key": "user", "value": "bob"},
    {"namespace": null, "key": "status", "value": "active"}
  ]
}
```

### Collected Buckets as JSON
```json
{
  "sec": {"user": "bob", "pass": "123"},
  "global": {"status": "active"}
}
```

### Conversion Methods
TokenStream provides:
- `to_string() -> String` (stream format)
- `to_json() -> String` (JSON format)
- `to_buckets() -> HashMap` (collected format)
- `from_string(s: &str) -> Result<TokenStream>`
- `from_json(s: &str) -> Result<TokenStream>`

This allows full round-trip conversion between all formats.