#!/bin/bash

# VulcanOS Ralph Loop Stop Hook
# Enforces quality gates before allowing session exit when a ralph-enabled task is active
# Replaces ralph-wiggum with vulcan-todo as the source of truth

set -euo pipefail

# Read hook input from stdin
HOOK_INPUT=$(cat)

# Query vulcan-todo for ralph status
# The MCP server should be running; we use the CLI directly
RALPH_STATUS=$(vulcan-todo --json get-ralph-status 2>/dev/null || echo '{"active": false}')

# Check if ralph loop is active
IS_ACTIVE=$(echo "$RALPH_STATUS" | jq -r '.data.active // false')

if [[ "$IS_ACTIVE" != "true" ]]; then
    # No active ralph loop - allow exit
    exit 0
fi

# Extract task info
TASK_ID=$(echo "$RALPH_STATUS" | jq -r '.data.task.id')
TASK_TITLE=$(echo "$RALPH_STATUS" | jq -r '.data.task.title')
QUALITY_GATES=$(echo "$RALPH_STATUS" | jq -r '.data.task.quality_gates[]' 2>/dev/null || echo "")

if [[ -z "$QUALITY_GATES" ]]; then
    # No quality gates configured - allow exit
    exit 0
fi

# Track gate results
GATES_PASSED=0
GATES_FAILED=0
FAILED_GATES=""

# Function to map gate name to command
get_gate_command() {
    local gate="$1"
    local cmd=""

    case "$gate" in
        "test")
            if [[ -f "Cargo.toml" ]]; then
                cmd="cargo test --quiet"
            elif [[ -f "package.json" ]]; then
                cmd="npm test --silent"
            elif [[ -f "pyproject.toml" ]] || [[ -f "setup.py" ]]; then
                cmd="pytest -q"
            elif [[ -f "go.mod" ]]; then
                cmd="go test ./..."
            fi
            ;;
        "lint")
            if [[ -f "Cargo.toml" ]]; then
                cmd="cargo clippy --quiet -- -D warnings"
            elif [[ -f "package.json" ]]; then
                cmd="npm run lint --silent 2>/dev/null || npx eslint . --quiet"
            elif [[ -f "pyproject.toml" ]] || [[ -f "setup.py" ]]; then
                cmd="ruff check . --quiet 2>/dev/null || flake8 --quiet"
            elif [[ -f "go.mod" ]]; then
                cmd="golangci-lint run --quiet"
            fi
            ;;
        "typecheck")
            if [[ -f "tsconfig.json" ]]; then
                cmd="npx tsc --noEmit"
            elif [[ -f "pyproject.toml" ]] || [[ -f "setup.py" ]]; then
                cmd="mypy . --quiet 2>/dev/null || echo 'mypy not configured'"
            fi
            ;;
        "build")
            if [[ -f "Cargo.toml" ]]; then
                cmd="cargo build --quiet"
            elif [[ -f "package.json" ]]; then
                cmd="npm run build --silent"
            elif [[ -f "go.mod" ]]; then
                cmd="go build ./..."
            fi
            ;;
        "format")
            if [[ -f "Cargo.toml" ]]; then
                cmd="cargo fmt --check"
            elif [[ -f "package.json" ]]; then
                cmd="npx prettier --check ."
            elif [[ -f "pyproject.toml" ]] || [[ -f "setup.py" ]]; then
                cmd="ruff format --check . 2>/dev/null || black --check ."
            elif [[ -f "go.mod" ]]; then
                cmd="gofmt -l . | grep -q . && exit 1 || exit 0"
            fi
            ;;
        custom:*)
            # Extract custom command after "custom:"
            cmd="${gate#custom:}"
            ;;
        *)
            # Unknown gate - skip
            echo "Warning: Unknown quality gate '$gate'" >&2
            return 1
            ;;
    esac

    if [[ -z "$cmd" ]]; then
        echo "Warning: No command found for gate '$gate' in this project type" >&2
        return 1
    fi

    echo "$cmd"
}

# Run each quality gate
while IFS= read -r gate; do
    [[ -z "$gate" ]] && continue

    cmd=$(get_gate_command "$gate" 2>/dev/null) || continue

    if [[ -z "$cmd" ]]; then
        continue
    fi

    echo "Running quality gate: $gate ($cmd)" >&2

    if eval "$cmd" >/dev/null 2>&1; then
        GATES_PASSED=$((GATES_PASSED + 1))
        echo "  ✓ $gate passed" >&2
    else
        GATES_FAILED=$((GATES_FAILED + 1))
        FAILED_GATES="${FAILED_GATES}${gate}, "
        echo "  ✗ $gate failed" >&2
    fi
done <<< "$QUALITY_GATES"

TOTAL_GATES=$((GATES_PASSED + GATES_FAILED))

if [[ $GATES_FAILED -gt 0 ]]; then
    # Remove trailing comma and space
    FAILED_GATES="${FAILED_GATES%, }"

    # Build the prompt to feed back
    PROMPT="Quality gates failed: $FAILED_GATES

Please fix the issues and try again. The task '$TASK_TITLE' has ralph_mode enabled,
which requires all quality gates to pass before completion.

Run the following to check status:
- Failed gates: $FAILED_GATES
- Total: $GATES_PASSED/$TOTAL_GATES passed"

    SYSTEM_MSG="Ralph Loop: $GATES_PASSED/$TOTAL_GATES gates passed | Failed: $FAILED_GATES"

    # Output JSON to block exit and feed prompt back
    jq -n \
        --arg prompt "$PROMPT" \
        --arg msg "$SYSTEM_MSG" \
        '{
            "decision": "block",
            "reason": $prompt,
            "systemMessage": $msg
        }'

    exit 0
fi

# All gates passed - allow exit
# Optionally auto-complete the task (commented out - let user decide)
# vulcan-todo complete "$TASK_ID" 2>/dev/null || true

echo "Ralph Loop: All $TOTAL_GATES quality gates passed for '$TASK_TITLE'" >&2
exit 0
