# Architecture

RACK keeps commerce authority independent from transport and protocol SDKs. HTTP and MCP are ingress/representation surfaces; payment protocol adapters are not allowed to become the domain authority.

The core design separates request identity, payment attempt identity, replay identity, logical execution identity, execution-attempt generation, settlement identity, receipt identity, and reconciliation identity. This avoids collapsing operationally different retries into one ambiguous identifier.

Phase 6 established fenced durable execution and proof-driven recovery. Phase 7 operates that authority with reconciliation queries, bounded scheduling, operator workflows, and low-cardinality observability rather than introducing a second state machine.
