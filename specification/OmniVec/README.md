# OmniVec-II
## Heterogeneous Compute Architecture

### CPU
- AMD Zen 4
- x86-64-v3 baseline
- AVX2
- FMA3
- Runtime ISA dispatch

### GPU
- AMD Radeon RDNA
- AMDGPU
- ROCm where supported
- Vulkan compute
- HIP where supported

### NPU
- AMD XDNA
- amdxdna driver
- XRT runtime
- NPU compiler/toolchain
- NPU workload scheduling

### Compute Engines
- Omni Numeric Math Engine
- Omni Compression Engine
- PixelGen
- OmniColor96
- PFXQI

### Dispatch
- CPU execution
- GPU execution
- NPU execution
- Cooperative scheduling
- Workload-specific backend selection

### Engineering
- Capability detection
- Memory-aware dispatch
- CPU fallback
- Performance profiling
- Reproducible benchmarks

### Status
Architecture: Defined
Implementation: In progress
Verification: Pending
