# Security and correctness invariants

- Money is represented in exact atomic units; floating-point values do not define payment authority.
- Raw transport paths, MCP metadata, and protocol wire payloads are untrusted representations, not authority by themselves.
- Replay identity and client idempotency identity are distinct concerns.
- A stale execution generation cannot commit after a newer authority takes over.
- Ambiguous external side effects are reconciled; they are not blindly re-executed.
- x402 settlement ambiguity must not trigger business execution again.
- MPP completion and execution recovery must not cause a second payment completion.
- Client disconnects are transport events, not proof of payment or execution failure.
- Telemetry must not contain private keys, authorization secrets, signed payment credentials, or unbounded attacker-controlled labels.
