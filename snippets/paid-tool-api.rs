// Illustrative public API shape. Internal payment/replay/reconciliation implementation is private.
app.add_paid_mcp_tool(
    "wallet_analysis",
    CommercePolicy::named("premium-analysis"),
    wallet_analysis,
);
