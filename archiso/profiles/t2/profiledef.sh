#!/usr/bin/env bash
# shellcheck disable=SC2034

# =============================================================================
# VulcanOS T2 Profile - archiso Profile Definition
# T2 MacBook Pro Distribution
# =============================================================================

iso_name="vulcanos-t2"
iso_label="VULCAN_T2_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="VulcanOS Project <https://github.com/yourusername/vulcanos>"
iso_application="VulcanOS T2 MacBook Pro Live/Install Medium"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="arch"
buildmodes=('iso')
bootmodes=(
    'bios.syslinux.mbr'
    'bios.syslinux.eltorito'
    'uefi-ia32.grub.esp'
    'uefi-x64.grub.esp'
    'uefi-ia32.grub.eltorito'
    'uefi-x64.grub.eltorito'
)
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--long' '-19')
file_permissions=(
    ["/etc/shadow"]="0:0:400"
    ["/etc/gshadow"]="0:0:400"
    ["/root"]="0:0:750"
    ["/etc/sudoers.d/wheel"]="0:0:440"
    ["/usr/local/bin/"]="0:0:755"
)
