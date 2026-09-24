# RACK Engineering Showcase

**Production-grade machine-payment infrastructure for paid MCP tools and machine-facing APIs.**

This is a curated public engineering showcase for **Rust Agent Commerce Kit (RACK)**. The commercial implementation remains private and authoritative; this repository is not a source mirror.

## What RACK is designed to solve

RACK focuses on the hard parts around machine-native commerce rather than basic HTTP `402` handling:

- protocol-neutral durable commerce authority;
- MCP/HTTP convergence;
- exact money, asset, recipient, and network validation;
- replay and idempotency boundaries;
- fenced execution and durable results;
- settlement ambiguity and proof-driven reconciliation;
- x402 and MPP lifecycle differences;
- Base and Solana payment profiles;
- operator diagnostics and structured observability;
- adversarial/concurrency testing and dependency requalification.

## Current private implementation status

The audited snapshot used for this showcase has **Phase 0–6 frozen** and **Phase 7 (reconciliation, operations, and observability) in active development**. This public repository intentionally lags private development when disclosure review has not been completed.

## Architecture

```text
MCP / HTTP
    |
    v
Canonical Resource / Commerce Request
    |
    v
Protocol-neutral durable authority
    |
    +--> x402 (Base / Solana)
    |
    +--> MPP charge lifecycle
    |
    v
Replay + idempotency + payment binding
    |
    v
Fenced execution authority
    |
    v
Durable result / settlement
    |
    v
Reconciliation + operator observability
```

## Public example

`examples/domain-demo` is an independently written, dependency-free demonstration of exact atomic money representation, replay-key uniqueness, and reconciliation outcomes. It is intentionally smaller than production RACK and is not production payment code.

```bash
cargo test --manifest-path examples/domain-demo/Cargo.toml
```
