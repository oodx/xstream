#!/bin/bash
# Clean test runner - suppresses cargo build noise

echo "Running XStream tests..."
echo "========================"

# Run tests and filter output
cargo test "$@" 2>&1 | grep -E "^(test |running |test result:|\s+---|FAILED)" | sed 's/^/  /'

echo "========================"
echo "Done!"