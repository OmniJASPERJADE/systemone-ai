# OmniVec-II
## Heterogeneous Compute Architecture
Organization: OmniSoft

### 1. Architecture

OmniVec-II provides unified compute orchestration across:

- AMD Ryzen CPU
- AMD Radeon RDNA GPU
- AMD XDNA NPU where supported
- OmniUMA memory management

### 2. CPU Vector Engine

Baseline:
- x86-64-v3
- SSE4.2
- AVX2
- FMA

Optional:
- x86-64-v4
- AVX-512

Requirements:
- Runtime ISA detection
- OS extended-state verification
- Optimized kernel dispatch
- Portable fallback

### 3. GPU Engine

- AMD Radeon
- RDNA architecture
- Vulkan compute
- Mesa RADV
- ROCm/HIP where supported
- GPU workload scheduling

### 4. NPU Engine

- AMD XDNA where supported
- OmniAI runtime
- vCIM interface
- NPU workload dispatch
- Cooperative scheduling

### 5. OmniUMA

OmniUMA provides:

- Unified allocation interface
- CPU memory management
- GPU memory management
- NPU memory management
- Buffer tracking
- Memory synchronization
- Allocation policies
- Memory pressure management

Physical memory sharing and coherency depend on
the capabilities of each hardware platform.

### 6. Möbius Fabric

OmniVec-II Möbius Fabric coordinates:

- Numeric Math Engine
- Omni Compression Engine
- PixelGen
- OmniColor96
- Memory orchestration
- Heterogeneous compute dispatch

### 7. Runtime Dispatch

1. Detect hardware capabilities.
2. Identify available compute backends.
3. Select optimized implementation.
4. Allocate required memory.
5. Dispatch workload.
6. Synchronize results.
7. Measure execution performance.

### 8. Qualification

- CPU vector benchmarks
- GPU compute benchmarks
- NPU inference benchmarks
- Memory bandwidth benchmarks
- Synchronization overhead
- Cross-backend correctness
- Thermal and power measurements

### 9. Engineering Status

Architecture: Defined
Implementation: In progress
Verification: Pending
Performance qualification: Pending
