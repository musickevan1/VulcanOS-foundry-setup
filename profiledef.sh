#!/usr/bin/env bash
# NewOS archiso profile configuration
# Build profile definition for custom Arch Linux distribution on T2 MacBook Pro

iso_name="newos"
iso_label="NEWOS_$(date +%Y%m)"
iso_publisher="NewOS Project"
iso_application="NewOS Live/Install Medium"
iso_version="$(date +%Y.%m.%d)"
install_dir="arch"
buildmodes=('iso')
bootmodes=('bios' 'uefi')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-Xcompression-level' '9' '-Xbcj' 'x86')

# File permissions array
# Format: [source_path]="uid:gid:mode"
declare -a file_permissions=(
	["/etc/shadow"]="0:0:400"
	["/etc/gshadow"]="0:0:400"
	["/root"]="0:0:750"
	["/etc/sudoers.d/wheel"]="0:0:440"
	["/usr/local/bin/"]="0:0:755"
)
