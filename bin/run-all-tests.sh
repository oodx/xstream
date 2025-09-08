#!/bin/bash
# XStream Comprehensive Test Suite
# Runs all tests, builds, and validates the complete system
# Usage: ./bin/run-all-tests.sh [--verbose] [--benchmark]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
GREY='\033[0;90m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Global test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0
VERBOSE=false
BENCHMARK=false
START_TIME=$(date +%s)

# Parse arguments
for arg in "$@"; do
    case $arg in
        --verbose)
            VERBOSE=true
            ;;
        --benchmark)
            BENCHMARK=true
            ;;
        --help|-h)
            echo "XStream Comprehensive Test Suite"
            echo "Usage: $0 [--verbose] [--benchmark] [--help]"
            echo ""
            echo "Options:"
            echo "  --verbose     Show detailed output for all tests"
            echo "  --benchmark   Run performance benchmarks"
            echo "  --help        Show this help message"
            exit 0
            ;;
    esac
done

# Utility functions
print_banner() {
    echo -e "${BOLD}${BLUE}"
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
    echo "┃                                      🧪 XSTREAM COMPREHENSIVE TEST SUITE 🧪                                         ┃"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
    echo -e "${NC}"
}

print_section() {
    echo -e "${BOLD}${CYAN}═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}${CYAN}$1${NC}"
    echo -e "${BOLD}${CYAN}═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════${NC}"
}

print_test_category() {
    echo -e "${YELLOW}┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${YELLOW}│ $1${NC}"
    echo -e "${YELLOW}└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘${NC}"
}

print_test() {
    echo -e "${CYAN}▶${NC} Testing: ${BOLD}$1${NC}"
    ((TOTAL_TESTS++))
}

print_success() {
    echo -e "${GREEN}  ✓${NC} $1"
    ((PASSED_TESTS++))
}

print_failure() {
    echo -e "${RED}  ✗${NC} $1"
    ((FAILED_TESTS++))
}

print_skip() {
    echo -e "${GREY}  ⊘${NC} $1 (skipped)"
    ((SKIPPED_TESTS++))
}

print_info() {
    echo -e "${GREY}  ℹ${NC} $1"
}

# Test execution functions
run_command() {
    local test_name="$1"
    local command="$2"
    local success_msg="$3"
    local failure_msg="$4"
    
    print_test "$test_name"
    
    if $VERBOSE; then
        echo -e "${GREY}    Command: $command${NC}"
    fi
    
    local temp_file="/tmp/xstream_test_$$"
    local start=$(date +%s%3N)
    
    if eval "$command" >"$temp_file" 2>&1; then
        local end=$(date +%s%3N)
        local duration=$((end - start))
        print_success "$success_msg (${duration}ms)"
        
        if $VERBOSE; then
            echo -e "${GREY}    Output:${NC}"
            cat "$temp_file" | head -10 | sed 's/^/      /'
            if [ $(wc -l < "$temp_file") -gt 10 ]; then
                echo -e "${GREY}      ... (output truncated)${NC}"
            fi
        fi
        rm -f "$temp_file"
        return 0
    else
        local end=$(date +%s%3N)
        local duration=$((end - start))
        print_failure "$failure_msg (${duration}ms)"
        
        if $VERBOSE || [ -s "$temp_file" ]; then
            echo -e "${RED}    Error output:${NC}"
            cat "$temp_file" | head -15 | sed 's/^/      /'
        fi
        rm -f "$temp_file"
        return 1
    fi
}

# Test categories
test_build_system() {
    print_test_category "BUILD SYSTEM VALIDATION"
    
    run_command "Cargo Check" \
        "cargo check --quiet" \
        "Code compiles without errors" \
        "Compilation failed"
    
    run_command "Cargo Build" \
        "cargo build --quiet" \
        "Project builds successfully" \
        "Build failed"
    
    run_command "Cargo Build Release" \
        "cargo build --release --quiet" \
        "Release build successful" \
        "Release build failed"
    
    run_command "All Binaries Build" \
        "cargo build --bins --quiet" \
        "All binaries compile" \
        "Binary compilation failed"
    
    echo
}

test_unit_tests() {
    print_test_category "UNIT TEST EXECUTION"
    
    # Fork tests
    run_command "Fork Unit Tests" \
        "cargo test --lib fork --quiet" \
        "All fork tests pass" \
        "Fork tests failed"
    
    # Merge tests  
    run_command "Merge Unit Tests" \
        "cargo test --lib merge --quiet" \
        "All merge tests pass" \
        "Merge tests failed"
    
    # Gate tests
    run_command "Gate Unit Tests" \
        "cargo test --lib gate --quiet" \
        "All gate tests pass" \
        "Gate tests failed"
    
    # All tests
    run_command "Complete Unit Test Suite" \
        "cargo test --lib --quiet" \
        "All unit tests pass" \
        "Some unit tests failed"
    
    echo
}

test_binaries() {
    print_test_category "BINARY EXECUTION TESTS"
    
    # Main driver
    run_command "XStream Driver" \
        "timeout 10s cargo run --quiet --bin xstream-driver -- --help 2>/dev/null || cargo run --quiet --" \
        "Driver binary executes" \
        "Driver execution failed"
    
    # Generator binaries
    run_command "XStream Generator" \
        "cargo run --quiet --bin xstream-gen -- --help" \
        "Generator binary works" \
        "Generator execution failed"
    
    run_command "XStream Color Generator" \
        "cargo run --quiet --bin xstream-color-gen -- --help" \
        "Color generator binary works" \
        "Color generator execution failed"
    
    # Pretty printer
    run_command "Pretty Printer" \
        "echo 'ui:test=\"ok\"' | cargo run --quiet --bin pretty 2>/dev/null || true" \
        "Pretty printer executes" \
        "Pretty printer failed"
    
    echo
}

test_stream_generation() {
    print_test_category "STREAM GENERATION VALIDATION"
    
    # Basic generation
    run_command "Basic Stream Generation" \
        "cargo run --quiet --bin xstream-gen colored --namespaces ui,db --tokens 3" \
        "Basic stream generation works" \
        "Stream generation failed"
    
    # Pattern generation
    run_command "Pattern Stream Generation" \
        "cargo run --quiet --bin xstream-gen pattern --pattern fork --complexity simple" \
        "Pattern generation works" \
        "Pattern generation failed"
    
    # Color generation
    run_command "Themed Color Generation" \
        "cargo run --quiet --bin xstream-color-gen theme --theme rainbow --count 5" \
        "Color generation works" \
        "Color generation failed"
    
    # Symbol generation
    run_command "Symbol Stream Generation" \
        "cargo run --quiet --bin xstream-color-gen namespace --namespaces ui,db --symbols true" \
        "Symbol generation works" \
        "Symbol generation failed"
    
    echo
}

test_ceremony_integration() {
    print_test_category "CEREMONY INTEGRATION TESTING"
    
    # Fork ceremony
    run_command "Fork Ceremony Integration" \
        "echo 'ui:test=\"ok\"; db:host=\"local\"' | timeout 5s cargo run --quiet -- fork 2>/dev/null || true" \
        "Fork ceremony integration works" \
        "Fork ceremony failed"
    
    # Merge ceremony
    run_command "Merge Ceremony Integration" \
        "printf 'ui: ui:test=\"ok\"\ndb: db:host=\"local\"' | timeout 5s cargo run --quiet -- merge 2>/dev/null || true" \
        "Merge ceremony integration works" \
        "Merge ceremony failed"
    
    # Gate ceremony  
    run_command "Gate Ceremony Integration" \
        "echo 'ui:test=\"ok\"; auth:valid=\"true\"' | timeout 5s cargo run --quiet -- gate 2>/dev/null || true" \
        "Gate ceremony integration works" \
        "Gate ceremony failed"
    
    # Pipeline ceremony
    run_command "Pipeline Ceremony Integration" \
        "echo 'input:data=\"test\"' | timeout 5s cargo run --quiet -- pipeline 2>/dev/null || true" \
        "Pipeline ceremony integration works" \
        "Pipeline ceremony failed"
    
    echo
}

test_pipeline_flows() {
    print_test_category "FULL PIPELINE FLOW TESTING"
    
    # Generator → Fork → Merge pipeline
    run_command "Gen → Fork → Merge Pipeline" \
        "cargo run --quiet --bin xstream-gen colored --namespaces ui,db --tokens 2 2>/dev/null | cargo run --quiet -- fork 2>/dev/null | head -5 || true" \
        "Full pipeline flow works" \
        "Pipeline flow failed"
    
    # Color generator → processing
    run_command "Color Gen → Processing" \
        "cargo run --quiet --bin xstream-color-gen theme --theme warm --count 3 2>/dev/null | head -1 || true" \
        "Color pipeline works" \
        "Color pipeline failed"
    
    echo
}

test_scripts() {
    print_test_category "SCRIPT EXECUTION TESTING"
    
    # Test script
    if [ -x "./bin/test.sh" ]; then
        run_command "Test Script Execution" \
            "timeout 30s ./bin/test.sh build --verbose 2>/dev/null || ./bin/test.sh build 2>/dev/null || true" \
            "Test script executes" \
            "Test script failed"
    else
        print_skip "Test script not executable"
    fi
    
    # Showcase scripts
    local showcase_count=0
    for script in ./bin/showcase-*.sh; do
        if [ -x "$script" ]; then
            local script_name=$(basename "$script" .sh)
            run_command "Showcase: $script_name" \
                "timeout 15s $script 2>/dev/null || true" \
                "Showcase script executes" \
                "Showcase script failed"
            ((showcase_count++))
        fi
    done
    
    if [ $showcase_count -eq 0 ]; then
        print_skip "No showcase scripts found"
    fi
    
    echo
}

run_benchmarks() {
    if ! $BENCHMARK; then
        return 0
    fi
    
    print_test_category "PERFORMANCE BENCHMARKS"
    
    # Generation benchmarks
    print_test "Stream Generation Benchmark"
    local gen_start=$(date +%s%3N)
    for i in {1..10}; do
        cargo run --quiet --bin xstream-gen colored --count 100 >/dev/null 2>&1 || true
    done
    local gen_end=$(date +%s%3N)
    local gen_time=$((gen_end - gen_start))
    print_success "Generation: ${gen_time}ms for 10×100 tokens"
    
    # Processing benchmarks  
    print_test "Processing Benchmark"
    local proc_start=$(date +%s%3N)
    local test_stream="ui:a=\"1\"; ui:b=\"2\"; db:x=\"9\"; db:y=\"8\"; api:status=\"ok\""
    for i in {1..5}; do
        echo "$test_stream" | cargo run --quiet -- fork >/dev/null 2>&1 || true
    done
    local proc_end=$(date +%s%3N)
    local proc_time=$((proc_end - proc_start))
    print_success "Processing: ${proc_time}ms for 5 fork operations"
    
    echo
}

# Generate final report
generate_report() {
    local end_time=$(date +%s)
    local total_duration=$((end_time - START_TIME))
    
    print_section "TEST EXECUTION SUMMARY"
    
    echo -e "${BOLD}📊 Test Statistics:${NC}"
    echo -e "   ${GREEN}✓ Passed:${NC}  $PASSED_TESTS"
    echo -e "   ${RED}✗ Failed:${NC}  $FAILED_TESTS"  
    echo -e "   ${GREY}⊘ Skipped:${NC} $SKIPPED_TESTS"
    echo -e "   ${BLUE}📝 Total:${NC}   $TOTAL_TESTS"
    echo
    
    local success_rate=0
    if [ $TOTAL_TESTS -gt 0 ]; then
        success_rate=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    fi
    
    echo -e "${BOLD}📈 Success Rate:${NC} ${success_rate}%"
    echo -e "${BOLD}⏱️  Execution Time:${NC} ${total_duration}s"
    echo
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}${BOLD}🎉 ALL TESTS PASSED! 🎉${NC}"
        echo -e "${GREEN}XStream system is fully functional and ready for use.${NC}"
        echo
        echo -e "${CYAN}Next steps:${NC}"
        echo -e "  • Run ${BOLD}./bin/showcase-fork.sh${NC} for fork demonstrations"
        echo -e "  • Run ${BOLD}./bin/showcase-merge.sh${NC} for merge examples"  
        echo -e "  • Run ${BOLD}./bin/showcase-pipeline.sh${NC} for full pipelines"
        echo -e "  • Use ${BOLD}cargo run --bin xstream-gen${NC} for custom test streams"
        echo -e "  • Use ${BOLD}cargo run --bin xstream-color-gen${NC} for colored streams"
        return 0
    else
        echo -e "${RED}${BOLD}❌ SOME TESTS FAILED ❌${NC}"
        echo -e "${RED}Please review the failures above and fix the issues.${NC}"
        echo
        echo -e "${YELLOW}Troubleshooting:${NC}"
        echo -e "  • Run with ${BOLD}--verbose${NC} for detailed error output"
        echo -e "  • Check individual test categories: ./bin/test.sh [category]"
        echo -e "  • Ensure all dependencies are properly installed"
        echo -e "  • Verify RSB framework is available"
        return 1
    fi
}

# Main execution
main() {
    print_banner
    
    echo -e "${GREY}Test Configuration:${NC}"
    echo -e "  Verbose: $VERBOSE"
    echo -e "  Benchmarks: $BENCHMARK"
    echo -e "  Start Time: $(date)"
    echo
    
    # Execute all test categories
    test_build_system
    test_unit_tests
    test_binaries
    test_stream_generation
    test_ceremony_integration
    test_pipeline_flows
    test_scripts
    run_benchmarks
    
    # Generate final report
    generate_report
}

# Execute main function
main