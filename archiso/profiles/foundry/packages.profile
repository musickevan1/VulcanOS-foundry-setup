# ============================================================================
# VulcanOS Foundry Profile - AI Workstation specific packages
# ============================================================================

# ----------------------------------------------------------------------------
# GENERIC KERNEL
# ----------------------------------------------------------------------------
linux
linux-headers

# ----------------------------------------------------------------------------
# NVIDIA GPU DRIVERS (RTX 5070 Ti - Blackwell)
# Note: Uses nvidia-open-dkms for RTX 50 series (sm_120)
# Standard nvidia-dkms does NOT support Blackwell architecture
# ----------------------------------------------------------------------------
nvidia-open-dkms
nvidia-utils
nvidia-settings
lib32-nvidia-utils

# ----------------------------------------------------------------------------
# CUDA/ML STACK
# Note: CUDA 12.8+ required for Blackwell (sm_120) support
# PyTorch stable does NOT support sm_120 - must use nightly build (post-install)
# ----------------------------------------------------------------------------
cuda
cudnn
nvidia-container-toolkit
