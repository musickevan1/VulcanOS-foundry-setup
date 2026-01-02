#!/bin/bash
# VulcanOS Configuration Backup Script
# Creates timestamped backups of all critical configuration files
# Usage: backup-vulcan-config.sh [backup-name]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
BACKUP_ROOT="${HOME}/VulcanOS-backups"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
BACKUP_NAME="${1:-auto-backup}"
BACKUP_DIR="${BACKUP_ROOT}/${TIMESTAMP}-${BACKUP_NAME}"

# What to backup
CONFIG_DIRS=(
    "hypr"
    "waybar"
    "wofi"
    "swaync"
    "kitty"
    "alacritty"
    "nvim"
    "opencode"
    "themes"
    "vulcan"
)

LOCAL_BIN_SCRIPTS=(
    "vulcan-*"
    "opencode-*"
    "docker-mcp-*"
)

DOTFILES=(
    ".bashrc"
    ".gitconfig"
)

# Helper functions
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Create backup directory structure
create_backup_dir() {
    print_info "Creating backup directory: ${BACKUP_DIR}"
    mkdir -p "${BACKUP_DIR}"/{config,local-bin,dotfiles,metadata}
}

# Backup .config directory items
backup_configs() {
    print_info "Backing up configuration directories..."
    
    for dir in "${CONFIG_DIRS[@]}"; do
        local src="${HOME}/.config/${dir}"
        local dest="${BACKUP_DIR}/config/${dir}"
        
        if [[ -e "$src" ]]; then
            # Check if symlink
            if [[ -L "$src" ]]; then
                local target=$(readlink -f "$src")
                print_info "  → ${dir} (symlink to ${target})"
                
                # Save symlink info
                echo "${dir}|symlink|${target}" >> "${BACKUP_DIR}/metadata/symlinks.txt"
                
                # Also backup the actual files
                cp -rL "$src" "$dest"
            else
                print_info "  → ${dir} (directory)"
                echo "${dir}|directory|" >> "${BACKUP_DIR}/metadata/symlinks.txt"
                cp -r "$src" "$dest"
            fi
        else
            print_warning "  ⚠ ${dir} not found, skipping"
        fi
    done
}

# Backup scripts from .local/bin
backup_scripts() {
    print_info "Backing up scripts from ~/.local/bin..."
    
    mkdir -p "${BACKUP_DIR}/local-bin"
    
    for pattern in "${LOCAL_BIN_SCRIPTS[@]}"; do
        # Use nullglob to handle no matches
        shopt -s nullglob
        local files=("${HOME}/.local/bin/${pattern}")
        shopt -u nullglob
        
        if [[ ${#files[@]} -gt 0 ]]; then
            for file in "${files[@]}"; do
                if [[ -f "$file" ]]; then
                    local basename=$(basename "$file")
                    
                    # Check if symlink
                    if [[ -L "$file" ]]; then
                        local target=$(readlink -f "$file")
                        print_info "  → ${basename} (symlink to ${target})"
                        echo "${basename}|symlink|${target}" >> "${BACKUP_DIR}/metadata/bin-symlinks.txt"
                        
                        # Copy actual file
                        cp -L "$file" "${BACKUP_DIR}/local-bin/${basename}"
                    else
                        print_info "  → ${basename}"
                        cp "$file" "${BACKUP_DIR}/local-bin/${basename}"
                    fi
                    
                    # Preserve executable bit
                    chmod +x "${BACKUP_DIR}/local-bin/${basename}"
                fi
            done
        fi
    done
}

# Backup dotfiles
backup_dotfiles() {
    print_info "Backing up home directory dotfiles..."
    
    for dotfile in "${DOTFILES[@]}"; do
        local src="${HOME}/${dotfile}"
        
        if [[ -f "$src" ]]; then
            print_info "  → ${dotfile}"
            cp "$src" "${BACKUP_DIR}/dotfiles/${dotfile}"
        else
            print_warning "  ⚠ ${dotfile} not found, skipping"
        fi
    done
}

# Save metadata
save_metadata() {
    print_info "Saving system metadata..."
    
    local meta_file="${BACKUP_DIR}/metadata/backup-info.txt"
    
    cat > "$meta_file" << EOF
Backup Information
==================
Timestamp: ${TIMESTAMP}
Backup Name: ${BACKUP_NAME}
User: ${USER}
Hostname: $(hostname)
System: $(uname -a)
Date: $(date)

VulcanOS Version
================
$(cat /home/evan/VulcanOS/VERSION 2>/dev/null || echo "VERSION file not found")

Package Information
===================
Hyprland: $(hyprctl version | head -1 2>/dev/null || echo "Not installed")
Waybar: $(waybar --version 2>/dev/null || echo "Not installed")

Directory Structure
===================
$(tree -L 2 "${HOME}/.config" 2>/dev/null || echo "tree not installed")

Installed Vulcan Scripts
=========================
$(ls -la ~/.local/bin/vulcan-* 2>/dev/null || echo "No vulcan scripts found")
EOF

    # Save list of all backed up files
    print_info "Creating file inventory..."
    find "${BACKUP_DIR}" -type f > "${BACKUP_DIR}/metadata/file-inventory.txt"
}

# Create restore script
create_restore_script() {
    print_info "Creating restore script..."
    
    local restore_script="${BACKUP_DIR}/restore.sh"
    
    cat > "$restore_script" << 'EOFSCRIPT'
#!/bin/bash
# Auto-generated restore script
# Created: TIMESTAMP_PLACEHOLDER

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

BACKUP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${YELLOW}WARNING: This will restore configurations from backup${NC}"
echo "Backup directory: ${BACKUP_DIR}"
echo ""
echo "This will:"
echo "  - Restore .config directories"
echo "  - Restore .local/bin scripts"
echo "  - Restore dotfiles"
echo ""
read -p "Continue? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Restore cancelled"
    exit 1
fi

# Create new backup before restoring
echo -e "${GREEN}Creating safety backup before restore...${NC}"
SAFETY_BACKUP="${HOME}/VulcanOS-backups/$(date +%Y%m%d-%H%M%S)-pre-restore"
mkdir -p "${SAFETY_BACKUP}"
cp -r ~/.config "${SAFETY_BACKUP}/" 2>/dev/null || true
cp -r ~/.local/bin "${SAFETY_BACKUP}/" 2>/dev/null || true
echo "Safety backup created at: ${SAFETY_BACKUP}"

# Restore configs
echo -e "${GREEN}Restoring configuration directories...${NC}"
for dir in "${BACKUP_DIR}"/config/*; do
    if [[ -d "$dir" ]]; then
        dirname=$(basename "$dir")
        echo "  → Restoring ${dirname}"
        
        # Remove existing (whether file, dir, or symlink)
        rm -rf "${HOME}/.config/${dirname}"
        
        # Copy from backup
        cp -r "$dir" "${HOME}/.config/${dirname}"
    fi
done

# Restore scripts
echo -e "${GREEN}Restoring scripts...${NC}"
if [[ -d "${BACKUP_DIR}/local-bin" ]]; then
    for script in "${BACKUP_DIR}"/local-bin/*; do
        if [[ -f "$script" ]]; then
            scriptname=$(basename "$script")
            echo "  → Restoring ${scriptname}"
            cp "$script" "${HOME}/.local/bin/${scriptname}"
            chmod +x "${HOME}/.local/bin/${scriptname}"
        fi
    done
fi

# Restore dotfiles
echo -e "${GREEN}Restoring dotfiles...${NC}"
if [[ -d "${BACKUP_DIR}/dotfiles" ]]; then
    for dotfile in "${BACKUP_DIR}"/dotfiles/.*; do
        if [[ -f "$dotfile" ]]; then
            filename=$(basename "$dotfile")
            echo "  → Restoring ${filename}"
            cp "$dotfile" "${HOME}/${filename}"
        fi
    done
fi

echo ""
echo -e "${GREEN}✅ Restore complete!${NC}"
echo ""
echo "Safety backup preserved at: ${SAFETY_BACKUP}"
echo ""
echo "You may need to:"
echo "  - Reload Hyprland: hyprctl reload"
echo "  - Restart Waybar: pkill waybar && waybar &"
echo "  - Restart other services as needed"
EOFSCRIPT

    # Replace timestamp placeholder
    sed -i "s/TIMESTAMP_PLACEHOLDER/${TIMESTAMP}/" "$restore_script"
    
    chmod +x "$restore_script"
}

# Compress backup (optional)
compress_backup() {
    if command -v tar &> /dev/null; then
        print_info "Compressing backup..."
        
        local archive="${BACKUP_DIR}.tar.gz"
        tar -czf "$archive" -C "${BACKUP_ROOT}" "$(basename "${BACKUP_DIR}")"
        
        print_success "Compressed backup: ${archive}"
        print_info "Size: $(du -h "${archive}" | cut -f1)"
    else
        print_warning "tar not available, skipping compression"
    fi
}

# Cleanup old backups
cleanup_old_backups() {
    local keep_count=10
    
    print_info "Cleaning up old backups (keeping ${keep_count} most recent)..."
    
    # List all backup directories, sort by date, skip newest N
    local old_backups=$(find "${BACKUP_ROOT}" -maxdepth 1 -type d -name "20*" | sort -r | tail -n +$((keep_count + 1)))
    
    if [[ -n "$old_backups" ]]; then
        echo "$old_backups" | while read -r old_backup; do
            print_info "  → Removing old backup: $(basename "$old_backup")"
            rm -rf "$old_backup"
            
            # Also remove compressed version if exists
            [[ -f "${old_backup}.tar.gz" ]] && rm -f "${old_backup}.tar.gz"
        done
    else
        print_info "No old backups to clean up"
    fi
}

# Main execution
main() {
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║         VulcanOS Configuration Backup Utility              ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    
    create_backup_dir
    backup_configs
    backup_scripts
    backup_dotfiles
    save_metadata
    create_restore_script
    
    echo ""
    print_success "Backup completed successfully!"
    echo ""
    echo "Backup location: ${BACKUP_DIR}"
    echo ""
    print_info "Backup contents:"
    du -sh "${BACKUP_DIR}"/{config,local-bin,dotfiles,metadata} 2>/dev/null | sed 's/^/  /'
    echo ""
    print_info "To restore this backup:"
    echo "  ${BACKUP_DIR}/restore.sh"
    echo ""
    
    # Optional: compress
    read -p "Compress this backup? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        compress_backup
    fi
    
    # Optional: cleanup
    read -p "Clean up old backups? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cleanup_old_backups
    fi
    
    echo ""
    print_success "All done! 🎉"
}

# Run main
main
