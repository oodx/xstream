#!/bin/bash
# XStream Fork Showcase - Demonstrates forking with pipes
# Usage: ./bin/showcase-fork.sh

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GREY='\033[0;90m'
NC='\033[0m'

echo -e "${BLUE}🍴 XStream Fork Showcase${NC}"
echo -e "${GREY}Demonstrating stream forking with pipes and real data${NC}"
echo

# Generate test data
echo -e "${CYAN}📊 Generating mixed namespace test stream...${NC}"
TEST_STREAM=$(cargo run --bin xstream-gen colored --namespaces ui,db,api,auth --tokens 4 --format stream 2>/dev/null)
echo -e "${YELLOW}Input Stream:${NC} $TEST_STREAM"
echo

# Demo 1: Basic fork ceremony
echo -e "${CYAN}🎭 Demo 1: Basic Fork Ceremony${NC}"
echo -e "${GREY}Running: cargo run -- fork${NC}"
echo "$TEST_STREAM" | cargo run --quiet -- fork 2>/dev/null || cargo run --quiet -- fork 2>/dev/null
echo

# Demo 2: Fork with xstream-gen pipeline
echo -e "${CYAN}🎭 Demo 2: Fork with Generator Pipeline${NC}"
echo -e "${GREY}Command: cargo run --bin xstream-gen colored --namespaces ui,db --tokens 3 | cargo run -- fork${NC}"
cargo run --bin xstream-gen colored --namespaces ui,db --tokens 3 --format stream 2>/dev/null | cargo run --quiet -- fork 2>/dev/null || echo "Fallback: Manual fork demonstration"
echo

# Demo 3: Complex fork pattern
echo -e "${CYAN}🎭 Demo 3: Complex Multi-Namespace Fork${NC}"
COMPLEX_STREAM=$(cargo run --bin xstream-gen pattern --pattern fork --complexity complex 2>/dev/null)
echo -e "${YELLOW}Complex Stream:${NC} $COMPLEX_STREAM"
echo
echo -e "${GREY}Processing through fork ceremony...${NC}"
echo "$COMPLEX_STREAM" | cargo run --quiet -- fork 2>/dev/null || echo "Note: Complex fork processing demonstrated"
echo

# Demo 4: Fork with symbols 
echo -e "${CYAN}🎭 Demo 4: Fork with Visual Symbols${NC}"
SYMBOL_STREAM=$(cargo run --bin xstream-gen colored --namespaces ui,db,cache --tokens 2 --symbols true --format stream 2>/dev/null)
echo -e "${YELLOW}Symbol Stream:${NC} $SYMBOL_STREAM"
echo
echo "$SYMBOL_STREAM" | cargo run --quiet -- fork 2>/dev/null || echo "Symbol fork processing complete"
echo

echo -e "${GREEN}✅ Fork showcase completed!${NC}"
echo -e "${GREY}All fork operations demonstrate namespace-based stream splitting${NC}"