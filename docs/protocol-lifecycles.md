# Protocol lifecycle distinction

RACK does not pretend x402 and MPP share one settlement lifecycle.

## x402 profile

```text
verify
-> fenced execution
-> durable result
-> settle/finalize
-> result or receipt release
```

## MPP charge profile

```text
credential validation
-> payment completion
-> durable completion artifact
-> settled payment authority
-> fenced execution
-> durable result
-> response or receipt
```

The public showcase documents this semantic separation without publishing private adapter or facilitator implementation details.
