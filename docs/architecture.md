# Architecture

```text
MCP / HTTP ingress
      |
      v
canonical ResourceId resolution
      |
      v
commerce policy
      |
  +---+---+
  |       |
x402     MPP
  |       |
  +---+---+
      |
      v
durable payment authority
      |
      v
fenced execution / durable result
      |
      v
receipt + reconciliation
```

Transport representations are interoperable wire contracts; PostgreSQL-backed canonical identities remain durable commerce authority.
