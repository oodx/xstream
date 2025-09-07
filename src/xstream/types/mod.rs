pub mod namespace;
pub mod token;
pub mod bucket;

// Re-export public types
pub use namespace::Namespace;
pub use token::{Token, TokenStreamable};
pub use bucket::{TokenBucket, BucketMode, collect_tokens};

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
}