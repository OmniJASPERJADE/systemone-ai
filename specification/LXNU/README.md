# LXNU
## System-One Kernel Architecture

### Platform
- AMD64 only
- AMD Zen 4 minimum
- Linux-derived kernel
- Linux 6.18 LTS baseline
- Limine bootloader

### Architecture
- XNU-inspired design patterns
- Native Linux kernel compatibility
- AMD CPU scheduling
- AMD RDNA graphics support
- AMD XDNA NPU support

### Required Subsystems
- AMDGPU
- DRM/KMS
- KFD
- amdxdna
- IOMMU
- ACPI
- CPU power management
- Memory management
- Security framework

### XDNA Requirement
XDNA must function on Ryzen 7 8845HS.
Required:
- amdxdna driver
- Compatible signed firmware
- XRT userspace
- XDNA1 compiler/toolchain
- NPU execution tests

### Boot
- Limine
- EROFS system image
- Initramfs
- A/B recovery support

### Status
Architecture: Defined
Implementation: In progress
Verification: Pending
