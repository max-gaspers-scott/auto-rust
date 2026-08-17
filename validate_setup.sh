#!/bin/bash
# Validation script for Goose + Auto Rust integration

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Auto-Rust Goose Integration Validator       ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
echo ""

ERRORS=0
WARNINGS=0

# Check 1: Python venv exists
echo -n "Checking Python virtual environment... "
if [ -d "venv" ] && [ -f "venv/bin/python" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo "  Run: python3 -m venv venv && venv/bin/pip install fastmcp"
    ERRORS=$((ERRORS + 1))
fi

# Check 2: fastmcp installed
echo -n "Checking fastmcp installation... "
if venv/bin/python -c "import fastmcp" 2>/dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo "  Run: venv/bin/pip install fastmcp"
    ERRORS=$((ERRORS + 1))
fi

# Check 3: MCP server file exists
echo -n "Checking MCP server file... "
if [ -f "auto_rust_mcp.py" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo "  Missing: auto_rust_mcp.py"
    ERRORS=$((ERRORS + 1))
fi

# Check 4: Recipe file exists
echo -n "Checking recipe file... "
if [ -f "auto-rust-recipe.yaml" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo "  Missing: auto-rust-recipe.yaml"
    ERRORS=$((ERRORS + 1))
fi

# Check 5: MCP server is executable
echo -n "Checking MCP server permissions... "
if [ -x "auto_rust_mcp.py" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${YELLOW}⚠${NC}"
    echo "  Run: chmod +x auto_rust_mcp.py"
    WARNINGS=$((WARNINGS + 1))
fi

# Check 6: Goose installed
echo -n "Checking Goose installation... "
if command -v goose &> /dev/null; then
    VERSION=$(goose --version 2>&1 | head -1)
    echo -e "${GREEN}✓${NC} ($VERSION)"
else
    echo -e "${RED}✗${NC}"
    echo "  Install from: https://github.com/block/goose"
    ERRORS=$((ERRORS + 1))
fi

# Check 7: Cargo installed
echo -n "Checking Cargo (Rust)... "
if command -v cargo &> /dev/null; then
    VERSION=$(cargo --version | cut -d' ' -f2)
    echo -e "${GREEN}✓${NC} (v$VERSION)"
else
    echo -e "${RED}✗${NC}"
    echo "  Install from: https://rustup.rs"
    ERRORS=$((ERRORS + 1))
fi

# Check 8: Cargo.toml edition
echo -n "Checking Cargo.toml edition... "
EDITION=$(grep '^edition' Cargo.toml | cut -d'"' -f2)
if [ "$EDITION" = "2021" ] || [ "$EDITION" = "2018" ]; then
    echo -e "${GREEN}✓${NC} (edition = \"$EDITION\")"
elif [ "$EDITION" = "2024" ]; then
    echo -e "${YELLOW}⚠${NC} (edition = \"2024\" is invalid)"
    echo "  Should be: 2021, 2018, or 2015"
    echo "  Fix: sed -i 's/edition = \"2024\"/edition = \"2021\"/' Cargo.toml"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${YELLOW}?${NC} (edition = \"$EDITION\")"
    WARNINGS=$((WARNINGS + 1))
fi

# Check 9: Test script exists
echo -n "Checking test script... "
if [ -f "test_in_project.sh" ] && [ -x "test_in_project.sh" ]; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${YELLOW}⚠${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

# Check 10: Try importing MCP server
echo -n "Testing MCP server syntax... "
if timeout 2 venv/bin/python -c "import sys; sys.path.insert(0, '.'); import auto_rust_mcp" 2>/dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${YELLOW}⚠${NC}"
    echo "  MCP server has import errors (check syntax)"
    WARNINGS=$((WARNINGS + 1))
fi

echo ""
echo -e "${BLUE}════════════════════════════════════════════════${NC}"
echo ""

# Summary
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed!${NC}"
    echo ""
    echo "Ready to test! Run:"
    echo -e "  ${YELLOW}./test_in_project.sh${NC}"
    echo ""
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠ Setup complete with $WARNINGS warning(s)${NC}"
    echo ""
    echo "You can proceed with testing, but consider fixing warnings."
    echo ""
else
    echo -e "${RED}✗ Found $ERRORS error(s) and $WARNINGS warning(s)${NC}"
    echo ""
    echo "Fix the errors above before testing."
    echo ""
    exit 1
fi

# Show next steps
echo -e "${BLUE}Next Steps:${NC}"
echo "  1. Run: ./test_in_project.sh"
echo "  2. Or manually: goose run --recipe $(pwd)/auto-rust-recipe.yaml -s"
echo ""
