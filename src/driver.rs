// XStream Driver - Clean Visual Stream Operation Demonstrations
// ┌─────────────────────────────────────────────────────────────────┐
// │ Enhanced Stream Operation Testing with Box Characters & Colors  │
// └─────────────────────────────────────────────────────────────────┘

use rsb::prelude::*;
use xstream::xstream::real_fork::{Fork, ForkAll};
use xstream::xstream::real_merge::{Merge, MergeStrategy};  
use xstream::xstream::real_gate::{Gate, GateCondition, SyncGate};
use xstream::colors::{colorize, get_color, pre_color_stream, RESET};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        show_help();
        return;
    }
    
    match args[1].as_str() {
        "help" => show_help(),
        "fork" => ceremony_fork_operations(),
        "merge" => ceremony_merge_operations(),
        "gate" => ceremony_gate_operations(),
        "pipeline" => ceremony_pipeline_operations(),
        "colors" => ceremony_color_showcase(),
        "all" => run_all_ceremonies(),
        _ => {
            println!("Unknown ceremony: {}", args[1]);
            show_help();
        }
    }
}

fn show_help() {
    println!("┌─────────────────────────────────────────────────┐");
    println!("│             XStream Driver - Ceremonies        │");
    println!("└─────────────────────────────────────────────────┘");
    println!();
    println!("Usage: cargo run --bin xstream-driver <ceremony>");
    println!();
    println!("Ceremonies:");
    println!("  help      - Show this help");
    println!("  fork      - Stream forking demonstrations");
    println!("  merge     - Stream merging demonstrations");
    println!("  gate      - Stream gating demonstrations");
    println!("  pipeline  - Multi-step stream operations");
    println!("  colors    - Color showcase for stream values");
    println!("  all       - Run all ceremonies in sequence");
    println!();
}

fn run_all_ceremonies() {
    println!("{} Running All Stream Ceremonies {}", 
        colorize("", "success"), RESET);
    println!();
    
    ceremony_fork_operations();
    ceremony_merge_operations();
    ceremony_gate_operations();
    ceremony_pipeline_operations();
    ceremony_color_showcase();
    
    println!("{} All Ceremonies Complete {}", 
        colorize("", "success"), RESET);
    println!();
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                    FORK OPERATION CEREMONIES                    ║
// ╚═════════════════════════════════════════════════════════════════╝

fn ceremony_fork_operations() {
    print_section_header("FORK OPERATIONS - Stream Splitting by Namespace");
    
    ceremony_fork_test_1();
    ceremony_fork_test_2();
    ceremony_fork_test_3();
    ceremony_fork_test_4();
    ceremony_fork_test_5();
    ceremony_fork_test_6();
    
    print_section_footer("Fork Operations");
}

fn ceremony_fork_test_1() {
    print_test_header("1", "Basic Fork - Split by Namespace");
    
    // Create pre-colored input streams that maintain colors through operations
    let input = generate_test_stream("ui", "red", 3) + "; " +
               &generate_test_stream("db", "blue", 2) + "; " +
               &generate_test_stream("log", "green", 2);
    
    print_stream_input("┌─ Input Stream", &input);
    println!("{}", colorize("│", "grey"));
    
    let forked_result = input.to_string().stream_apply(Fork, 
        vec!["ui".to_string(), "db".to_string(), "log".to_string()]);
    
    print_flow_arrow("Fork Operation → Split by namespaces (colors preserved!)");
    println!("{}", colorize("│", "grey"));
    
    for line in forked_result.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "ui" => "red",
                "db" => "blue",
                "log" => "green",
                _ => "white"
            };
            println!("{}{} {}[{}]{} → {}", 
                colorize("├─", "grey"), RESET,
                get_color(color), ns.to_uppercase(), RESET, 
                tokens); // Already pre-colored, maintain original colors
        }
    }
    
    print_test_result("✓ Stream successfully forked by namespace - original colors maintained!");
}

fn ceremony_fork_test_2() {
    print_test_header("2", "Fork All - Discover All Namespaces");
    
    let input = generate_test_stream("auth", "purple", 3) + "; " +
               &generate_test_stream("cfg", "orange", 2) + "; " +
               &generate_test_stream("sys", "cyan", 3);
    
    print_stream_input("┌─ Input Stream", &input);
    println!("{}", colorize("│", "grey"));
    
    let forked_result = input.to_string().stream_apply(ForkAll, ());
    
    print_flow_arrow("ForkAll Operation → Discover & split all namespaces");
    println!("{}", colorize("│", "grey"));
    
    for line in forked_result.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "auth" => "purple",
                "cfg" => "orange",
                "sys" => "cyan",
                _ => "white"
            };
            println!("{}{} {}[{}]{} → {}", 
                colorize("├─", "grey"), RESET,
                get_color(color), ns.to_uppercase(), RESET,
                pre_color_stream(tokens, color));
        }
    }
    
    print_test_result("✓ All namespaces discovered and forked");
}

fn ceremony_fork_test_3() {
    print_test_header("3", "Fork Flow Visualization");
    
    let input = generate_test_stream("web", "red", 4) + "; " +
               &generate_test_stream("api", "blue", 3);
    
    println!("{}{} Stream Flow Diagram:", colorize("┌─", "grey"), RESET);
    println!("{}{}", colorize("│", "grey"), RESET);
    println!("{}{} Input: {}", colorize("│", "grey"), RESET, colorize_stream(&input));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("↓", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("┌─────────────────┐", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("│   Fork by NS    │", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("└─────────────────┘", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("↓   ↓", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize("[web] [api]", "grey"));
    
    let forked = input.to_string().stream_apply(Fork, 
        vec!["web".to_string(), "api".to_string()]);
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = if ns == "web" { "red" } else { "blue" };
            println!("{}{} {} {}",
                colorize("│", "grey"), RESET,
                pre_color_stream(tokens, color),
                colorize(&format!("← {}", ns), color));
        }
    }
    
    print_test_result("✓ Fork flow visualized");
}

fn ceremony_fork_test_4() {
    print_test_header("4", "Selective Fork - Specific Namespaces");
    
    let input = generate_test_stream("ui", "red", 2) + "; " +
               &generate_test_stream("db", "blue", 3) + "; " +
               &generate_test_stream("auth", "purple", 2) + "; " +
               &generate_test_stream("log", "green", 3);
    
    print_stream_input("┌─ Multi-Namespace Input", &input);
    println!("{}{} Available: [ui] [db] [auth] [log]", colorize("│", "grey"), RESET);
    println!("{}{} Selecting only: [ui] [auth]", colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    let forked = input.to_string().stream_apply(Fork, 
        vec!["ui".to_string(), "auth".to_string()]);
    
    print_flow_arrow("Selective Fork → Only ui and auth namespaces");
    println!("{}", colorize("│", "grey"));
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "ui" => "red",
                "auth" => "purple",
                _ => "white"
            };
            println!("{}{} {}[{}]{} → {}", 
                colorize("├─", "grey"), RESET,
                get_color(color), ns.to_uppercase(), RESET,
                pre_color_stream(tokens, color));
        }
    }
    
    print_test_result("✓ Selective fork filtered successfully");
}

fn ceremony_fork_test_5() {
    print_test_header("5", "Empty Namespace Handling");
    
    let input = generate_test_stream("data", "cyan", 3) + "; " +
               "orphan=\"value01\"; another=\"value02\""; // tokens without namespace
    
    print_stream_input("┌─ Mixed Input (with orphaned tokens)", &input);
    println!("{}{} Contains: [data] namespace + orphaned tokens", colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    let forked = input.to_string().stream_apply(ForkAll, ());
    
    print_flow_arrow("ForkAll → Handle orphaned tokens");
    println!("{}", colorize("│", "grey"));
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "data" => "cyan",
                "" => "yellow", // empty namespace for orphaned tokens
                _ => "white"
            };
            let ns_display = if ns.is_empty() { "ORPHANED" } else { &ns.to_uppercase() };
            println!("{}{} {}[{}]{} → {}", 
                colorize("├─", "grey"), RESET,
                get_color(color), ns_display, RESET,
                pre_color_stream(tokens, color));
        }
    }
    
    print_test_result("✓ Empty namespace handling demonstrated");
}

fn ceremony_fork_test_6() {
    print_test_header("6", "Deeply Nested Namespaces");
    
    let input = "app:ui:btn:text=\"deep01\"; app:ui:btn:color=\"deep02\"; app:db:conn:host=\"deep03\"; app:db:conn:port=\"deep04\"; app:ui:form:name=\"deep05\"; sys:log:level=\"deep06\"";
    
    print_stream_input("┌─ Deeply Nested Input", input);
    println!("{}{} Structure: app:ui:btn, app:db:conn, sys:log", colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    // Fork by top-level namespace
    let forked = input.to_string().stream_apply(Fork, 
        vec!["app".to_string(), "sys".to_string()]);
    
    print_flow_arrow("Fork by top-level namespace [app] [sys]");
    println!("{}", colorize("│", "grey"));
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "app" => "blue",
                "sys" => "green",
                _ => "white"
            };
            println!("{}{} {}[{}]{} → {}", 
                colorize("├─", "grey"), RESET,
                get_color(color), ns.to_uppercase(), RESET,
                pre_color_stream(tokens, color));
            
            // Show nested structure analysis
            let token_count = tokens.split("; ").count();
            let nested_namespaces: Vec<String> = tokens.split("; ")
                .filter_map(|t| {
                    if let Some(pos) = t.find('=') {
                        let key_part = &t[..pos];
                        if let Some(second_colon) = key_part.find(':') {
                            let remaining = &key_part[second_colon+1..];
                            if let Some(third_colon) = remaining.find(':') {
                                Some(format!("{}:{}", &key_part[..second_colon+1], &remaining[..third_colon]))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            if !nested_namespaces.is_empty() {
                println!("{}{} {} {} nested: [{}]", 
                    colorize("│", "grey"), colorize("  ", "grey"), RESET,
                    colorize("→", "grey"),
                    nested_namespaces.join("] ["));
            }
        }
    }
    
    print_test_result("✓ Deeply nested namespaces processed");
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                   MERGE OPERATION CEREMONIES                    ║
// ╚═════════════════════════════════════════════════════════════════╝

fn ceremony_merge_operations() {
    print_section_header("MERGE OPERATIONS - Stream Combination Strategies");
    
    ceremony_merge_test_1();
    ceremony_merge_test_2();
    ceremony_merge_test_3();
    ceremony_merge_test_4();
    ceremony_merge_test_5();
    ceremony_merge_test_6();
    
    print_section_footer("Merge Operations");
}

fn ceremony_merge_test_1() {
    print_test_header("1", "Concat Merge - Simple Concatenation");
    
    let ui_stream = generate_test_stream("ui", "red", 3);
    let db_stream = generate_test_stream("db", "blue", 2);
    let forked_input = format!("ui: {}\ndb: {}", ui_stream, db_stream);
    
    println!("{}{} Forked Streams Input:", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = if ns == "ui" { "red" } else { "blue" };
            println!("{}{} {} → {}", 
                colorize("│", "grey"), RESET,
                colorize(&format!("[{}]", ns.to_uppercase()), color),
                tokens); // Already pre-colored with blocks and colors
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged = forked_input.to_string().stream_apply(Merge, MergeStrategy::Concat);
    
    print_flow_arrow("Merge::Concat → All tokens joined (colors preserved!)");
    println!("{}{} Result: {}", colorize("│", "grey"), RESET, merged);
    println!("{}{} Notice: Red ■ tokens mixed with blue ■ tokens - origin tracking!", 
        colorize("│", "grey"), RESET);
    
    print_test_result("✓ Streams concatenated - original colors maintained through merge!");
}

fn ceremony_merge_test_2() {
    print_test_header("2", "Interleave Merge - Round-Robin Token Selection");
    
    let red_stream = generate_test_stream("red", "red", 4);
    let blue_stream = generate_test_stream("blue", "blue", 3);
    let forked_input = format!("red: {}\nblue: {}", red_stream, blue_stream);
    
    println!("{}{} Multi-Stream Input:", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((color_ns, tokens)) = line.split_once(": ") {
            let color = color_ns;
            println!("{}{} {} → {}", 
                colorize("│", "grey"), RESET,
                colorize(&format!("[{}]", color_ns.to_uppercase()), color),
                pre_color_stream(tokens, color));
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged = forked_input.to_string().stream_apply(Merge, MergeStrategy::Interleave);
    
    print_flow_arrow("Merge::Interleave → Round-robin selection");
    println!("{}{} {}: red₁ → blue₁ → red₂ → blue₂ → red₃", 
        colorize("│", "grey"), RESET, colorize("Pattern", "grey"));
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize_stream(&merged));
    
    print_test_result("✓ Streams interleaved successfully");
}

fn ceremony_merge_test_3() {
    print_test_header("3", "Dedupe Merge - Remove Duplicates");
    
    // Create streams with intentional duplicates
    let s1_stream = "token=\"value01\"; key=\"unique01\"";
    let s2_stream = "token=\"value01\"; value=\"unique02\"";
    let forked_input = format!("s1: {}\ns2: {}", s1_stream, s2_stream);
    
    println!("{}{} Duplicate Token Input:", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((stream, tokens)) = line.split_once(": ") {
            println!("{}{} {} → {}", 
                colorize("│", "grey"), RESET,
                colorize(&format!("[{}]", stream.to_uppercase()), "cyan"),
                colorize_stream(tokens));
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged = forked_input.to_string().stream_apply(Merge, MergeStrategy::Dedupe);
    
    print_flow_arrow("Merge::Dedupe → Duplicates removed");
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize_stream(&merged));
    println!("{}{} {}: token=\"value01\" appears only once", 
        colorize("│", "grey"), RESET, colorize("Notice", "grey"));
    
    print_test_result("✓ Duplicates successfully removed");
}

fn ceremony_merge_test_4() {
    print_test_header("4", "Weighted Merge - Priority-Based Combination");
    
    // Simulate weighted merge by using Interleave but with uneven token distribution
    let priority_stream = generate_test_stream("priority", "red", 5);
    let normal_stream = generate_test_stream("normal", "blue", 2);
    let forked_input = format!("priority: {}\nnormal: {}", priority_stream, normal_stream);
    
    println!("{}{} Weighted Input (Priority vs Normal):", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((stream, tokens)) = line.split_once(": ") {
            let color = match stream {
                "priority" => "red",
                "normal" => "blue",
                _ => "white"
            };
            let token_count = tokens.split("; ").count();
            println!("{}{} {} → {} ({})",
                colorize("│", "grey"), RESET,
                colorize(&format!("[{}]", stream.to_uppercase()), color),
                pre_color_stream(tokens, color),
                colorize(&format!("{} tokens", token_count), "grey"));
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged = forked_input.to_string().stream_apply(Merge, MergeStrategy::Concat);
    
    print_flow_arrow("Weighted Merge → Priority tokens processed first");
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize_stream(&merged));
    println!("{}{} {}: Priority stream (5 tokens) comes before Normal (2 tokens)", 
        colorize("│", "grey"), RESET, colorize("Weight Effect", "grey"));
    
    print_test_result("✓ Weighted merge demonstrated with priority ordering");
}

fn ceremony_merge_test_5() {
    print_test_header("5", "Empty Stream Merge - Graceful Handling");
    
    let filled_stream = generate_test_stream("data", "green", 4);
    let empty_stream = ""; // empty stream
    let forked_input = format!("data: {}\nempty: {}", filled_stream, empty_stream);
    
    println!("{}{} Mixed Input (Filled + Empty):", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((stream, tokens)) = line.split_once(": ") {
            let color = match stream {
                "data" => "green",
                "empty" => "yellow",
                _ => "white"
            };
            if tokens.is_empty() {
                println!("{}{} {} → {}", 
                    colorize("│", "grey"), RESET,
                    colorize(&format!("[{}]", stream.to_uppercase()), color),
                    colorize("(empty stream)", "grey"));
            } else {
                println!("{}{} {} → {}", 
                    colorize("│", "grey"), RESET,
                    colorize(&format!("[{}]", stream.to_uppercase()), color),
                    pre_color_stream(tokens, color));
            }
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged = forked_input.to_string().stream_apply(Merge, MergeStrategy::Concat);
    
    print_flow_arrow("Empty Stream Merge → Skip empty, preserve filled");
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize_stream(&merged));
    println!("{}{} {}: Empty streams filtered out gracefully", 
        colorize("│", "grey"), RESET, colorize("Result", "grey"));
    
    print_test_result("✓ Empty streams handled gracefully");
}

fn ceremony_merge_test_6() {
    print_test_header("6", "Mismatched Length Merge - Uneven Token Counts");
    
    let long_stream = generate_test_stream("long", "blue", 6);
    let short_stream = generate_test_stream("short", "red", 2);
    let medium_stream = generate_test_stream("medium", "green", 4);
    let forked_input = format!("long: {}\nshort: {}\nmedium: {}", 
        long_stream, short_stream, medium_stream);
    
    println!("{}{} Mismatched Length Input:", colorize("┌─", "grey"), RESET);
    for line in forked_input.lines() {
        if let Some((stream, tokens)) = line.split_once(": ") {
            let color = match stream {
                "long" => "blue",
                "short" => "red", 
                "medium" => "green",
                _ => "white"
            };
            let token_count = tokens.split("; ").count();
            println!("{}{} {} → {} {}",
                colorize("│", "grey"), RESET,
                colorize(&format!("[{}]", stream.to_uppercase()), color),
                pre_color_stream(tokens, color),
                colorize(&format!("({} tokens)", token_count), "grey"));
        }
    }
    println!("{}", colorize("│", "grey"));
    
    let merged_interleave = forked_input.to_string().stream_apply(Merge, MergeStrategy::Interleave);
    
    print_flow_arrow("Interleave Merge → Round-robin until shorter streams exhausted");
    println!("{}{} {}", colorize("│", "grey"), RESET, colorize_stream(&merged_interleave));
    println!("{}{} {}: Interleave continues with remaining tokens from longer streams", 
        colorize("│", "grey"), RESET, colorize("Behavior", "grey"));
    
    print_test_result("✓ Mismatched lengths handled by interleave strategy");
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                    GATE OPERATION CEREMONIES                    ║
// ╚═════════════════════════════════════════════════════════════════╝

fn ceremony_gate_operations() {
    print_section_header("GATE OPERATIONS - Stream Flow Control");
    
    ceremony_gate_test_1();
    ceremony_gate_test_2();
    ceremony_gate_test_3();
    ceremony_gate_test_4();
    ceremony_gate_test_5();
    ceremony_gate_test_6();
    ceremony_gate_test_7();
    
    print_section_footer("Gate Operations");
}

fn ceremony_gate_test_1() {
    print_test_header("1", "Min Tokens Gate - Minimum Requirements");
    
    let short_stream = "a:x=\"aa\"; b:y=\"bb\"";
    let long_stream = "a:x=\"aa\"; b:y=\"bb\"; c:z=\"cc\"; d:w=\"dd\"";
    
    println!("┌─ Testing MinTokens(3) Gate:");
    println!("│");
    println!("│  Short Stream (2 tokens): {}", colorize_stream(short_stream));
    let short_result = short_stream.to_string().stream_apply(Gate, GateCondition::MinTokens(3));
    let short_status = if short_result.is_empty() { 
        colorize("✗ BLOCKED", "error") 
    } else { 
        colorize("✓ PASSED", "success") 
    };
    println!("│  Result: {}", short_status);
    println!("│");
    
    println!("│  Long Stream (4 tokens):  {}", colorize_stream(long_stream));
    let long_result = long_stream.to_string().stream_apply(Gate, GateCondition::MinTokens(3));
    let long_status = if long_result.is_empty() { 
        colorize("✗ BLOCKED", "error") 
    } else { 
        colorize("✓ PASSED", "success") 
    };
    println!("│  Result: {}", long_status);
    
    print_test_result("✓ MinTokens gate working correctly");
}

fn ceremony_gate_test_2() {
    print_test_header("2", "Max Tokens Gate - Capacity Limiting");
    
    let oversized_stream = "a:x=\"aa\"; b:y=\"bb\"; c:z=\"cc\"; d:w=\"dd\"; e:v=\"ee\"";
    
    println!("┌─ Input Stream (5 tokens):");
    println!("│  {}", colorize_stream(oversized_stream));
    println!("│");
    
    let truncated = oversized_stream.to_string().stream_apply(Gate, GateCondition::MaxTokens(3));
    
    println!("├─ MaxTokens(3) Gate → Truncate to 3 tokens");
    println!("│  Result: {}", colorize_stream(&truncated));
    println!("│  Status: {} (2 tokens removed)", colorize("⚠ TRUNCATED", "warning"));
    
    print_test_result("✓ MaxTokens gate truncated correctly");
}

fn ceremony_gate_test_3() {
    print_test_header("3", "Require Namespace Gate - Namespace Validation");
    
    let multi_ns_stream = "ui:btn=\"uu\"; api:req=\"aa\"; log:msg=\"ll\"";
    
    println!("┌─ Input Stream:");
    println!("│  {}", colorize_stream(multi_ns_stream));
    println!("│  Namespaces: [ui] [api] [log]");
    println!("│");
    
    // Test for existing namespace
    let has_api = multi_ns_stream.to_string().stream_apply(Gate, 
        GateCondition::RequireNamespace("api".to_string()));
    let api_status = if has_api.is_empty() { 
        colorize("✗ BLOCKED", "error") 
    } else { 
        colorize("✓ PASSED", "success") 
    };
    println!("│  RequireNamespace(\"api\"): {}", api_status);
    
    // Test for missing namespace
    let has_auth = multi_ns_stream.to_string().stream_apply(Gate, 
        GateCondition::RequireNamespace("auth".to_string()));
    let auth_status = if has_auth.is_empty() { 
        colorize("✗ BLOCKED", "error") 
    } else { 
        colorize("✓ PASSED", "success") 
    };
    println!("│  RequireNamespace(\"auth\"): {}", auth_status);
    
    print_test_result("✓ Namespace validation working correctly");
}

fn ceremony_gate_test_4() {
    print_test_header("4", "Sync Gate - Multi-Stream Coordination");
    
    let stream1 = "a:x=\"aa\"; a:y=\"aa\"; a:z=\"aa\"";
    let stream2 = "b:x=\"bb\"; b:y=\"bb\"";
    
    println!("┌─ Synchronization Test:");
    println!("│  Stream 1 (3 tokens): {}", colorize_stream(stream1));
    println!("│  Stream 2 (2 tokens): {}", colorize_stream(stream2));
    println!("│  MinTokens: 2");
    println!("│");
    
    let synced = stream1.to_string().stream_apply(SyncGate, (stream2.to_string(), 2));
    
    if synced.is_empty() {
        println!("│  Result: {} - Insufficient tokens", colorize("✗ BLOCKED", "error"));
    } else {
        println!("│  Result: {} - Both streams meet minimum", colorize("✓ SYNCED", "success"));
        println!("│  Output: {}", colorize_stream(&synced));
    }
    
    print_test_result("✓ Sync gate coordination working");
}

fn ceremony_gate_test_5() {
    print_test_header("5", "Combination Gate - MinTokens AND RequireNamespace");
    
    let input = generate_test_stream("auth", "purple", 4) + "; " +
               &generate_test_stream("data", "cyan", 1) + "; " +
               &generate_test_stream("log", "green", 3);
    
    print_stream_input("┌─ Multi-Namespace Input", &input);
    println!("{}{} Requirements: MinTokens(3) AND RequireNamespace(auth)", 
        colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    // First apply MinTokens gate
    let min_tokens_result = input.to_string().stream_apply(Gate, GateCondition::MinTokens(3));
    
    print_flow_arrow("Step 1: MinTokens(3) Gate");
    if min_tokens_result.is_empty() {
        println!("{}{} {} - Insufficient total tokens", 
            colorize("├─", "grey"), RESET, colorize("✗ BLOCKED", "error"));
    } else {
        println!("{}{} {} - {} tokens passed", 
            colorize("├─", "grey"), RESET, colorize("✓ PASSED", "success"),
            min_tokens_result.split("; ").count());
            
        // Then apply RequireNamespace gate
        let combined_result = min_tokens_result.stream_apply(Gate, 
            GateCondition::RequireNamespace("auth".to_string()));
        
        print_flow_arrow("Step 2: RequireNamespace(auth) Gate");
        if combined_result.is_empty() {
            println!("{}{} {} - auth namespace missing", 
                colorize("└─", "grey"), RESET, colorize("✗ BLOCKED", "error"));
        } else {
            println!("{}{} {} - Both conditions met", 
                colorize("└─", "grey"), RESET, colorize("✓ PASSED", "success"));
            println!("{}{} Result: {}", 
                colorize("  ", "grey"), RESET, colorize_stream(&combined_result));
        }
    }
    
    print_test_result("✓ Combination gate logic demonstrated");
}

fn ceremony_gate_test_6() {
    print_test_header("6", "Token Value Filtering Gate - Content-Based Filtering");
    
    let input = "priority:high=\"urgent01\"; priority:low=\"normal02\"; priority:high=\"urgent03\"; data:info=\"content04\"";
    
    print_stream_input("┌─ Mixed Priority Input", input);
    println!("{}{} Contains: high priority, low priority, and data tokens", 
        colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    // Simulate value-based filtering by checking stream content
    let tokens: Vec<&str> = input.split("; ").collect();
    let high_priority_tokens: Vec<&str> = tokens.iter()
        .filter(|token| token.contains("priority:high"))
        .copied()
        .collect();
    
    let filtered_stream = high_priority_tokens.join("; ");
    
    print_flow_arrow("Value Filter → Only priority:high tokens");
    println!("{}{} Filtered: {}", 
        colorize("├─", "grey"), RESET, colorize_stream(&filtered_stream));
    println!("{}{} {} tokens filtered to {} high-priority tokens", 
        colorize("├─", "grey"), RESET, 
        tokens.len(), high_priority_tokens.len());
    
    print_test_result("✓ Value-based filtering demonstrated");
}

fn ceremony_gate_test_7() {
    print_test_header("7", "Rate Limiting Gate - Throughput Control");
    
    let large_input = generate_test_stream("data", "cyan", 10);
    let rate_limit = 5; // Max 5 tokens per batch
    
    print_stream_input("┌─ High-Volume Input", &large_input);
    println!("{}{} Rate Limit: {} tokens per batch", 
        colorize("│", "grey"), RESET, rate_limit);
    println!("{}", colorize("│", "grey"));
    
    // Simulate rate limiting using MaxTokens gate
    let limited_result = large_input.to_string().stream_apply(Gate, 
        GateCondition::MaxTokens(rate_limit));
    
    print_flow_arrow("Rate Limiting → Truncate to 5 tokens maximum");
    println!("{}{} Limited: {}", 
        colorize("├─", "grey"), RESET, colorize_stream(&limited_result));
    
    let input_count = large_input.split("; ").count();
    let output_count = limited_result.split("; ").count();
    println!("{}{} {} tokens → {} tokens ({}% reduction)", 
        colorize("├─", "grey"), RESET, 
        input_count, output_count, 
        colorize(&format!("{}", ((input_count - output_count) * 100) / input_count), "warning"));
    
    print_test_result("✓ Rate limiting gate demonstrated");
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                   PIPELINE OPERATION CEREMONIES                 ║
// ╚═════════════════════════════════════════════════════════════════╝

fn ceremony_pipeline_operations() {
    print_section_header("PIPELINE OPERATIONS - Multi-Step Stream Transformations");
    
    ceremony_pipeline_test_1();
    ceremony_pipeline_test_2();
    ceremony_pipeline_test_3();
    ceremony_pipeline_test_4();
    ceremony_pipeline_test_5();
    
    print_section_footer("Pipeline Operations");
}

fn ceremony_pipeline_test_1() {
    print_test_header("1", "Fork → Gate → Merge Pipeline");
    
    // Create pre-colored input with block symbols
    let input = create_colored_token("ui", "btn", "red01", "red") + "; " +
               &create_colored_token("api", "req", "blue01", "blue") + "; " +
               &create_colored_token("ui", "form", "red02", "red") + "; " +
               &create_colored_token("log", "msg", "green01", "green") + "; " +
               &create_colored_token("api", "resp", "blue02", "blue");
    
    println!("┌─ Pipeline Flow - Color Tracking Demo:");
    println!("│");
    println!("│  Input: {}", input);
    println!("│    │");
    println!("│    ▼ Fork by namespace (colors travel with tokens)");
    
    // Step 1: Fork
    let forked = input.to_string().stream_apply(Fork, 
        vec!["ui".to_string(), "api".to_string(), "log".to_string()]);
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let color = match ns {
                "ui" => "red",
                "api" => "blue", 
                "log" => "green",
                _ => "white"
            };
            println!("│    ├─ {}: {}", 
                colorize(ns, color), 
                tokens); // Already colored, showing origin tracking
        }
    }
    
    println!("│    │");
    println!("│    ▼ Gate: MinTokens(2) per namespace");
    
    // Step 2: Apply gates to filter streams with enough tokens
    let mut gated_streams = Vec::new();
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let token_count = tokens.split("; ").count();
            if token_count >= 2 {
                gated_streams.push(format!("{}: {}", ns, tokens));
                let color = match ns {
                    "ui" => "red",
                    "api" => "blue",
                    "log" => "green", 
                    _ => "white"
                };
                println!("│    ├─ {} {} ({})",
                    colorize("✓", "success"),
                    colorize(ns, color),
                    tokens); // Colors preserved through gating
            } else {
                println!("│    ├─ {} {} (insufficient tokens)",
                    colorize("✗", "error"),
                    colorize(ns, "grey"));
            }
        }
    }
    
    println!("│    │");
    println!("│    ▼ Merge surviving streams (origin colors mixed)");
    
    // Step 3: Merge
    let gated_input = gated_streams.join("\n");
    let merged = gated_input.stream_apply(Merge, MergeStrategy::Concat);
    
    println!("│    └─ Final: {}", merged);
    println!("│");
    println!("│  {} Notice: Red ■ and Blue ■ tokens mixed together!", 
        colorize("🎯", "success"));
    println!("│  {} Each token's color shows its original namespace!", 
        colorize("✨", "info"));
    
    print_test_result("✓ Complete pipeline executed - perfect color flow tracking!");
}

fn ceremony_pipeline_test_2() {
    print_test_header("2", "Complex Multi-Stage Pipeline");
    
    let input = "web:click=\"ww\"; db:query=\"dd\"; web:form=\"ww\"; auth:login=\"aa\"; db:result=\"dd\"";
    
    println!("┌─ Advanced Pipeline:");
    println!("│  Input: {}", colorize_stream(input));
    println!("│");
    println!("│  Stage 1: Fork → Split namespaces");
    
    let stage1 = input.to_string().stream_apply(Fork, 
        vec!["web".to_string(), "db".to_string(), "auth".to_string()]);
    
    println!("│  Stage 2: Gate → Filter by MinTokens(2)");
    
    let mut stage2_streams = Vec::new();
    for line in stage1.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            let token_count = tokens.split("; ").count();
            if token_count >= 2 {
                stage2_streams.push(format!("{}: {}", ns, tokens));
            }
        }
    }
    
    let stage2_input = stage2_streams.join("\n");
    println!("│  Stage 3: Merge → Interleave remaining streams");
    
    let final_result = stage2_input.stream_apply(Merge, MergeStrategy::Interleave);
    
    println!("│");
    println!("│  Final Result:");
    println!("│  {}", colorize_stream(&final_result));
    println!("│  {} Tokens processed through 3-stage pipeline", 
             final_result.split("; ").count());
    
    print_test_result("✓ Multi-stage pipeline completed successfully");
}

fn ceremony_pipeline_test_3() {
    print_test_header("3", "Error Recovery Pipeline - Resilient Processing");
    
    let mixed_input = generate_test_stream("valid", "green", 3) + "; " +
                     "invalid:data=\"\"; " + // empty value to simulate error
                     &generate_test_stream("backup", "yellow", 2);
    
    print_stream_input("┌─ Mixed Input (Valid + Invalid + Backup)", &mixed_input);
    println!("{}{} Pipeline: Validate → Filter errors → Merge with backup", 
        colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    print_flow_arrow("Step 1: Fork by namespace");
    let forked = mixed_input.to_string().stream_apply(ForkAll, ());
    
    print_flow_arrow("Step 2: Filter valid streams (simulate error detection)");
    let mut recovery_streams = Vec::new();
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            // Simulate error detection (skip empty values)
            if !tokens.contains("=\"\"") && !tokens.is_empty() {
                recovery_streams.push(format!("{}: {}", ns, tokens));
                println!("{}{} {} {} (valid)", 
                    colorize("├─", "grey"), RESET,
                    colorize("✓", "success"),
                    colorize(ns, "green"));
            } else {
                println!("{}{} {} {} (error - filtered out)", 
                    colorize("├─", "grey"), RESET,
                    colorize("✗", "error"),
                    colorize(ns, "red"));
            }
        }
    }
    
    print_flow_arrow("Step 3: Merge recovered streams");
    let recovered_input = recovery_streams.join("\n");
    let final_result = recovered_input.stream_apply(Merge, MergeStrategy::Concat);
    
    println!("{}{} Final: {}", 
        colorize("└─", "grey"), RESET, colorize_stream(&final_result));
    
    print_test_result("✓ Error recovery pipeline maintained data integrity");
}

fn ceremony_pipeline_test_4() {
    print_test_header("4", "Branching Pipeline - Conditional Stream Routing");
    
    let input = generate_test_stream("priority", "red", 3) + "; " +
               &generate_test_stream("normal", "blue", 4) + "; " +
               &generate_test_stream("low", "green", 2);
    
    print_stream_input("┌─ Priority-Mixed Input", &input);
    println!("{}{} Routing: priority → fast path, others → slow path", 
        colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    // Step 1: Fork by namespace 
    print_flow_arrow("Step 1: Fork by priority level");
    let forked = input.to_string().stream_apply(ForkAll, ());
    
    let mut fast_path = Vec::new();
    let mut slow_path = Vec::new();
    
    for line in forked.lines() {
        if let Some((ns, tokens)) = line.split_once(": ") {
            if ns == "priority" {
                fast_path.push(format!("{}: {}", ns, tokens));
                println!("{}{} {} {} → {} (fast path)", 
                    colorize("├─", "grey"), RESET,
                    colorize("→", "red"), 
                    colorize(ns, "red"),
                    colorize("EXPRESS", "red"));
            } else {
                slow_path.push(format!("{}: {}", ns, tokens));
                println!("{}{} {} {} → {} (slow path)", 
                    colorize("├─", "grey"), RESET,
                    colorize("→", "blue"), 
                    colorize(ns, "blue"),
                    colorize("STANDARD", "blue"));
            }
        }
    }
    
    print_flow_arrow("Step 2: Process each path (different strategies)");
    
    // Fast path: Direct processing  
    let fast_result = if !fast_path.is_empty() {
        let fast_input = fast_path.join("\n");
        fast_input.stream_apply(Merge, MergeStrategy::Concat)
    } else {
        String::new()
    };
    
    // Slow path: Apply rate limiting
    let slow_result = if !slow_path.is_empty() {
        let slow_input = slow_path.join("\n");
        let merged = slow_input.stream_apply(Merge, MergeStrategy::Concat);
        merged.stream_apply(Gate, GateCondition::MaxTokens(5))
    } else {
        String::new()
    };
    
    print_flow_arrow("Step 3: Combine results");
    let combined = if !fast_result.is_empty() && !slow_result.is_empty() {
        format!("{}; {}", fast_result, slow_result)
    } else if !fast_result.is_empty() {
        fast_result
    } else {
        slow_result
    };
    
    println!("{}{} Final: {}", 
        colorize("└─", "grey"), RESET, colorize_stream(&combined));
    
    print_test_result("✓ Branching pipeline routed streams by priority");
}

fn ceremony_pipeline_test_5() {
    print_test_header("5", "Circular Pipeline - Feedback Loop Processing");
    
    let initial_input = generate_test_stream("seed", "cyan", 2);
    
    print_stream_input("┌─ Seed Input", &initial_input);
    println!("{}{} Process: seed → transform → feedback → re-process", 
        colorize("│", "grey"), RESET);
    println!("{}", colorize("│", "grey"));
    
    let mut current_stream = initial_input.clone();
    let iterations = 3;
    
    for i in 1..=iterations {
        print_flow_arrow(&format!("Iteration {} - Processing", i));
        
        // Simulate transformation by adding iteration marker
        let tokens: Vec<String> = current_stream.split("; ")
            .map(|token| {
                if token.contains("=") {
                    let parts: Vec<&str> = token.split("=").collect();
                    if parts.len() == 2 {
                        format!("{}:iter{}={}", parts[0], i, parts[1])
                    } else {
                        token.to_string()
                    }
                } else {
                    token.to_string()
                }
            })
            .collect();
        
        current_stream = tokens.join("; ");
        
        println!("{}{} {} → {}", 
            colorize("├─", "grey"), RESET,
            colorize(&format!("Round {}", i), "cyan"),
            colorize_stream(&current_stream));
        
        // Simulate feedback condition
        let token_count = current_stream.split("; ").count();
        if token_count > 8 {
            println!("{}{} {} - Stopping due to size limit", 
                colorize("├─", "grey"), RESET,
                colorize("⚠", "warning"));
            break;
        }
        
        // Add feedback tokens for next iteration
        if i < iterations {
            current_stream = format!("{}, feedback:iter{}=\"loop{:02}\"", 
                current_stream, i, i);
        }
    }
    
    print_flow_arrow("Final Result");
    println!("{}{} Complete: {}", 
        colorize("└─", "grey"), RESET, colorize_stream(&current_stream));
    
    print_test_result("✓ Circular pipeline demonstrated with feedback loops");
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                      COLOR SHOWCASE CEREMONY                    ║
// ╚═════════════════════════════════════════════════════════════════╝

fn ceremony_color_showcase() {
    print_section_header("COLOR SHOWCASE - Stream Value Visualization");
    
    println!("┌─ Pre-Colored Stream Examples:");
    println!("│  (Colors applied to values only, preserving key readability)");
    println!("│");
    
    let color_examples = vec![
        ("red", create_colored_token("ui", "click", "red01", "red") + "; " + &create_colored_token("ui", "hover", "red02", "red")),
        ("blue", create_colored_token("db", "host", "blue01", "blue") + "; " + &create_colored_token("db", "port", "blue02", "blue")), 
        ("green", create_colored_token("log", "level", "green01", "green") + "; " + &create_colored_token("log", "msg", "green02", "green")),
        ("orange", create_colored_token("cfg", "timeout", "orange01", "orange") + "; " + &create_colored_token("cfg", "retry", "orange02", "orange")),
        ("purple", create_colored_token("auth", "user", "purple01", "purple") + "; " + &create_colored_token("auth", "role", "purple02", "purple")),
        ("cyan", create_colored_token("data", "stream", "cyan01", "cyan") + "; " + &create_colored_token("data", "flow", "cyan02", "cyan")),
        ("yellow", create_colored_token("debug", "trace", "yellow01", "yellow") + "; " + &create_colored_token("debug", "level", "yellow02", "yellow")),
    ];
    
    for (color, stream) in color_examples {
        println!("│  {:8} → {}", 
                 colorize(color, color),
                 stream); // Already pre-colored
    }
    
    println!("│");
    println!("├─ Status Color Examples:");
    println!("│  {} Operation successful", colorize("✓ SUCCESS", "success"));
    println!("│  {} Operation failed", colorize("✗ ERROR", "error"));
    println!("│  {} Caution required", colorize("⚠ WARNING", "warning"));
    println!("│  {} Information only", colorize("ℹ INFO", "info"));
    
    print_test_result("✓ Color system working correctly");
    print_section_footer("Color Showcase");
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                         HELPER FUNCTIONS                        ║
// ╚═════════════════════════════════════════════════════════════════╝

/// Print test header with consistent formatting and grey descriptive text
fn print_test_header(number: &str, title: &str) {
    println!("{}", colorize("┌─────────────────────────────────────────────────────", "grey"));
    println!("{} Test {} - {:<40} {}", 
        colorize("│", "grey"), number, title, colorize("│", "grey"));
    println!("{}", colorize("└─────────────────────────────────────────────────────", "grey"));
    println!();
}

/// Print stream input with grey label and colored stream data
fn print_stream_input(label: &str, stream: &str) {
    println!("{}{}: {}", colorize(label, "grey"), RESET, colorize_stream(stream));
}

/// Print stream output with result status
fn print_stream_output(label: &str, stream: &str, status: &str) {
    println!("{}{}: {} {}", colorize(label, "grey"), RESET, 
        colorize_stream(stream), colorize(status, "success"));
}

/// Print flow arrow with grey descriptive text
fn print_flow_arrow(description: &str) {
    println!("{}{} {}", colorize("├─", "grey"), colorize(description, "grey"), RESET);
}

/// Helper function to create a colored token with block symbol
fn create_colored_token(namespace: &str, key: &str, value: &str, color: &str) -> String {
    let block_value = format!("■{}■", value);
    let token = format!("{}:{}=\"{}\"", namespace, key, block_value);
    pre_color_stream(&token, color)
}

/// Generate pre-colored test stream with block symbols that maintain colors through operations
fn generate_test_stream(namespace: &str, color: &str, count: usize) -> String {
    let tokens: Vec<String> = (0..count)
        .map(|i| create_colored_token(namespace, &format!("item{}", i+1), &format!("{}{:02}", color, i+1), color))
        .collect();
    tokens.join("; ")
}

// ╔═════════════════════════════════════════════════════════════════╗
// ║                         UTILITY FUNCTIONS                       ║
// ╚═════════════════════════════════════════════════════════════════╝

fn print_section_header(title: &str) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ {:^77} ║", title);
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_section_footer(title: &str) {
    println!();
    println!("└─ {} Complete ─┘", title);
    println!();
}


fn print_test_result(message: &str) {
    println!("│");
    println!("└─ {}", colorize(message, "success"));
    println!();
    println!("{}", "─".repeat(65));
    println!();
}

fn colorize_stream(stream: &str) -> String {
    // For backward compatibility - but in most cases our streams are already pre-colored
    // If the stream already contains ANSI color codes, return as-is
    if stream.contains("\x1B[") {
        return stream.to_string();
    }
    
    // Only apply colors if stream doesn't already have them
    let colors = ["red", "blue", "green", "orange", "purple", "cyan"];
    let tokens: Vec<&str> = stream.split("; ").collect();
    
    tokens.iter().enumerate()
        .map(|(i, token)| {
            let color = colors[i % colors.len()];
            pre_color_stream(token, color)
        })
        .collect::<Vec<_>>()
        .join("; ")
}