#!/usr/bin/env bash
# Quick test script for vulcan-s2t model improvements
# Usage: test-s2t.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== vulcan-s2t Quick Test ==="
echo ""

# Check 1: Scripts exist and are executable
echo "1. Checking scripts..."
for script in vulcan-s2t vulcan-s2t-cloud; do
    if [[ -f "$SCRIPT_DIR/$script" ]]; then
        if [[ -x "$SCRIPT_DIR/$script" ]]; then
            echo "  ✓ $script - exists and executable"
        else
            echo "  ✗ $script - exists but not executable"
            chmod +x "$SCRIPT_DIR/$script"
        fi
    else
        echo "  ✗ $script - not found"
    fi
done
echo ""

# Check 2: Prompt files exist
echo "2. Checking prompt files..."
PROMPT_DIR="$HOME/.config/vulcan-s2t/prompts"
for prompt in exact.txt clean.txt agent.txt opencode.txt exact-code.txt; do
    if [[ -f "$PROMPT_DIR/$prompt" ]]; then
        echo "  ✓ $prompt"
    else
        echo "  ✗ $prompt - not found in $PROMPT_DIR"
    fi
done
echo ""

# Check 3: Python syntax
echo "3. Checking Python syntax..."
if python3 -m py_compile "$SCRIPT_DIR/vulcan-s2t-cloud" 2>/dev/null; then
    echo "  ✓ vulcan-s2t-cloud - valid Python syntax"
else
    echo "  ✗ vulcan-s2t-cloud - syntax error"
fi
echo ""

# Check 4: Config template
echo "4. Checking settings.conf.default..."
SETTINGS_DEFAULT="$HOME/VulcanOS/dotfiles/vulcan-s2t/.config/vulcan-s2t/settings.conf.default"
if [[ -f "$SETTINGS_DEFAULT" ]]; then
    echo "  ✓ settings.conf.default exists"
    if grep -q "S2T_MODEL_OPENCODE" "$SETTINGS_DEFAULT"; then
        echo "  ✓ Mode-specific models configured"
    else
        echo "  ✗ Mode-specific models not found"
    fi
    if grep -q "S2T_ENABLE_FALLBACK" "$SETTINGS_DEFAULT"; then
        echo "  ✓ Fallback model support configured"
    else
        echo "  ✗ Fallback model support not found"
    fi
else
    echo "  ✗ settings.conf.default not found at $SETTINGS_DEFAULT"
fi
echo ""

# Check 5: API connection
echo "5. Testing API connection..."
if "$SCRIPT_DIR/vulcan-s2t" test 2>/dev/null; then
    echo "  ✓ API connection working"
else
    echo "  ✗ API connection failed"
fi
echo ""

# Check 6: New CLI commands
echo "6. Checking new CLI commands..."
if "$SCRIPT_DIR/vulcan-s2t" modes 2>/dev/null | grep -q "available"; then
    echo "  ✓ vulcan-s2t modes command works"
else
    echo "  ✗ vulcan-s2t modes command failed"
fi
if "$SCRIPT_DIR/vulcan-s2t" mode 2>/dev/null | grep -q "Default mode"; then
    echo "  ✓ vulcan-s2t mode command works"
else
    echo "  ✗ vulcan-s2t mode command failed"
fi
echo ""

# Summary
echo "=== Test Complete ==="
echo ""
echo "If all checks passed, try recording:"
echo "  vulcan-s2t start clean"
echo ""
echo "To test different modes:"
echo "  vulcan-s2t start exact-code"
echo "  vulcan-s2t start opencode"
