<div align="center">
<br/>

# hypervisor-engine

### Type-1 hypervisor for Sorabbyngo OS — VM lifecycle, isolation, and resource partitioning.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/hypervisor-engine)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/hypervisor-engine)

</div>

---

## What It Does

hypervisor-engine provides lightweight VM isolation on Sorabbyngo OS. It partitions hardware resources between the host OS, AI agent containers (Byngox), and user workloads — ensuring security boundaries without full virtualisation overhead.

---

## Features

| Feature | Description |
|---|---|
| **VM Lifecycle** | Create, start, pause, snapshot, destroy VMs |
| **Resource Partitioning** | CPU, memory, and I/O quotas per VM |
| **Memory Isolation** | EPT/NPT-based memory isolation between VMs |
| **Device Passthrough** | PCIe passthrough for GPU, NIC, storage |
| **Byngox Integration** | Agent containers run in isolated VM partitions |
| **Snapshot & Restore** | Live VM snapshots for fast workload migration |
| **Sentinel Integration** | All VM exits monitored by sentinel-runtime |

---

## Architecture

```
hypervisor-engine/
├── src/
│   ├── vm/           # VM lifecycle management
│   ├── memory/       # EPT/NPT memory isolation
│   ├── cpu/          # vCPU management and scheduling
│   ├── devices/      # Virtio devices, PCIe passthrough
│   ├── snapshot/     # VM snapshot and restore
│   └── kvm/          # KVM ioctl interface layer
├── Cargo.toml
└── README.md
```

---

## Dependencies

- Linux KVM (kernel virtualisation module)
- `sorabbyngo-os/sentinel-runtime` — exit monitoring
- `sorabbyngo-os/identity-core` — VM identity

---

## Stack

- **Language:** Rust
- **Hypervisor interface:** KVM (ioctl)
- **OS target:** Sorabbyngo OS / server-os

---

*Sorabbyngo OS · Kisii, Kenya · 2026*
