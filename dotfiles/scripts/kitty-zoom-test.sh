#!/bin/bash
# Test script for kitty zoom indicator
# Run this INSIDE a kitty terminal to debug

echo "=== Kitty Zoom Indicator Debug ==="
echo ""

# Check environment
echo "1. KITTY_PID: $KITTY_PID"
echo "2. KITTY_WINDOW_ID: $KITTY_WINDOW_ID"
echo ""

# Test remote control
echo "3. Testing remote control..."
if kitten @ ls &>/dev/null; then
    echo "   ✓ Remote control is working"
else
    echo "   ✗ Remote control FAILED - check allow_remote_control setting"
    echo "   Run: grep allow_remote ~/.config/kitty/kitty.conf"
fi
echo ""

# Try to set a user var
echo "4. Setting user var zoom=TEST..."
if kitten @ set-user-vars zoom=TEST 2>&1; then
    echo "   ✓ set-user-vars succeeded"
else
    echo "   ✗ set-user-vars FAILED"
fi
echo ""

# Check current window info
echo "5. Current window user_vars:"
kitten @ ls 2>/dev/null | python3 -c "
import sys, json
data = json.load(sys.stdin)
for os_win in data:
    for tab in os_win.get('tabs', []):
        for win in tab.get('windows', []):
            if win.get('is_self'):
                print(f'   Window ID: {win.get(\"id\")}')
                print(f'   user_vars: {win.get(\"user_vars\", {})}')
" 2>/dev/null || echo "   Could not parse window info"

echo ""
echo "6. Tab title template from config:"
grep "tab_title_template" ~/.config/kitty/kitty.conf | head -2
echo ""
echo "=== End Debug ==="
