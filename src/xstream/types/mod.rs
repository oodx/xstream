pub mod namespace;
pub mod token;
pub mod bucket;
pub mod error;
pub mod streamable;

// Re-export public types
pub use namespace::Namespace;
pub use token::{Token, TokenStreamable, tokenize_string, is_token_streamable};
pub use bucket::{TokenBucket, BucketMode, collect_tokens};
pub use error::{TokenBucketError, TokenBucketResult};

// Re-export token-specific streamables
pub use streamable::{
    TokenCount, ExtractKeys, ExtractValues, FilterTokens,
    ExtractNamespaces, FilterByNamespace, TokenValidate,
    TokenToLines, LinesToTokens,
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize() {
        let stream = "user=bob; sec:pass=123; ns=animals; dog=fido;";
        let tokens = stream.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
    }
    
    #[test]
    fn test_namespace_switching() {
        let stream = "item=value1; ns=animals; dog=fido; cat=fluffy; ns=colors; red=#FF0000; ns=global; final=done;";
        let tokens = stream.tokenize().unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Flat);
        
        // Check global namespace
        assert!(bucket.data.contains_key("global"));
        assert_eq!(bucket.data["global"]["item"], "value1");
        assert_eq!(bucket.data["global"]["final"], "done");
        
        // Check animals namespace
        assert!(bucket.data.contains_key("animals"));
        assert_eq!(bucket.data["animals"]["dog"], "fido");
        assert_eq!(bucket.data["animals"]["cat"], "fluffy");
        
        // Check colors namespace
        assert!(bucket.data.contains_key("colors"));
        assert_eq!(bucket.data["colors"]["red"], "#FF0000");
        
        // ns= tokens should not appear in the bucket data
        assert!(!bucket.data.values().any(|map| map.contains_key("ns")));
    }
    
    #[test]
    fn test_prefixed_tokens_ignore_active_namespace() {
        let stream = "tok1=val1; tok2=val2; ns=color; tok4=val4; meta:p=q; sec:user=bob;";
        let tokens = stream.tokenize().unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Flat);
        
        // Check global namespace (tok1, tok2)
        assert!(bucket.data.contains_key("global"));
        assert_eq!(bucket.data["global"]["tok1"], "val1");
        assert_eq!(bucket.data["global"]["tok2"], "val2");
        
        // Check color namespace (tok4 due to ns=color)
        assert!(bucket.data.contains_key("color"));
        assert_eq!(bucket.data["color"]["tok4"], "val4");
        
        // Check meta namespace (meta:p=q ignores active namespace)
        assert!(bucket.data.contains_key("meta"));
        assert_eq!(bucket.data["meta"]["p"], "q");
        
        // Check sec namespace (sec:user=bob ignores active namespace)
        assert!(bucket.data.contains_key("sec"));
        assert_eq!(bucket.data["sec"]["user"], "bob");
        
        // Verify meta:p=q did NOT go to color namespace
        assert!(!bucket.data["color"].contains_key("p"));
    }
    
    #[test]
    fn test_hierarchical_namespaces() {
        let stream = "anything.it.wants.to.be.token:key1=val1; anything.it.wants.different:key2=val2; anything.else:key3=val3; other.root:key4=val4;";
        let tokens = stream.tokenize().unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Hybrid);
        
        // Test exact namespace access
        assert!(bucket.get_namespace("anything.it.wants.to.be.token").is_some());
        assert_eq!(bucket.get_namespace("anything.it.wants.to.be.token").unwrap()["key1"], "val1");
        
        // Test hierarchical queries
        let under_anything = bucket.get_all_under("anything");
        assert!(under_anything.contains(&"anything.it.wants.to.be.token".to_string()));
        assert!(under_anything.contains(&"anything.it.wants.different".to_string()));
        assert!(under_anything.contains(&"anything.else".to_string()));
        assert!(!under_anything.contains(&"other.root".to_string()));
        
        let under_wants = bucket.get_all_under("anything.it.wants");
        assert!(under_wants.contains(&"anything.it.wants.to.be.token".to_string()));
        assert!(under_wants.contains(&"anything.it.wants.different".to_string()));
        assert!(!under_wants.contains(&"anything.else".to_string()));
    }
    
    #[test] 
    fn test_namespace_tree_navigation() {
        let stream = "a:k1=v1; a.b:k2=v2; a.b.c:k3=v3; a.b.d:k4=v4; a.e:k5=v5;";
        let tokens = stream.tokenize().unwrap();
        let bucket = collect_tokens(&tokens, BucketMode::Tree);
        
        // Test parent-child relationships
        let root_children = bucket.get_children("");
        assert!(root_children.contains(&"a".to_string()));
        
        let a_children = bucket.get_children("a");
        assert!(a_children.contains(&"a.b".to_string()));
        assert!(a_children.contains(&"a.e".to_string()));
        
        let ab_children = bucket.get_children("a.b");
        assert!(ab_children.contains(&"a.b.c".to_string()));
        assert!(ab_children.contains(&"a.b.d".to_string()));
        
        // Test siblings
        let ab_siblings = bucket.get_siblings("a.b");
        assert!(ab_siblings.contains(&"a.e".to_string()));
        assert!(!ab_siblings.contains(&"a.b".to_string())); // Should not include self
        
        let cd_siblings = bucket.get_siblings("a.b.c");
        assert!(cd_siblings.contains(&"a.b.d".to_string()));
    }
    
    #[test]
    fn test_namespace_parsing() {
        let ns = Namespace::from_string("super.duper.namespace");
        assert_eq!(ns.parts, vec!["super", "duper", "namespace"]);
        assert_eq!(ns.to_string(), "super.duper.namespace");
    }
    
    #[test]
    fn test_token_display() {
        let token = Token {
            namespace: Some(Namespace::from_string("sec")),
            key: "user".to_string(),
            value: "alice".to_string(),
        };
        assert_eq!(token.to_string(), "sec:user=alice");
    }
    
    #[test]
    fn test_quoted_value_stripping() {
        let stream = r#"key1="value1"; key2='value2'; key3="value with spaces"; key4=unquoted;"#;
        let tokens = stream.tokenize().unwrap();
        
        println!("Tokens parsed:");
        for token in &tokens {
            println!("  key: '{}', value: '{}'", token.key, token.value);
        }
        
        assert_eq!(tokens.len(), 4);
        // Quotes should now be stripped
        assert_eq!(tokens[0].value, "value1");
        assert_eq!(tokens[1].value, "value2");  
        assert_eq!(tokens[2].value, "value with spaces");
        assert_eq!(tokens[3].value, "unquoted");
    }
    
    #[test]
    fn test_stdin_pipe_format() {
        // Test the exact format from your stdin pipe example
        let stream = r#"key1="value1";"#;
        let tokens = stream.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].key, "key1");
        assert_eq!(tokens[0].value, "value1");
        assert!(tokens[0].namespace.is_none());
    }
    
    #[test]
    fn test_tokenbucket_from_tokens() {
        let stream = "item=val1; ns=animals; dog=fido; sec:user=bob;";
        let tokens = stream.tokenize().unwrap();
        let bucket = TokenBucket::from_tokens(&tokens, BucketMode::Flat);
        
        assert!(bucket.data.contains_key("global"));
        assert_eq!(bucket.data["global"]["item"], "val1");
        
        assert!(bucket.data.contains_key("animals"));
        assert_eq!(bucket.data["animals"]["dog"], "fido");
        
        assert!(bucket.data.contains_key("sec"));
        assert_eq!(bucket.data["sec"]["user"], "bob");
    }
    
    #[test]
    fn test_tokenbucket_from_str() {
        // Direct string to bucket - perfect for stdin pipes!
        let input = r#"host="localhost"; port="8080"; ns=db; user="admin"; pass="secret";"#;
        let bucket = TokenBucket::from_str(input, BucketMode::Hybrid).unwrap();
        
        // Check global namespace
        assert!(bucket.data.contains_key("global"));
        assert_eq!(bucket.data["global"]["host"], "localhost");
        assert_eq!(bucket.data["global"]["port"], "8080");
        
        // Check db namespace
        assert!(bucket.data.contains_key("db"));
        assert_eq!(bucket.data["db"]["user"], "admin");
        assert_eq!(bucket.data["db"]["pass"], "secret");
        
        // Verify quotes were stripped
        assert!(!bucket.data["global"]["host"].contains('"'));
        assert!(!bucket.data["db"]["user"].contains('"'));
    }
    
    #[test]
    fn test_error_handling() {
        // Test empty input
        let result = TokenBucket::from_str("", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::EmptyInput)));
        
        let result = TokenBucket::from_str("   ", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::EmptyInput)));
        
        // Test malformed tokens - missing '='
        let result = TokenBucket::from_str("key_without_equals", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        
        // Test empty key
        let result = TokenBucket::from_str("=value", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        
        // Test input with only separators
        let result = TokenBucket::from_str(";;;", BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
    }
    
    #[test]
    fn test_error_messages() {
        // Test that error messages are informative
        let result = TokenBucket::from_str("bad_token", BucketMode::Flat);
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("missing '=' separator"));
            assert!(msg.contains("bad_token"));
        } else {
            panic!("Expected ParseError");
        }
        
        let result = TokenBucket::from_str("=empty_key", BucketMode::Flat);
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("empty key"));
        } else {
            panic!("Expected ParseError");
        }
    }
    
    #[test]
    fn test_partial_success() {
        // Test that some good tokens work even if others fail
        let result = TokenBucket::from_str("good=value; bad_token; another=good;", BucketMode::Flat);
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("bad_token"));
        } else {
            panic!("Expected ParseError due to malformed token");
        }
        
        // But valid input should work fine
        let result = TokenBucket::from_str("good=value; another=good;", BucketMode::Flat);
        assert!(result.is_ok());
        let bucket = result.unwrap();
        assert_eq!(bucket.data["global"]["good"], "value");
        assert_eq!(bucket.data["global"]["another"], "good");
    }
    
    #[test]
    fn test_spacing_rules() {
        // ✅ ALLOWED: Space after semicolon
        let result = TokenBucket::from_str(r#"k1="val1"; k2="val2";"#, BucketMode::Flat);
        assert!(result.is_ok());
        
        let result = TokenBucket::from_str(r#"k1="val1";   k2="val2";"#, BucketMode::Flat);
        assert!(result.is_ok());
        
        // ❌ NOT ALLOWED: Space before semicolon  
        let result = TokenBucket::from_str(r#"k1="val1" ;k2="val2";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("trailing spaces not allowed"));
        }
        
        // ❌ NOT ALLOWED: Space after equals
        let result = TokenBucket::from_str(r#"k1= "val1";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("space after '=' not allowed"));
        }
        
        // ❌ NOT ALLOWED: Space before equals
        let result = TokenBucket::from_str(r#"k1 ="val1";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("space before '=' not allowed"));
        }
    }
    
    #[test]
    fn test_spacing_edge_cases() {
        // Multiple spaces after semicolon should work
        let result = TokenBucket::from_str(r#"k1="val1";     k2="val2";"#, BucketMode::Flat);
        assert!(result.is_ok());
        let bucket = result.unwrap();
        assert_eq!(bucket.data["global"]["k1"], "val1");
        assert_eq!(bucket.data["global"]["k2"], "val2");
        
        // Leading spaces in the whole string should work
        let result = TokenBucket::from_str(r#"   k1="val1"; k2="val2";"#, BucketMode::Flat);
        assert!(result.is_ok());
        
        // Test with namespaces - same rules apply
        let result = TokenBucket::from_str(r#"ns:key="value"; another:key="value";"#, BucketMode::Flat);
        assert!(result.is_ok());
        
        // Test namespace with space before equals should fail
        let result = TokenBucket::from_str(r#"ns:key ="value";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("space before '=' not allowed"));
        }
    }
    
    #[test]
    fn test_no_spaces_in_keys() {
        // ❌ NOT ALLOWED: Space in namespace part
        let result = TokenBucket::from_str(r#"my namespace:key="value";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("spaces not allowed in namespace"));
            assert!(msg.contains("my namespace"));
        }
        
        // ❌ NOT ALLOWED: Space in key part of prefixed key
        let result = TokenBucket::from_str(r#"ns:my key="value";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("spaces not allowed in key"));
            assert!(msg.contains("my key"));
        }
        
        // ❌ NOT ALLOWED: Space in non-prefixed key
        let result = TokenBucket::from_str(r#"my key="value";"#, BucketMode::Flat);
        assert!(matches!(result, Err(TokenBucketError::ParseError(_))));
        if let Err(TokenBucketError::ParseError(msg)) = result {
            assert!(msg.contains("spaces not allowed in key"));
            assert!(msg.contains("my key"));
        }
        
        // ✅ ALLOWED: Dots in namespace (hierarchical)
        let result = TokenBucket::from_str(r#"my.deep.namespace:key="value";"#, BucketMode::Flat);
        assert!(result.is_ok());
        let bucket = result.unwrap();
        assert!(bucket.data.contains_key("my.deep.namespace"));
        
        // ✅ ALLOWED: Underscores and hyphens
        let result = TokenBucket::from_str(r#"my_namespace:my-key="value";"#, BucketMode::Flat);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_standalone_functions() {
        // Test tokenize_string function directly
        let tokens = tokenize_string(r#"host="localhost"; db:user="admin";"#).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].key, "host");
        assert_eq!(tokens[0].value, "localhost");
        assert!(tokens[0].namespace.is_none());
        assert_eq!(tokens[1].key, "user");
        assert_eq!(tokens[1].value, "admin");
        assert!(tokens[1].namespace.is_some());
        
        // Test tokenize_string with error
        let result = tokenize_string("bad token");
        assert!(matches!(result, Err(_)));
    }
    
    #[test]
    fn test_is_token_streamable() {
        // ✅ Valid inputs should return true
        assert!(is_token_streamable(r#"host="localhost";"#));
        assert!(is_token_streamable(r#"k1="v1"; k2="v2";"#));
        assert!(is_token_streamable(r#"ns:key="value";"#));
        assert!(is_token_streamable(r#"ns=config; host="localhost";"#));
        assert!(is_token_streamable(""));  // empty is valid
        assert!(is_token_streamable(";;;"));  // only separators is valid
        
        // ❌ Invalid inputs should return false
        assert!(!is_token_streamable("bad_token"));  // missing =
        assert!(!is_token_streamable("=empty_key"));  // empty key
        assert!(!is_token_streamable(r#"k1= "val1";"#));  // space after =
        assert!(!is_token_streamable(r#"k1 ="val1";"#));  // space before =
        assert!(!is_token_streamable(r#"k1="val1" ;k2="val2";"#));  // space before ;
        assert!(!is_token_streamable(r#"my namespace:key="value";"#));  // space in namespace
        assert!(!is_token_streamable(r#"ns:my key="value";"#));  // space in key
        assert!(!is_token_streamable(r#"my key="value";"#));  // space in plain key
    }
    
    #[test]
    fn test_validation_consistency() {
        let test_cases = vec![
            r#"host="localhost";"#,
            r#"k1="v1"; k2="v2";"#,
            "bad_token",
            r#"k1= "val1";"#,
            r#"my namespace:key="value";"#,
        ];
        
        for case in test_cases {
            // Both validation methods should agree
            let streamable_result = is_token_streamable(case);
            let trait_result = case.validate().is_ok();
            let tokenize_result = tokenize_string(case).is_ok();
            
            assert_eq!(streamable_result, trait_result, "Mismatch for: {}", case);
            assert_eq!(streamable_result, tokenize_result, "Mismatch for: {}", case);
        }
    }
    
    #[test]
    fn test_generator_integration() {
        use crate::xstream::gen::{gen_token, gen_flat_token, gen_ns_token, gen_config_stream, ValueType};
        
        // Test individual token generation
        let token = gen_token(Some("test"), Some("key"), ValueType::Literal("value".to_string()));
        assert!(is_token_streamable(&token));
        let parsed = tokenize_string(&token).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].key, "key");
        assert_eq!(parsed[0].value, "value");
        
        // Test flat token generation
        let flat_token = gen_flat_token(Some("host"), ValueType::Literal("localhost".to_string()));
        assert!(is_token_streamable(&flat_token));
        
        // Test namespace token generation  
        let ns_token = gen_ns_token(Some("config"));
        assert!(is_token_streamable(&ns_token));
        assert_eq!(ns_token, "ns=config");
        
        // Test config stream generation
        let config = gen_config_stream();
        assert!(is_token_streamable(&config));
        let bucket = TokenBucket::from_str(&config, BucketMode::Hybrid).unwrap();
        
        // Should have global, db, and auth namespaces
        assert!(bucket.data.contains_key("global"));
        assert!(bucket.data.contains_key("db"));
        assert!(bucket.data.contains_key("auth"));
        
        // Global should have host, port, debug
        assert!(bucket.data["global"].contains_key("host"));
        assert!(bucket.data["global"].contains_key("port"));
        assert!(bucket.data["global"].contains_key("debug"));
    }
}