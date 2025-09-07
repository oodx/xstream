use std::collections::HashMap;
use super::{namespace::Namespace, token::{Token, TokenStreamable}, error::{TokenBucketError, TokenBucketResult}};

#[derive(Debug, Clone)]
pub enum BucketMode {
    Flat,    // Just HashMap<String, HashMap<String, String>>
    Tree,    // Nested structure only
    Hybrid,  // Both flat and tree index
}

#[derive(Debug)]
pub struct TokenBucket {
    pub mode: BucketMode,
    pub data: HashMap<String, HashMap<String, String>>,
    pub tree: Option<HashMap<String, Vec<String>>>,  // Only populated in Tree/Hybrid mode
}

impl TokenBucket {
    pub fn new(mode: BucketMode) -> Self {
        TokenBucket {
            mode: mode.clone(),
            data: HashMap::new(),
            tree: match mode {
                BucketMode::Flat => None,
                _ => Some(HashMap::new()),
            }
        }
    }
    
    pub fn from_tokens(tokens: &[Token], mode: BucketMode) -> Self {
        collect_tokens(tokens, mode)
    }
    
    pub fn from_str(input: &str, mode: BucketMode) -> TokenBucketResult<Self> {
        if input.trim().is_empty() {
            return Err(TokenBucketError::EmptyInput);
        }
        
        let tokens = input.tokenize()
            .map_err(|e| TokenBucketError::ParseError(e))?;
            
        if tokens.is_empty() {
            return Err(TokenBucketError::ParseError("No valid tokens found in input".to_string()));
        }
        
        Ok(Self::from_tokens(&tokens, mode))
    }
    
    pub fn insert(&mut self, namespace: &Namespace, key: String, value: String) {
        let ns_key = namespace.to_string();
        
        // Always store flat data
        self.data.entry(ns_key.clone())
            .or_insert_with(HashMap::new)
            .insert(key, value);
        
        // Build tree if needed
        match self.mode {
            BucketMode::Flat => {},
            BucketMode::Tree | BucketMode::Hybrid => {
                self.build_tree_index(namespace);
            }
        }
    }
    
    fn build_tree_index(&mut self, namespace: &Namespace) {
        if let Some(ref mut tree) = self.tree {
            let _full_path = namespace.to_string();
            
            // Build parent-child relationships for all levels
            let mut current_path = String::new();
            for (i, part) in namespace.parts.iter().enumerate() {
                if i > 0 {
                    let parent_path = current_path.clone();
                    current_path.push(namespace.delimiter);
                    current_path.push_str(part);
                    
                    // Add current path to parent's children
                    tree.entry(parent_path)
                        .or_insert_with(Vec::new)
                        .push(current_path.clone());
                } else {
                    current_path = part.clone();
                    // Root level entries
                    tree.entry("".to_string())
                        .or_insert_with(Vec::new)
                        .push(current_path.clone());
                }
            }
        }
    }
    
    // Get direct children of a namespace
    pub fn get_children(&self, namespace: &str) -> Vec<String> {
        if let Some(ref tree) = self.tree {
            tree.get(namespace).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }
    
    // Get all namespaces under a given prefix (descendants)
    pub fn get_all_under(&self, prefix: &str) -> Vec<String> {
        self.data.keys()
            .filter(|ns| ns.starts_with(prefix) && ns.as_str() != prefix)
            .cloned()
            .collect()
    }
    
    // Get namespace data by exact match
    pub fn get_namespace(&self, namespace: &str) -> Option<&HashMap<String, String>> {
        self.data.get(namespace)
    }
    
    // Get siblings (namespaces at same level)
    pub fn get_siblings(&self, namespace: &str) -> Vec<String> {
        if let Some(parent) = self.get_parent(namespace) {
            self.get_children(&parent)
                .into_iter()
                .filter(|ns| ns != namespace)
                .collect()
        } else {
            // Root level siblings
            self.get_children("")
                .into_iter()
                .filter(|ns| ns != namespace)
                .collect()
        }
    }
    
    fn get_parent(&self, namespace: &str) -> Option<String> {
        if let Some(last_dot) = namespace.rfind('.') {
            Some(namespace[..last_dot].to_string())
        } else {
            None
        }
    }
}

// Standalone function to collect tokens into a bucket
pub fn collect_tokens(tokens: &[Token], mode: BucketMode) -> TokenBucket {
    let mut bucket = TokenBucket::new(mode);
    let mut active_namespace = Namespace::from_string("global");
    
    for token in tokens {
        // Handle ns= tokens for namespace switching
        if token.namespace.is_none() && token.key == "ns" {
            active_namespace = Namespace::from_string(&token.value);
            continue;
        }
        
        // Use token's namespace if present, otherwise use active namespace
        let namespace = token.namespace.as_ref().unwrap_or(&active_namespace);
        
        bucket.insert(namespace, token.key.clone(), token.value.clone());
    }
    
    bucket
}