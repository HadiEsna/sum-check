# Sum-check

Welcome! This project aims to implement and experiment with the sum-check protocol as explained in the [Zero Knowledge Book](https://www.zero-knowledge-book.com/).

## Overview

This is a Rust implementation of the sum-check protocol, which is a fundamental building block in various zero-knowledge proof systems. The project is structured into several key components:

- **Equation Module**: Handles mathematical equations and terms required for the protocol
- **Sum Check Module**: Contains the core implementation of the sum-check protocol
  - Interactive prover implementation
  - Verifier implementation
- **GKR Module**: Work in progress implementation of the GKR protocol (Goldwasser-Kalai-Rothblum)

## Technical Details

- **Language**: Rust (2021 edition)
- **Dependencies**:
  - `num` (0.4) - For advanced numerical operations
  - `rand` (0.8) - For cryptographic randomness

## Project Structure

```
src/
├── equation/       # Equation handling implementation
├── gkr/           # GKR protocol implementation
├── sum_check/     # Core sum-check protocol
    ├── prover.rs
    ├── verifier.rs
    └── proving_process_interactive.rs
```

## Getting Started

To use this library, add it to your `Cargo.toml`:

```toml
[dependencies]
sum_check = { git = "https://github.com/HadiEsna/sum-check" }
```

## Roadmap

- [x] implementing needed libraries (equation)
- [x] implementing sum-check protocol
- [ ] implementing gkr
- [ ] implementing non-interactive version of the prover
