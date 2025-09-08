#!/bin/bash
# XStream Test Suite
# Easy testing commands for all ceremonies
# Usage: ./bin/test.sh [ceremony_name|all] [--verbose]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
GREY='\033[0;90m'
NC='\033[0m' # No Color

# Global counters
TESTS_PASSED=0
TESTS_FAILED=0
VERBOSE=false

# Parse arguments
if [[ "$2" == "--verbose" ]] || [[ "$1" == "--verbose" ]]; then
    VERBOSE=true
fi

CEREMONY=${1:-all}
if [[ "$CEREMONY" == "--verbose" ]]; then
    CEREMONY="all"
fi

# Functions for colored output
print_header() {
    echo -e "${BLUE}┌─────────────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BLUE}│${NC} $1"
    echo -e "${BLUE}└─────────────────────────────────────────────────────────────────────────────┘${NC}"
}

print_test_name() {
    echo -e "${CYAN}▶${NC} Testing: ${YELLOW}$1${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
    ((TESTS_PASSED++))
}

print_failure() {
    echo -e "${RED}✗${NC} $1"
    ((TESTS_FAILED++))
}

print_info() {
    echo -e "${GREY}ℹ${NC} $1"
}

# Test utility functions
run_cargo_test() {
    local test_name=$1
    if $VERBOSE; then
        echo -e "${GREY}Running: cargo test $test_name${NC}"
    fi
    
    if cargo test "$test_name" --lib --quiet >/dev/null 2>&1; then
        print_success "Unit test: $test_name"
        return 0
    else
        print_failure "Unit test: $test_name"
        if $VERBOSE; then
            echo -e "${RED}Error details:${NC}"
            cargo test "$test_name" --lib 2>&1 | head -20
        fi
        return 1
    fi
}

run_ceremony() {
    local ceremony_name=$1
    shift
    if $VERBOSE; then
        echo -e "${GREY}Running: cargo run -- $ceremony_name $@${NC}"
    fi
    
    if cargo run --quiet -- "$ceremony_name" "$@" >/dev/null 2>&1; then
        print_success "Ceremony: $ceremony_name $*"
        return 0
    else
        print_failure "Ceremony: $ceremony_name $*"
        if $VERBOSE; then
            echo -e "${RED}Error details:${NC}"
            cargo run -- "$ceremony_name" "$@" 2>&1 | head -10
        fi
        return 1
    fi
}

generate_test_stream() {
    local namespace=$1
    local tokens=${2:-3}
    for i in $(seq 1 $tokens); do
        echo -n "${namespace}:item${i}=\"val$(printf "%02d" $i)\""
        if [[ $i -lt $tokens ]]; then
            echo -n "; "
        fi
    done
    echo
}

# Individual test functions

test_fork_ceremonies() {
    print_header "🍴 FORK CEREMONY TESTS"
    
    print_test_name "Basic fork operations"
    run_ceremony fork
    
    print_test_name "Fork unit tests"
    run_cargo_test "test_fork_by_namespace"
    run_cargo_test "test_fork_macro" 
    run_cargo_test "test_fork_all"
    run_cargo_test "test_fork_pattern"
    run_cargo_test "test_real_fork_with_streamable"
    
    print_info "Fork ceremony tests completed"
    echo
}

test_merge_ceremonies() {
    print_header "🔀 MERGE CEREMONY TESTS"
    
    print_test_name "Basic merge operations"
    run_ceremony merge
    
    print_test_name "Merge unit tests"
    run_cargo_test "test_merge_concat"
    run_cargo_test "test_merge_macro"
    run_cargo_test "test_merge_interleave"
    run_cargo_test "test_merge_with_strategy"
    run_cargo_test "test_collision_policy"
    run_cargo_test "test_rsb_streamable_merge"
    
    print_info "Merge ceremony tests completed"
    echo
}

test_gate_ceremonies() {
    print_header "🚪 GATE CEREMONY TESTS"
    
    print_test_name "Basic gate operations"
    run_ceremony gate
    
    print_test_name "Gate unit tests"  
    run_cargo_test "test_gate_min_tokens"
    run_cargo_test "test_gate_max_tokens"
    run_cargo_test "test_gate_require_namespace"
    run_cargo_test "test_sync_gate"
    
    print_info "Gate ceremony tests completed"
    echo
}

test_pipeline_ceremonies() {
    print_header "🔄 PIPELINE CEREMONY TESTS"
    
    print_test_name "Basic pipeline operations"
    run_ceremony pipeline
    
    print_info "Pipeline ceremony tests completed"
    echo
}

test_build_system() {
    print_header "🔧 BUILD SYSTEM TESTS"
    
    print_test_name "Project compilation"
    if $VERBOSE; then
        echo -e "${GREY}Running: cargo build${NC}"
    fi
    
    if cargo build --quiet >/dev/null 2>&1; then
        print_success "Project builds successfully"
    else
        print_failure "Project build failed"
        if $VERBOSE; then
            echo -e "${RED}Build errors:${NC}"
            cargo build 2>&1 | head -20
        fi
    fi
    
    print_test_name "All unit tests"
    if cargo test --lib --quiet >/dev/null 2>&1; then
        print_success "All unit tests pass"
    else
        print_failure "Some unit tests failed"
        if $VERBOSE; then
            echo -e "${RED}Test failures:${NC}"
            cargo test --lib 2>&1 | head -20
        fi
    fi
    
    echo
}

test_stream_generation() {
    print_header "📊 STREAM GENERATION TESTS"
    
    print_test_name "Test stream generation utilities"
    
    local test_stream
    test_stream=$(generate_test_stream "ui" 3)
    if [[ -n "$test_stream" ]]; then
        print_success "Stream generation: $test_stream"
    else
        print_failure "Stream generation failed"
    fi
    
    test_stream=$(generate_test_stream "db" 2)  
    if [[ -n "$test_stream" ]]; then
        print_success "Stream generation: $test_stream"
    else
        print_failure "Stream generation failed"
    fi
    
    echo
}

# Main test execution

print_header "🧪 XSTREAM TEST SUITE EXECUTION"
echo -e "${PURPLE}Ceremony:${NC} $CEREMONY"
echo -e "${PURPLE}Verbose:${NC} $VERBOSE"
echo

case $CEREMONY in
    "fork")
        test_fork_ceremonies
        ;;
    "merge")
        test_merge_ceremonies
        ;;
    "gate")
        test_gate_ceremonies
        ;;
    "pipeline")
        test_pipeline_ceremonies
        ;;
    "build")
        test_build_system
        ;;
    "gen"|"generation")
        test_stream_generation
        ;;
    "all"|*)
        test_build_system
        test_stream_generation
        test_fork_ceremonies
        test_merge_ceremonies
        test_gate_ceremonies 
        test_pipeline_ceremonies
        ;;
esac

# Final results
print_header "📋 TEST RESULTS SUMMARY"
echo -e "${GREEN}Passed:${NC} $TESTS_PASSED"
echo -e "${RED}Failed:${NC} $TESTS_FAILED"
echo -e "${PURPLE}Total:${NC} $((TESTS_PASSED + TESTS_FAILED))"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed.${NC}"
    exit 1
fi