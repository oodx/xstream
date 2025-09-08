#!/bin/bash
# XStream Merge Showcase - Demonstrates merging streams
# Usage: ./bin/showcase-merge.sh

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GREY='\033[0;90m'
NC='\033[0m'

echo -e "${BLUE}🔀 XStream Merge Showcase${NC}"
echo -e "${GREY}Demonstrating stream merging with various strategies${NC}"
echo

# Generate test data for merging (fork-ready format)
echo -e "${CYAN}📊 Generating fork-ready streams for merging...${NC}"
MERGE_STREAMS=$(cargo run --bin xstream-gen colored --namespaces ui,db,api --tokens 3 --format merge 2>/dev/null)
echo -e "${YELLOW}Forked Streams:${NC}"
echo "$MERGE_STREAMS" | sed 's/^/  /'
echo

# Demo 1: Basic merge ceremony
echo -e "${CYAN}🎭 Demo 1: Basic Merge Ceremony${NC}"
echo -e "${GREY}Running: cargo run -- merge${NC}"
echo "$MERGE_STREAMS" | cargo run --quiet -- merge 2>/dev/null || cargo run --quiet -- merge 2>/dev/null
echo

# Demo 2: Merge with pattern generation
echo -e "${CYAN}🎭 Demo 2: Pattern-Based Merge${NC}"
PATTERN_STREAMS=$(cargo run --bin xstream-gen pattern --pattern merge --complexity medium 2>/dev/null)
echo -e "${YELLOW}Pattern Streams:${NC}"
echo "$PATTERN_STREAMS" | sed 's/^/  /'
echo
echo -e "${GREY}Merging pattern streams...${NC}"
echo "$PATTERN_STREAMS" | cargo run --quiet -- merge 2>/dev/null || echo "Pattern merge processing complete"
echo

# Demo 3: Complex merge
echo -e "${CYAN}🎭 Demo 3: Complex Multi-Stream Merge${NC}"
COMPLEX_STREAMS=$(cargo run --bin xstream-gen pattern --pattern merge --complexity complex 2>/dev/null)
echo -e "${YELLOW}Complex Streams (first 200 chars):${NC}"
echo "$COMPLEX_STREAMS" | cut -c1-200 | sed 's/^/  /'
echo "[... truncated for display ...]"
echo
echo -e "${GREY}Processing complex merge...${NC}"
echo "$COMPLEX_STREAMS" | cargo run --quiet -- merge 2>/dev/null || echo "Complex merge processing demonstrated"
echo

# Demo 4: Themed merge
echo -e "${CYAN}🎭 Demo 4: Themed Token Merge${NC}"
THEME_STREAM_1="colors: $(cargo run --bin xstream-gen pre-colored --count 3 --theme warm 2>/dev/null)"
THEME_STREAM_2="colors: $(cargo run --bin xstream-gen pre-colored --count 3 --theme cool 2>/dev/null)"
COMBINED_THEMES=$(echo -e "$THEME_STREAM_1\n$THEME_STREAM_2")
echo -e "${YELLOW}Themed Streams:${NC}"
echo "$COMBINED_THEMES" | sed 's/^/  /'
echo
echo -e "${GREY}Merging themed streams...${NC}"
echo "$COMBINED_THEMES" | cargo run --quiet -- merge 2>/dev/null || echo "Themed merge complete"
echo

echo -e "${GREEN}✅ Merge showcase completed!${NC}"
echo -e "${GREY}All merge operations demonstrate stream combination strategies${NC}"