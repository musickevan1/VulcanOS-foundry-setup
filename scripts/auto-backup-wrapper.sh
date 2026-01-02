#!/bin/bash
# Automatic Backup Wrapper for Orchestrator Sessions
# Runs before each destructive phase to ensure rollback capability
# Usage: auto-backup-wrapper.sh "phase-name" "command-to-run"

set -e

PHASE_NAME="$1"
shift  # Remove first argument, rest are the command to run

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

BACKUP_ROOT="${HOME}/VulcanOS-backups"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
PHASE_BACKUP_DIR="${BACKUP_ROOT}/${TIMESTAMP}-before-${PHASE_NAME}"

print_info() {
    echo -e "${BLUE}[BACKUP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[BACKUP]${NC} $1"
}

print_error() {
    echo -e "${RED}[BACKUP]${NC} $1"
}

# Quick backup function (no prompts, automatic)
quick_backup() {
    print_info "Creating automatic backup before ${PHASE_NAME}..."
    
    mkdir -p "${PHASE_BACKUP_DIR}"/{config,local-bin,dotfiles,metadata}
    
    # Backup configs
    for dir in hypr waybar wofi swaync kitty alacritty nvim opencode themes vulcan; do
        if [[ -e "${HOME}/.config/${dir}" ]]; then
            cp -rL "${HOME}/.config/${dir}" "${PHASE_BACKUP_DIR}/config/" 2>/dev/null || true
        fi
    done
    
    # Backup scripts
    if [[ -d "${HOME}/.local/bin" ]]; then
        cp "${HOME}/.local/bin/vulcan-"* "${PHASE_BACKUP_DIR}/local-bin/" 2>/dev/null || true
        cp "${HOME}/.local/bin/opencode-"* "${PHASE_BACKUP_DIR}/local-bin/" 2>/dev/null || true
        cp "${HOME}/.local/bin/docker-mcp-"* "${PHASE_BACKUP_DIR}/local-bin/" 2>/dev/null || true
    fi
    
    # Backup dotfiles
    cp "${HOME}/.bashrc" "${PHASE_BACKUP_DIR}/dotfiles/" 2>/dev/null || true
    cp "${HOME}/.gitconfig" "${PHASE_BACKUP_DIR}/dotfiles/" 2>/dev/null || true
    
    # Save metadata
    cat > "${PHASE_BACKUP_DIR}/metadata/info.txt" << EOF
Phase: ${PHASE_NAME}
Timestamp: ${TIMESTAMP}
Date: $(date)
User: ${USER}
Hostname: $(hostname)
Command: $@
EOF
    
    # Create quick restore script
    cat > "${PHASE_BACKUP_DIR}/quick-restore.sh" << 'EOFSCRIPT'
#!/bin/bash
set -e
BACKUP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo "Restoring from: ${BACKUP_DIR}"
cp -r "${BACKUP_DIR}/config/"* ~/.config/ 2>/dev/null || true
cp "${BACKUP_DIR}/local-bin/"* ~/.local/bin/ 2>/dev/null || true
chmod +x ~/.local/bin/vulcan-* 2>/dev/null || true
cp "${BACKUP_DIR}/dotfiles/"* ~/ 2>/dev/null || true
echo "✅ Restore complete! Reload services as needed."
EOFSCRIPT
    chmod +x "${PHASE_BACKUP_DIR}/quick-restore.sh"
    
    print_success "Backup created: ${PHASE_BACKUP_DIR}"
    echo "${PHASE_BACKUP_DIR}" > /tmp/vulcan-last-backup.txt
}

# Create backup
quick_backup

# Run the actual command
print_info "Executing phase command..."
echo ""

if "$@"; then
    print_success "✅ Phase ${PHASE_NAME} completed successfully"
    exit 0
else
    EXIT_CODE=$?
    print_error "❌ Phase ${PHASE_NAME} failed with exit code ${EXIT_CODE}"
    echo ""
    print_error "ROLLBACK AVAILABLE:"
    print_error "  ${PHASE_BACKUP_DIR}/quick-restore.sh"
    exit $EXIT_CODE
fi
