use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicMoney {
    pub amount: u128,
    pub asset: &'static str,
}

impl AtomicMoney {
    pub fn new(amount: u128, asset: &'static str) -> Result<Self, &'static str> {
        if asset.trim().is_empty() { return Err("asset is required"); }
        Ok(Self { amount, asset })
    }
}

#[derive(Default)]
pub struct ReplayGuard {
    seen: BTreeSet<String>,
}

impl ReplayGuard {
    pub fn claim(&mut self, replay_key: impl Into<String>) -> Result<(), &'static str> {
        if self.seen.insert(replay_key.into()) { Ok(()) } else { Err("replay") }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationOutcome {
    AuthoritativeSuccess,
    AuthoritativeFailure,
    StillUnknown,
    ProvedNotApplied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_uses_exact_atomic_units() {
        let m = AtomicMoney::new(1_000, "USDC").unwrap();
        assert_eq!(m.amount, 1_000);
    }

    #[test]
    fn duplicate_replay_key_is_rejected() {
        let mut guard = ReplayGuard::default();
        guard.claim("request:payment:1").unwrap();
        assert_eq!(guard.claim("request:payment:1"), Err("replay"));
    }

    #[test]
    fn reconciliation_keeps_unknown_distinct_from_failure() {
        assert_ne!(ReconciliationOutcome::StillUnknown, ReconciliationOutcome::AuthoritativeFailure);
    }
}
