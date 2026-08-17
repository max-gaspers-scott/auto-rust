#!/bin/bash
# Test script for auto-rust Goose integration
# This script creates a test project and shows how to use Goose with the auto-rust tools

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}Auto-Rust Goose Integration Test${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""

# Create a test project directory
TEST_DIR="/tmp/test-rust-project-$(date +%s)"
echo -e "${YELLOW}Creating test directory: ${TEST_DIR}${NC}"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo ""
echo -e "${GREEN}✓ Test directory created${NC}"
echo ""

# Show the recipe path
RECIPE_PATH="/home/mgs/auto-rust-ai-agent/auto-rust-recipe.yaml"
echo -e "${BLUE}Recipe location: ${RECIPE_PATH}${NC}"
echo ""

# Check if Goose is installed
if ! command -v goose &> /dev/null; then
    echo -e "${YELLOW}⚠ Goose is not installed!${NC}"
    echo "Install from: https://github.com/block/goose"
    exit 1
fi

echo -e "${GREEN}✓ Goose found: $(which goose)${NC}"
echo -e "${GREEN}✓ Version: $(goose --version)${NC}"
echo ""

# Instructions for the user
echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}To test the integration, run:${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""
echo -e "${YELLOW}cd $TEST_DIR${NC}"
echo -e "${YELLOW}goose run --recipe $RECIPE_PATH -s${NC}"
echo ""
echo "Then try these commands in Goose:"
echo ""
echo "  1. \"What tools do you have access to?\""
echo "  2. \"Show me the auto-rust tools\""
echo "  3. \"Use setup_rust_project to create a new project\""
echo "  4. \"Generate SQL for a users table with id, email, and password_hash\""
echo ""
echo -e "${BLUE}================================================${NC}"
echo ""
echo -e "${GREEN}Test directory ready at: $TEST_DIR${NC}"
echo ""
echo "Note: You're now in the test directory. Run the goose command above to start!"
