// XStream Local Color Subset - Channel visualization and messaging
// Subset of your full color palette from paintbox/jynx

pub const RESET: &str = "\x1B[0m";

/// Channel colors for fork/merge visualization (your stderr favorites)
pub const CHANNEL_COLORS: &[(&str, &str)] = &[
    ("red", "\x1B[38;5;9m"),        // Channel 1 - UI/Frontend
    ("blue", "\x1B[36m"),           // Channel 2 - Database  
    ("green", "\x1B[38;5;10m"),     // Channel 3 - Success/Logs
    ("orange", "\x1B[38;5;214m"),   // Channel 4 - Warnings/Config
    ("purple", "\x1B[38;5;213m"),   // Channel 5 - Auth/Security
    ("cyan", "\x1B[38;5;14m"),      // Channel 6 - Data/Streams
    ("yellow", "\x1B[33m"),         // Channel 7 - Alerts/Debug
];

/// Message colors for driver output
#[cfg(feature = "rsb-visuals")]
pub fn get_color(name: &str) -> &'static str {
    rsb::visual::colors::get_color(name)
}

/// Message colors for driver output (local fallback)
#[cfg(not(feature = "rsb-visuals"))]
pub fn get_color(name: &str) -> &'static str {
    match name {
        // Your legacy stderr colors
        "red" => "\x1B[38;5;9m",
        "red2" => "\x1B[38;5;197m",
        "magenta" => "\x1B[35m",
        "blue" => "\x1B[36m",
        "blue2" => "\x1B[38;5;39m",
        "green" => "\x1B[38;5;10m",
        "orange" => "\x1B[38;5;214m",
        "purple" => "\x1B[38;5;213m",
        "purple2" => "\x1B[38;5;141m",
        "cyan" => "\x1B[38;5;14m",
        "yellow" => "\x1B[38;5;220m",
        "grey" => "\x1B[38;5;242m",
        "white" => "\x1B[38;5;247m",

        // Common semantic colors
        "success" => "\x1B[38;5;46m",
        "warning" => "\x1B[38;5;220m",
        "error" => "\x1B[38;5;196m",
        "info" => "\x1B[38;5;33m",

        _ => "",
    }
}

/// Get channel color by index
pub fn get_channel_color(index: usize) -> &'static str {
    CHANNEL_COLORS[index % CHANNEL_COLORS.len()].1
}

/// Get channel color name by index  
pub fn get_channel_color_name(index: usize) -> &'static str {
    CHANNEL_COLORS[index % CHANNEL_COLORS.len()].0
}

/// Colorize text with specified color
#[cfg(feature = "rsb-visuals")]
pub fn colorize(text: &str, color: &str) -> String {
    rsb::visual::colors::colorize(text, color)
}

#[cfg(not(feature = "rsb-visuals"))]
pub fn colorize(text: &str, color: &str) -> String {
    format!("{}{}{}", get_color(color), text, RESET)
}

/// Pre-color stream tokens - applies color to VALUES inside quotes
pub fn pre_color_stream(stream: &str, color_name: &str) -> String {
    let color_code = get_color(color_name);
    if color_code.is_empty() {
        return stream.to_string();
    }
    
    // Use same color codes as get_color() for consistency
    let simple_color = get_color(color_name);
    
    // Apply color only to quoted values: key="value" becomes key="[COLOR]value[RESET]"
    let re = regex::Regex::new(r#"="([^"]+)""#).unwrap();
    re.replace_all(stream, &format!(r#"="{}$1{}""#, simple_color, RESET)).to_string()
}

/// Create multiple pre-colored streams for testing
pub fn create_pre_colored_streams(base_streams: &[&str], colors: &[&str]) -> Vec<String> {
    base_streams.iter()
        .zip(colors.iter().cycle())
        .map(|(stream, color)| pre_color_stream(stream, color))
        .collect()
}

/// Generate test streams with color blocks for visual testing
pub fn gen_color_test_streams(colors: &[&str]) -> Vec<(String, String)> {
    colors.iter().enumerate().map(|(i, color)| {
        let namespace = char::from(b'a' + (i as u8));
        let color_char = color.chars().next().unwrap_or('x');
        let stream = format!("{}:x=\"{}■\"; {}:y=\"{}■\"", namespace, color_char, namespace, color_char);
        (color.to_string(), stream)
    }).collect()
}

/// Generate synchronized test streams with consistent token counts
pub fn gen_sync_test_streams(colors: &[&str], token_count: usize) -> Vec<String> {
    colors.iter().enumerate().map(|(i, color)| {
        let namespace = char::from(b'a' + (i as u8));
        let color_char = color.chars().next().unwrap_or('x');
        let tokens: Vec<String> = (0..token_count).map(|j| {
            let key = char::from(b'w' + (j as u8));
            format!("{}:{}=\"{}■\"", namespace, key, color_char)
        }).collect();
        let stream = tokens.join("; ");
        pre_color_stream(&stream, color)
    }).collect()
}

/// Colorize namespace tokens with channel colors
pub fn colorize_namespace_tokens(tokens: &str, namespace: &str, color_index: usize) -> String {
    let color_code = get_channel_color(color_index);
    let color_name = get_channel_color_name(color_index);
    
    // Format: [RED] namespace: tokens
    format!("{}[{}]{} {}: {}{}{}", 
            color_code, 
            color_name.to_uppercase(), 
            RESET,
            namespace,
            color_code,
            tokens,
            RESET)
}

/// Create visual separator with colors
pub fn colored_separator(title: &str) -> String {
    let blue = get_color("blue");
    format!("{}=== {} ==={}", blue, title, RESET)
}

/// Colorize fork operation display with visual stream weaving
pub fn colorize_fork_display(input: &str, forks: &[(String, String)]) -> String {
    let mut result = String::new();
    
    // Show original stream as unified flow
    result.push_str(&format!("{}Input Stream:{} {}\n", get_color("cyan"), RESET, input));
    
    // Visual fork separator - showing the split
    result.push_str(&format!("{}    │\n", get_color("grey")));
    result.push_str(&format!("{}    ├─── FORK ────\n", get_color("yellow")));
    result.push_str(&format!("{}    │\n", get_color("grey")));
    
    // Show each forked channel as flowing streams
    for (index, (namespace, tokens)) in forks.iter().enumerate() {
        let color_code = get_channel_color(index);
        let branch_char = if index == forks.len() - 1 { "└" } else { "├" };
        
        result.push_str(&format!("{}    {}── {}", get_color("grey"), branch_char, color_code));
        result.push_str(&format!("[{}] {} ══> {}{}\n", 
            get_channel_color_name(index).to_uppercase(),
            namespace,
            tokens,
            RESET));
    }
    
    result.push_str("\n");
    result
}

/// Colorize individual tokens in merged result by their namespace
pub fn colorize_merged_result(result: &str, namespace_colors: &std::collections::HashMap<String, usize>) -> String {
    let tokens = result.split("; ");
    let colored_tokens: Vec<String> = tokens.map(|token| {
        // Extract namespace from token (before : or assume global)
        if let Some(colon_pos) = token.find(':') {
            let namespace = &token[..colon_pos];
            if let Some(&color_index) = namespace_colors.get(namespace) {
                let color_code = get_channel_color(color_index);
                return format!("{}{}{}", color_code, token, RESET);
            }
        }
        // No namespace or unknown namespace - use default color
        token.to_string()
    }).collect();
    
    colored_tokens.join(&format!("{}; {}", RESET, ""))
}

/// Colorize merge operation display with visual stream weaving
pub fn colorize_merge_display(inputs: &[(String, String)], result: &str) -> String {
    let mut display = String::new();
    
    // Build namespace color mapping
    let mut namespace_colors = std::collections::HashMap::new();
    for (index, (namespace, _)) in inputs.iter().enumerate() {
        namespace_colors.insert(namespace.clone(), index);
    }
    
    // Show input streams flowing in
    display.push_str(&format!("{}Input Channels:{}\n", get_color("purple"), RESET));
    for (index, (namespace, tokens)) in inputs.iter().enumerate() {
        let color_code = get_channel_color(index);
        let branch_char = if index == inputs.len() - 1 { "└" } else { "├" };
        
        display.push_str(&format!("{}    {}── {}", get_color("grey"), branch_char, color_code));
        display.push_str(&format!("[{}] {} ══> {}{}\n", 
            get_channel_color_name(index).to_uppercase(),
            namespace,
            tokens,
            RESET));
    }
    
    // Visual merge separator - showing the weaving
    display.push_str(&format!("{}    │\n", get_color("grey")));
    display.push_str(&format!("{}    ├─── MERGE ───\n", get_color("yellow")));
    display.push_str(&format!("{}    ▼\n", get_color("grey")));
    
    // Show woven result stream
    let colored_result = colorize_merged_result(result, &namespace_colors);
    display.push_str(&format!("{}Woven Stream:{} {}\n\n", get_color("green"), RESET, colored_result));
    
    display
}

/// Show complete fork-transform-merge workflow with visual stream weaving
pub fn colorize_workflow_display(
    input: &str,
    forks: &[(String, String)], 
    transforms: &[(String, String)],
    final_result: &str
) -> String {
    let mut display = String::new();
    
    // Input stream
    display.push_str(&format!("{}🌊 Original Stream:{} {}\n", get_color("cyan"), RESET, input));
    
    // Fork visualization
    display.push_str(&format!("{}    │\n", get_color("grey")));
    display.push_str(&format!("{}    ├─── FORK ────\n", get_color("yellow")));
    display.push_str(&format!("{}    │\n", get_color("grey")));
    
    // Show forked channels with transforms
    for (index, ((_namespace, original), (_, transformed))) in forks.iter().zip(transforms.iter()).enumerate() {
        let color_code = get_channel_color(index);
        let branch_char = if index == forks.len() - 1 { "└" } else { "├" };
        
        // Show original channel
        display.push_str(&format!("{}    {}── {}", get_color("grey"), branch_char, color_code));
        display.push_str(&format!("[{}] {}{}\n", 
            get_channel_color_name(index).to_uppercase(),
            original,
            RESET));
            
        // Show transformation arrow
        display.push_str(&format!("{}         │{} {}⚡ transform{}\n", 
            get_color("grey"), 
            color_code,
            RESET,
            RESET));
            
        // Show transformed result
        display.push_str(&format!("{}         ▼ {}{}{}\n", 
            get_color("grey"),
            color_code,
            transformed,
            RESET));
    }
    
    // Merge back together
    display.push_str(&format!("{}    │\n", get_color("grey")));
    display.push_str(&format!("{}    ├─── MERGE ───\n", get_color("yellow")));
    display.push_str(&format!("{}    ▼\n", get_color("grey")));
    
    // Final woven result
    display.push_str(&format!("{}🎯 Woven Result:{} {}\n", get_color("green"), RESET, final_result));
    
    display
}

/// Show XOR gate weaving with visual stream switching
pub fn colorize_xor_weaving(
    stream_a: &str,
    stream_b: &str, 
    result: &str,
    gate_state: &crate::xstream::gate::GateState
) -> String {
    let mut display = String::new();
    
    // Show input streams
    display.push_str(&format!("{}🔄 XOR Gate Inputs:{}\n", get_color("purple"), RESET));
    display.push_str(&format!("{}  Stream A:{} {}{}{}\n", 
        get_color("cyan"), RESET, get_channel_color(0), stream_a, RESET));
    display.push_str(&format!("{}  Stream B:{} {}{}{}\n", 
        get_color("cyan"), RESET, get_channel_color(1), stream_b, RESET));
    
    // Visual gate representation
    display.push_str(&format!("{}    │ │\n", get_color("grey")));
    display.push_str(&format!("{}    ├─┼─── XOR GATE ───\n", get_color("yellow")));
    display.push_str(&format!("{}    │ │    (only one passes)\n", get_color("grey")));
    display.push_str(&format!("{}    ▼ ▼\n", get_color("grey")));
    
    // Show the weaving pattern
    let result_tokens: Vec<&str> = result.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    
    display.push_str(&format!("{}📈 Woven Pattern:{}\n", get_color("green"), RESET));
    
    for (i, token) in result_tokens.iter().enumerate() {
        let switch_info = gate_state.switches.get(i);
        let (stream_letter, color_index) = match switch_info {
            Some((_, stream)) if stream == "A" => ("A", 0),
            Some((_, stream)) if stream == "B" => ("B", 1),
            _ => ("?", 2),
        };
        
        let color_code = get_channel_color(color_index);
        let position_marker = if i % 2 == 0 { "├─" } else { "└─" };
        
        display.push_str(&format!("{}  {}[{}] {}{}{}\n", 
            get_color("grey"),
            position_marker,
            stream_letter,
            color_code,
            token,
            RESET));
    }
    
    display.push_str(&format!("\n{}🎯 Final Woven Stream:{} ", get_color("green"), RESET));
    
    // Colorize the final result by alternating colors
    let colored_tokens: Vec<String> = result_tokens.iter().enumerate().map(|(i, token)| {
        let color_index = i % 2; // Alternate between stream colors
        let color_code = get_channel_color(color_index);
        format!("{}{}{}", color_code, token, RESET)
    }).collect();
    
    display.push_str(&colored_tokens.join(&format!("{}; {}", RESET, "")));
    display.push_str("\n\n");
    
    display
}

/// Show multi-stream XOR gate weaving
pub fn colorize_multi_xor_weaving(streams: &[(String, String)], result: &str) -> String {
    let mut display = String::new();
    
    // Show all input streams
    display.push_str(&format!("{}🔄 Multi-XOR Gate Inputs:{}\n", get_color("purple"), RESET));
    for (index, (name, content)) in streams.iter().enumerate() {
        let color_code = get_channel_color(index);
        display.push_str(&format!("{}  Stream {}:{} {}{}{}\n", 
            get_color("cyan"), name, RESET, color_code, content, RESET));
    }
    
    // Visual multi-gate with proper branching
    display.push_str(&format!("{}    │\n", get_color("grey")));
    display.push_str(&format!("{}    ├─── MULTI-XOR GATE ───\n", get_color("yellow")));
    display.push_str(&format!("{}    │\n", get_color("grey")));
    display.push_str(&format!("{}    (cycles through {} streams)\n", 
        get_color("grey"), streams.len()));
    
    // Show weaving result
    display.push_str(&format!("{}    ▼\n", get_color("grey")));
    display.push_str(&format!("{}🎯 Cycling Woven Stream:{} ", get_color("green"), RESET));
    
    // Colorize result tokens cycling through stream colors
    let result_tokens: Vec<&str> = result.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let colored_tokens: Vec<String> = result_tokens.iter().enumerate().map(|(i, token)| {
        let color_index = i % streams.len(); // Cycle through all stream colors
        let color_code = get_channel_color(color_index);
        format!("{}{}{}", color_code, token, RESET)
    }).collect();
    
    display.push_str(&colored_tokens.join(&format!("{}; {}", RESET, "")));
    display.push_str("\n\n");
    
    display
}
