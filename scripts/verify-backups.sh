#!/bin/bash
# VulcanOS Backup Verification Script
# Checks integrity and completeness of all backups

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

BACKUP_ROOT="${HOME}/VulcanOS-backups"

print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_ok() {
    echo -e "  ${GREEN}✓${NC} $1"
}

print_warn() {
    echo -e "  ${YELLOW}⚠${NC} $1"
}

print_fail() {
    echo -e "  ${RED}✗${NC} $1"
}

print_info() {
    echo -e "  ${BLUE}ℹ${NC} $1"
}

# Check if backup directory exists
check_backup_dir() {
    print_header "Backup Directory Status"
    
    if [[ -d "$BACKUP_ROOT" ]]; then
        print_ok "Backup directory exists: $BACKUP_ROOT"
        
        local count=$(find "$BACKUP_ROOT" -maxdepth 1 -type d -name "20*" | wc -l)
        print_info "Total backups found: $count"
        
        local size=$(du -sh "$BACKUP_ROOT" 2>/dev/null | cut -f1)
        print_info "Total size: $size"
        
        echo ""
        return 0
    else
        print_fail "Backup directory not found: $BACKUP_ROOT"
        echo ""
        return 1
    fi
}

# Verify a single backup
verify_backup() {
    local backup_dir="$1"
    local backup_name=$(basename "$backup_dir")
    
    local issues=0
    
    echo -e "${YELLOW}Checking${NC} $backup_name"
    
    # Check structure
    if [[ -d "${backup_dir}/config" ]]; then
        local config_count=$(find "${backup_dir}/config" -type d -mindepth 1 -maxdepth 1 | wc -l)
        if [[ $config_count -gt 0 ]]; then
            print_ok "Config backups ($config_count dirs)"
        else
            print_warn "Config dir exists but is empty"
            ((issues++))
        fi
    else
        print_fail "Missing config directory"
        ((issues++))
    fi
    
    if [[ -d "${backup_dir}/local-bin" ]]; then
        local script_count=$(find "${backup_dir}/local-bin" -type f | wc -l)
        if [[ $script_count -gt 0 ]]; then
            print_ok "Scripts backed up ($script_count files)"
        else
            print_warn "Scripts dir exists but is empty"
            ((issues++))
        fi
    else
        print_warn "Missing local-bin directory"
        ((issues++))
    fi
    
    if [[ -d "${backup_dir}/metadata" ]]; then
        print_ok "Metadata directory exists"
        
        if [[ -f "${backup_dir}/metadata/backup-info.txt" ]] || [[ -f "${backup_dir}/metadata/info.txt" ]]; then
            print_ok "Backup info file present"
        else
            print_warn "Missing backup info file"
            ((issues++))
        fi
    else
        print_warn "Missing metadata directory"
        ((issues++))
    fi
    
    # Check restore script
    if [[ -f "${backup_dir}/restore.sh" ]]; then
        if [[ -x "${backup_dir}/restore.sh" ]]; then
            print_ok "Full restore script (executable)"
        else
            print_warn "Restore script exists but not executable"
            ((issues++))
        fi
    elif [[ -f "${backup_dir}/quick-restore.sh" ]]; then
        if [[ -x "${backup_dir}/quick-restore.sh" ]]; then
            print_ok "Quick restore script (executable)"
        else
            print_warn "Quick restore script exists but not executable"
            ((issues++))
        fi
    else
        print_fail "No restore script found"
        ((issues++))
    fi
    
    # Size check
    local backup_size=$(du -sh "$backup_dir" 2>/dev/null | cut -f1)
    print_info "Size: $backup_size"
    
    # Age check
    local backup_date=$(echo "$backup_name" | grep -oE "[0-9]{8}" | head -1)
    if [[ -n "$backup_date" ]]; then
        local age_days=$(( ($(date +%s) - $(date -d "${backup_date:0:4}-${backup_date:4:2}-${backup_date:6:2}" +%s)) / 86400 ))
        print_info "Age: $age_days days"
        
        if [[ $age_days -gt 30 ]]; then
            print_warn "Backup is older than 30 days - consider cleanup"
        fi
    fi
    
    echo ""
    return $issues
}

# Main verification
main() {
    print_header "VulcanOS Backup Verification Utility"
    echo ""
    
    check_backup_dir || exit 1
    
    # Get all backups
    local backups=($(find "$BACKUP_ROOT" -maxdepth 1 -type d -name "20*" | sort -r))
    
    if [[ ${#backups[@]} -eq 0 ]]; then
        print_warn "No backups found"
        echo ""
        echo "Create your first backup with:"
        echo "  /home/evan/VulcanOS/scripts/backup-vulcan-config.sh \"first-backup\""
        exit 0
    fi
    
    print_header "Individual Backup Verification"
    echo ""
    
    local total_issues=0
    local backup_count=0
    
    for backup in "${backups[@]}"; do
        verify_backup "$backup"
        local status=$?
        ((total_issues += status))
        ((backup_count++))
    done
    
    print_header "Summary"
    echo ""
    echo "  Total backups checked: $backup_count"
    echo "  Total issues found: $total_issues"
    echo ""
    
    if [[ $total_issues -eq 0 ]]; then
        print_ok "All backups verified successfully!"
    elif [[ $total_issues -lt 5 ]]; then
        print_warn "Minor issues found - backups should still be usable"
    else
        print_fail "Significant issues found - recommend creating fresh backup"
    fi
    
    echo ""
    
    # Check last backup
    print_header "Last Backup Status"
    echo ""
    
    if [[ -f /tmp/vulcan-last-backup.txt ]]; then
        local last_backup=$(cat /tmp/vulcan-last-backup.txt)
        if [[ -d "$last_backup" ]]; then
            print_ok "Last backup tracked: $(basename "$last_backup")"
            print_info "Restore with: ${last_backup}/quick-restore.sh"
        else
            print_warn "Last backup reference exists but directory not found"
            print_info "Path: $last_backup"
        fi
    else
        print_info "No last backup tracked (no orchestrator runs yet)"
    fi
    
    echo ""
    
    # Recommendations
    print_header "Recommendations"
    echo ""
    
    local latest_backup=$(ls -t "$BACKUP_ROOT" | head -1)
    local latest_date=$(echo "$latest_backup" | grep -oE "[0-9]{8}" | head -1)
    
    if [[ -n "$latest_date" ]]; then
        local days_since=$(( ($(date +%s) - $(date -d "${latest_date:0:4}-${latest_date:4:2}-${latest_date:6:2}" +%s)) / 86400 ))
        
        if [[ $days_since -gt 7 ]]; then
            print_warn "Last backup is $days_since days old"
            echo "      Consider creating a fresh backup:"
            echo "      /home/evan/VulcanOS/scripts/backup-vulcan-config.sh \"weekly\""
        else
            print_ok "Recent backup exists (${days_since} days old)"
        fi
    fi
    
    # Check disk space
    local home_free=$(df -h ~ | tail -1 | awk '{print $4}')
    print_info "Available disk space: $home_free"
    
    # Backup count recommendation
    if [[ $backup_count -gt 15 ]]; then
        print_warn "$backup_count backups stored - consider cleanup to save space"
        echo "      Keep 10 most recent, remove old ones manually"
    fi
    
    echo ""
    print_header "✅ Verification Complete"
    echo ""
}

main "$@"
