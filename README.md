# Rust Agent Commerce Kit

**Production-grade commerce infrastructure for paid MCP tools and machine APIs.**

RACK focuses on the hard production guarantees around machine-native payments: canonical resource/payment binding, replay-safe execution, durable once-only authority, settlement ambiguity and recovery, x402/MPP semantic separation, raw-wire interoperability proof, and reproducible qualification evidence.

## What the paid product proves

- x402 paid MCP raw-wire conformance;
- x402 Base and Solana support behind protocol-neutral durable authority;
- MPP Tempo charge without forcing false x402 semantics;
- canonical route/resource security;
- replay/idempotency convergence;
- fenced execution and lost-response recovery;
- result withholding on uncertain/failed settlement;
- operator reconciliation and diagnostics;
- evidence-bearing compatibility and upgrade qualification.

## Not just framework wiring

Rust, Axum, PostgreSQL, MCP, x402, and MPP are implementation components—not the commercial moat. The product value is the verified behavior when retries, failures, concurrency, hostile inputs, payments, and upgrades collide.

See `docs/architecture.md`, `docs/security-model.md`, `docs/compatibility.md`, `docs/evidence.md`, and `docs/test-matrix.md`.

The complete commercial implementation, replay/execution/reconciliation engines, protocol adapters, and adversarial harness remain private.
