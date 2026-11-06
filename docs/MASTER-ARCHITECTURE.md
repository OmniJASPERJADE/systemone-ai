# SYSTEM-ONE OS
## Master Architecture

### Operating System
- Independent Linux distribution
- AMD64 primary architecture
- antiX Linux development foundation
- APT / apt-get package manager
- Debian DEB package format
- glibc
- Limine bootloader
- EROFS + OverlayFS
- dinit PID 1
- System-One Health recovery

### LXNU
- Linux kernel 7.3.1+ target
- AMD Ryzen CPU support
- AMDGPU
- AMD XDNA / amdxdna
- Memory management
- Scheduling
- Kernel security

### OmniSoft
- omnid Rust supervisor
- dinit integration
- IPC
- System configuration
- Package integration
- Repository management
- Release engineering

### OmniRed
- Photon UI HDR
- Wayland
- Wayfire
- wlroots
- Mesa RADV
- Vulkan
- Display pipeline
- OmniRed Verify

### OmniSilicon
- AMD CPU
- Radeon RDNA 3 / 3.5 / 4
- XDNA / XDNA 2
- Firmware
- Hardware abstraction layer
- Device discovery

### OmniPerformance
- Omni Scheduler
- Omni Latency
- Omni Power
- PPW optimization
- OmniTheorem
- Telemetry
- Benchmarking

### Libraries
- OmniVec-II
- OmniUMA
- OmniColor96
- OCE
- QCX
- HDC-9
- PFXQI
- Omni Numeric Math
- PixelGen

### OmniAI
- vCIM
- CPU inference
- Radeon GPU inference
- XDNA NPU inference
- Cooperative dispatch
- Model management

### Applications
- OmniWorks
- OmniPages
- OmniNumbers
- OmniShow
- ArtCraft
- Omni Graph
- ProWave
- Symphony Audio
- Director's Cut
- OmniStudio

### Hardware
- Omni G1 LEO
- Omni G1 LEONARD
- Omni G1 TOTE
- Omni G1 LIBRE
- AIO-LYRA
- Omni G1 CUBE

### Development
- antiX / Debian
- GCC / Clang / LLVM
- Rust / Cargo
- CMake / Ninja / Make
- Git
- Debian packaging tools

### Engineering policy
- AMD-only hardware ecosystem
- No systemd
- dinit owns PID 1
- omnid provides higher-level orchestration
- APT and DEB are the package standards
- Every subsystem has an independent build and test pipeline
- No component is considered verified without passing tests

### Implementation status
Specified
In Development
Implemented
Verified
Production
