# System-One Implementation Status

## Implemented source
- OmniSoft: initial Rust service supervisor
- OmniSilicon: AMD PCI and accelerator discovery
- OmniVec-II: C++20 scalar vector operations
- GitHub Actions: Rust and C++ build and test pipeline

## Specified, not yet implemented
- LXNU kernel integration and AMD driver configuration
- OmniRed graphics and Photon desktop
- OmniPerformance scheduling and power management
- OmniUMA unified memory
- OmniColor96
- OCE compression
- QCX and HDC-9
- PFXQI optimization
- OmniAI and vCIM
- System-One installer and recovery

## OS baseline
- antiX/Debian development foundation
- APT and DEB packages
- dinit PID 1
- AMD64 primary target
- Limine bootloader
- EROFS and OverlayFS target

No subsystem is considered verified until its automated tests pass.
