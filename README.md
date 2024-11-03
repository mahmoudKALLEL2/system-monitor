# System Monitor

A system monitoring tool that performs stress tests on the CPU and memory.

## Commands

- `cpu`: Runs a stress test on the CPU.
- `memory`: Runs a stress test on the memory.

## Usage

```bash
cargo run cpu --duration 10 --threshold 80
cargo run memory --duration 10
