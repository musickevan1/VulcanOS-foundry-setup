#!/bin/bash
# Diagnostic script to test TUI

echo "=== TUI Diagnostic ==="
echo "Terminal: $TERM"
echo "TTY: $(tty 2>&1)"
echo ""
echo "Attempting to run vulcan-todo stats (non-TUI)..."
/home/evan/VulcanOS/vulcan-todo/target/release/vulcan-todo stats
echo ""
echo "Exit code: $?"
echo ""
echo "If stats worked, the binary is fine."
echo "To test TUI, run this in a real terminal:"
echo "  vulcan-todo"
echo "or"  
echo "  vulcan-todo tui"
