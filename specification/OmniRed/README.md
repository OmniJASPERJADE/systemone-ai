# OmniRed
## System-One Graphics Architecture

### Platform
- AMD64
- AMD Zen 4 minimum
- Radeon RDNA graphics
- Mesa RADV
- Vulkan

### Graphics Stack
- AMDGPU kernel driver
- DRM/KMS
- Mesa
- RADV Vulkan driver
- LLVM shader compiler
- Wayland
- wlroots
- Wayfire compositor

### Graphics Features
- RDNA 3.5 support
- RDNA 4 support
- Vulkan acceleration
- FSR 1-4 integration targets
- HDR display pipeline
- Variable refresh rate
- Multi-display support

### Memory
- AMD UMA support
- Shared CPU/GPU memory
- DMA-BUF
- Explicit synchronization
- GPU memory budgeting

### Photon Integration
- HDR color management
- Vulkan presentation
- Display calibration
- Low-latency composition

### Engineering Requirements
- No Nvidia dependency
- No Intel dependency
- Hardware-specific validation
- Reproducible graphics benchmarks

### Status
Architecture: Defined
Implementation: In progress
Verification: Pending
