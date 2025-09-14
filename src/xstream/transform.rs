// XStream Transform - Power chains for token stream transformations using RSB

use rsb::prelude::*;
use rsb::deps::{base64, urlencoding};
use crate::xstream::types::{TokenBucket, BucketMode, is_token_streamable};
// xsed now available from rsb::prelude via xcls module

/// TX - Transform markers/flags (just dumb markers, transformers decide what they mean)
#[derive(Debug, Clone, Copy)]
pub enum TX {
    // Operations
    ENCODE,
    DECODE,
    ESCAPE,
    UNESCAPE,
    
    // Targets/Categories
    QUOTES,
    HTML,
    UNICODE,
    URL,
    BASE64,
    ALL,
    
    // Case
    UPPER,
    LOWER,
}

/// TokenStream provides transformation chains for token streams
pub struct TokenStream {
    content: String,
}

impl TokenStream {
    /// Create a new TokenStream from a string
    pub fn new(content: impl Into<String>) -> Self {
        TokenStream {
            content: content.into(),
        }
    }
    
    /// Create from an existing token stream string
    pub fn from(content: impl Into<String>) -> Self {
        Self::new(content)
    }
    
    /// Translate all occurrences of pattern to replacement
    /// This is a sugar wrapper around RSB's sed
    pub fn translate(self, from: &str, to: &str) -> Self {
        let result = stream!(string: &self.content)
            .sed(from, to)
            .to_string();
        TokenStream::new(result)
    }
    
    /// Translate multiple pairs in sequence
    pub fn translate_many(self, pairs: &[(&str, &str)]) -> Self {
        let mut result = self.content;
        for (from, to) in pairs {
            result = stream!(string: &result)
                .sed(from, to)
                .to_string();
        }
        TokenStream::new(result)
    }
    
    /// Change quote style (double to single or vice versa)
    pub fn swap_quotes(self) -> Self {
        self.translate("=\"", "='")
            .translate("\"", "'")
    }
    
    /// Change all quotes to double quotes
    pub fn double_quotes(self) -> Self {
        self.translate("='", "=\"")
            .translate("'", "\"")
    }
    
    /// Change all quotes to single quotes  
    pub fn single_quotes(self) -> Self {
        self.translate("=\"", "='")
            .translate("\"", "'")
    }
    
    /// Remove all quotes from values
    pub fn strip_quotes(self) -> Self {
        self.translate("=\"", "=")
            .translate("='", "=")
            .translate("\"", "")
            .translate("'", "")
    }
    
    /// Add quotes to unquoted values (assumes values don't contain spaces/semicolons)
    pub fn add_quotes(self, quote_char: char) -> Self {
        // Match = followed by non-quote, capture until ; or end
        let pattern = format!("=([^{};]+)", quote_char);
        let replacement = format!("={}$1{}", quote_char, quote_char);
        
        let result = stream!(string: &self.content)
            .sed(&pattern, &replacement)
            .to_string();
        TokenStream::new(result)
    }
    
    /// Rename a namespace globally
    pub fn rename_namespace(self, old_ns: &str, new_ns: &str) -> Self {
        // Handle both ns= switches and prefixed tokens
        self.translate(&format!("ns={}", old_ns), &format!("ns={}", new_ns))
            .translate(&format!("{}:", old_ns), &format!("{}:", new_ns))
    }
    
    /// Rename a key across all namespaces
    pub fn rename_key(self, old_key: &str, new_key: &str) -> Self {
        // Match key in both prefixed and non-prefixed forms
        self.translate(&format!(":{}=", old_key), &format!(":{}=", new_key))
            .translate(&format!(" {}=", old_key), &format!(" {}=", new_key))
            .translate(&format!(";{}=", old_key), &format!(";{}=", new_key))
            // Handle start of line
            .translate_if_starts(&format!("{}=", old_key), &format!("{}=", new_key))
    }
    
    /// Translate only if the line starts with pattern
    pub fn translate_if_starts(self, from: &str, to: &str) -> Self {
        let result = if self.content.starts_with(from) {
            format!("{}{}", to, &self.content[from.len()..])
        } else {
            self.content
        };
        TokenStream::new(result)
    }
    
    /// Add a prefix to all namespaces
    pub fn prefix_namespaces(self, prefix: &str) -> Self {
        // Add prefix to ns= switches
        let result = stream!(string: &self.content)
            .sed("ns=", &format!("ns={}.", prefix))
            .to_string();
        
        // Also prefix any existing namespace prefixes in tokens
        // This is tricky - need to identify token patterns
        TokenStream::new(result)
    }
    
    /// Transform all values using a regex pattern
    pub fn transform_values(self, pattern: &str, replacement: &str) -> Self {
        let result = stream!(string: &self.content)
            .sed(pattern, replacement)
            .to_string();
        TokenStream::new(result)
    }
    
    /// Mask sensitive values (passwords, secrets, tokens)
    pub fn mask_sensitive(self) -> Self {
        self.translate_many(&[
            ("pass=\"", "pass=\"***"),
            ("password=\"", "password=\"***"),
            ("secret=\"", "secret=\"***"),
            ("token=\"", "token=\"***"),
            ("key=\"", "key=\"***"),
            ("pass='", "pass='***"),
            ("password='", "password='***"),
            ("secret='", "secret='***"),
            ("token='", "token='***"),
            ("key='", "key='***"),
        ])
        // Now remove everything after *** until quote
        .transform_values(r#"\*\*\*[^"']*"#, "***\"")
        .transform_values(r#"\*\*\*[^"']*'"#, "***'")
    }
    
    /// Compact format - remove spaces after semicolons
    pub fn compact(self) -> Self {
        self.translate("; ", ";")
    }
    
    /// Expand format - ensure space after semicolons
    pub fn expand(self) -> Self {
        self.translate(";", "; ")
            .translate(";  ", "; ") // Fix double spaces
    }
    
    /// Add line breaks after each token for readability
    pub fn multiline(self) -> Self {
        self.translate("; ", ";\n")
    }
    
    /// Convert back to single line
    pub fn singleline(self) -> Self {
        self.translate(";\n", "; ")
            .translate("\n", "; ")
    }
    
    /// Validate the stream is still parseable
    pub fn validate(&self) -> bool {
        is_token_streamable(&self.content)
    }
    
    /// Get the transformed content
    pub fn to_string(self) -> String {
        self.content
    }
    
    /// Parse into a TokenBucket
    pub fn parse(self, mode: BucketMode) -> Result<TokenBucket, String> {
        TokenBucket::from_str(&self.content, mode)
            .map_err(|e| format!("Parse error: {:?}", e))
    }
    
    /// Chain with a custom RSB stream operation
    pub fn custom<F>(self, f: F) -> Self 
    where
        F: FnOnce(Stream) -> Stream
    {
        let stream = stream!(string: &self.content);
        let result = f(stream).to_string();
        TokenStream::new(result)
    }
    
    /// Apply a regex substitution using RSB's sed
    pub fn regex(self, pattern: &str, replacement: &str) -> Self {
        let result = stream!(string: &self.content)
            .sed(pattern, replacement)
            .to_string();
        TokenStream::new(result)
    }
    
    /// Remove tokens matching a pattern
    pub fn remove_matching(self, pattern: &str) -> Self {
        // Remove pattern and cleanup any doubled semicolons
        self.regex(&format!(r#"{}[^;]*;?\s*"#, pattern), "")
            .translate(";;", ";")
            .translate("; ;", ";")
    }
    
    /// Keep only tokens matching a pattern
    pub fn keep_matching(self, pattern: &str) -> Self {
        // This is complex - we need to identify what to keep
        // For now, use a simple approach
        let tokens: Vec<&str> = self.content.split(';').collect();
        let kept: Vec<String> = tokens
            .iter()
            .filter(|t| t.contains(pattern))
            .map(|t| t.trim().to_string())
            .collect();
        TokenStream::new(kept.join("; "))
    }
    
    /// Sort tokens alphabetically (preserves namespaces)
    pub fn sort(self) -> Self {
        let mut tokens: Vec<&str> = self.content.split(';').collect();
        tokens.sort();
        let sorted = tokens
            .iter()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join("; ");
        TokenStream::new(sorted)
    }
    
    // === TERSE TRANSFORM METHODS ===
    
    /// Transform to upper or lower case
    pub fn upper(self) -> Self {
        // Use xsed's transform_values with closure!
        TokenStream::new(xsed(&self.content)
            .transform_values(|v| v.to_uppercase())
            .to_string())
    }
    
    pub fn lower(self) -> Self {
        TokenStream::new(xsed(&self.content)
            .transform_values(|v| v.to_lowercase())
            .to_string())
    }
    
    /// Escape based on TX flag
    pub fn esc(self, what: TX) -> Self {
        match what {
            TX::QUOTES => {
                self.translate("\"", "\\\"")
            },
            TX::HTML => {
                self.translate("&", "&amp;")
                    .translate("<", "&lt;")
                    .translate(">", "&gt;")
                    .translate("\"", "&quot;")
            },
            TX::ALL => {
                self.translate("\\", "\\\\")
                    .translate("\"", "\\\"")
                    .translate("\n", "\\n")
                    .translate("\r", "\\r")
                    .translate("\t", "\\t")
            },
            _ => self, // Ignore non-applicable flags
        }
    }
    
    /// Unescape based on tx flag
    pub fn unesc(self, what: TX) -> Self {
        match what {
            TX::QUOTES => {
                self.translate("\\\"", "\"")
            },
            TX::HTML => {
                self.translate("&quot;", "\"")
                    .translate("&lt;", "<")
                    .translate("&gt;", ">")
                    .translate("&amp;", "&")
            },
            TX::ALL => {
                self.translate("\\n", "\n")
                    .translate("\\r", "\r")
                    .translate("\\t", "\t")
                    .translate("\\\"", "\"")
                    .translate("\\\\", "\\")
            },
            _ => self,
        }
    }
    
    /// Base64 encode/decode
    pub fn base64(self, op: TX) -> Self {
        use base64::{Engine as _, engine::general_purpose};
        
        match op {
            TX::ENCODE => {
                let result = self.transform_values_with(|v| {
                    general_purpose::STANDARD.encode(v.as_bytes())
                });
                TokenStream::new(result)
            },
            TX::DECODE => {
                let result = self.transform_values_with(|v| {
                    general_purpose::STANDARD
                        .decode(v)
                        .ok()
                        .and_then(|bytes| String::from_utf8(bytes).ok())
                        .unwrap_or(v.to_string())
                });
                TokenStream::new(result)
            },
            _ => self,
        }
    }
    
    /// URL encode/decode
    pub fn url(self, op: TX) -> Self {
        match op {
            TX::ENCODE => {
                let result = self.transform_values_with(|v| {
                    urlencoding::encode(v).to_string()
                });
                TokenStream::new(result)
            },
            TX::DECODE => {
                let result = self.transform_values_with(|v| {
                    urlencoding::decode(v)
                        .map(|s| s.to_string())
                        .unwrap_or(v.to_string())
                });
                TokenStream::new(result)
            },
            _ => self,
        }
    }
    
    /// Unicode encode/decode
    pub fn unicode(self, op: TX) -> Self {
        match op {
            TX::ENCODE => {
                let result = self.transform_values_with(|v| {
                    v.chars()
                        .map(|c| {
                            if c.is_ascii() {
                                c.to_string()
                            } else {
                                format!("\\u{{{:x}}}", c as u32)
                            }
                        })
                        .collect()
                });
                TokenStream::new(result)
            },
            TX::DECODE => {
                // Decode \u{1F600} style escapes - simplified for now
                // TODO: Implement proper unicode decoding with xsed
                self
            },
            _ => self,
        }
    }
    
    // Helper to transform just values
    fn transform_values_with<F>(&self, f: F) -> String 
    where 
        F: Fn(&str) -> String
    {
        let tokens: Vec<&str> = self.content.split(';').collect();
        let transformed: Vec<String> = tokens
            .iter()
            .map(|token| {
                if let Some(eq_pos) = token.find('=') {
                    let (key, value) = token.split_at(eq_pos + 1);
                    let clean_value = value.trim_matches('"').trim_matches('\'');
                    format!("{}\"{}\"", key, f(clean_value))
                } else {
                    token.to_string()
                }
            })
            .collect();
        transformed.join(";")
    }
}

// Convenience function for creating a TokenStream
pub fn transform(content: impl Into<String>) -> TokenStream {
    TokenStream::new(content)
}