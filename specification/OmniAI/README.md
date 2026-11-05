# OmniAI
## AMD Heterogeneous AI Architecture

### 1. Hardware Platform
- AMD64
- AMD Zen 4 minimum
- AMD Ryzen AI processors
- AMD XDNA NPU
- AMD Radeon RDNA GPU

### 2. NPU Integration
- AMD XDNA1 support
- amdxdna Linux kernel driver
- Compatible firmware
- XRT userspace runtime
- NPU compiler/toolchain
- Hardware capability detection

### 3. Compute Backends
- XDNA NPU
- Radeon GPU
- Zen 4 CPU
- Cooperative execution

### 4. Runtime Architecture
- OmniAI Runtime
- vCIM compute interface
- Workload scheduler
- Tensor allocation
- Model execution
- Backend selection
- Performance monitoring

### 5. AI Workloads
- Matrix multiplication
- Neural network inference
- Image processing
- Computer vision
- Transformer inference
- Quantized models

### 6. Memory
- OmniUMA integration
- Shared memory accounting
- DMA-BUF where supported
- Zero-copy where supported
- Tensor memory management

### 7. Fallback
- NPU unavailable: GPU fallback
- GPU unavailable: CPU fallback
- Unsupported operators: compatible backend
- No silent loss of correctness

### 8. Ryzen AI 7 8845HS
- Zen 4 CPU
- Radeon 780M RDNA 3 iGPU
- XDNA1 NPU
- Advertised NPU peak: 16 TOPS

NPU acceleration requires compatible
firmware, drivers, runtime and operators.

### 9. Verification
- NPU device detection
- Driver initialization
- Runtime initialization
- Model compilation
- Inference execution
- CPU/GPU/NPU comparison
- Power and latency measurements

### Status
Architecture: Defined
Implementation: In progress
Hardware qualification: Pending
Verification: Pending
