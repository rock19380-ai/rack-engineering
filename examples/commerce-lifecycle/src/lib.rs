use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicMoney {
    amount: u128,
    asset: String,
}

impl AtomicMoney {
    pub fn new(amount: u128, asset: impl Into<String>) -> Result<Self, &'static str> {
        let asset = asset.into();
        if asset.trim().is_empty() {
            return Err("asset is required");
        }
        Ok(Self { amount, asset })
    }

    pub fn amount(&self) -> u128 {
        self.amount
    }

    pub fn asset(&self) -> &str {
        &self.asset
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommerceRequestId(String);

impl CommerceRequestId {
    pub fn parse(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("request id is required");
        }
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplayKey(String);

impl ReplayKey {
    pub fn new(
        request_id: &CommerceRequestId,
        payment_fingerprint: &str,
    ) -> Result<Self, &'static str> {
        if payment_fingerprint.trim().is_empty() {
            return Err("payment fingerprint is required");
        }
        Ok(Self(format!("{}:{payment_fingerprint}", request_id.0)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    Required,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotClaimed,
    Claimed,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementStatus {
    NotStarted,
    Settled,
    Rejected,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationOutcome {
    AuthoritativeSuccess,
    AuthoritativeFailure,
    ProvedNotApplied,
    StillUnknown,
}

#[derive(Debug, Clone)]
pub struct CommerceRecord {
    pub money: AtomicMoney,
    pub payment: PaymentStatus,
    pub execution: ExecutionStatus,
    pub settlement: SettlementStatus,
}

impl CommerceRecord {
    pub fn new(money: AtomicMoney) -> Self {
        Self {
            money,
            payment: PaymentStatus::Required,
            execution: ExecutionStatus::NotClaimed,
            settlement: SettlementStatus::NotStarted,
        }
    }

    pub fn verify_payment(&mut self) -> Result<(), &'static str> {
        if self.payment != PaymentStatus::Required {
            return Err("payment already resolved");
        }
        self.payment = PaymentStatus::Verified;
        Ok(())
    }

    pub fn claim_execution(&mut self) -> Result<(), &'static str> {
        if self.payment != PaymentStatus::Verified {
            return Err("payment must be verified before execution");
        }
        if self.execution != ExecutionStatus::NotClaimed {
            return Err("execution already claimed");
        }
        self.execution = ExecutionStatus::Claimed;
        Ok(())
    }

    pub fn finish_execution(&mut self, success: bool) -> Result<(), &'static str> {
        if self.execution != ExecutionStatus::Claimed {
            return Err("execution must be claimed first");
        }
        self.execution = if success {
            ExecutionStatus::Succeeded
        } else {
            ExecutionStatus::Failed
        };
        Ok(())
    }

    pub fn settle(&mut self, result: SettlementStatus) -> Result<(), &'static str> {
        if self.execution != ExecutionStatus::Succeeded {
            return Err("successful execution required before settlement");
        }
        if self.settlement != SettlementStatus::NotStarted {
            return Err("settlement already resolved");
        }
        self.settlement = result;
        Ok(())
    }
}

#[derive(Default)]
pub struct ReplayGuard {
    seen: BTreeSet<ReplayKey>,
}

impl ReplayGuard {
    pub fn claim(&mut self, key: ReplayKey) -> Result<(), &'static str> {
        if self.seen.insert(key) {
            Ok(())
        } else {
            Err("replay")
        }
    }
}

#[derive(Default)]
pub struct CommerceStore {
    records: BTreeMap<CommerceRequestId, CommerceRecord>,
}

impl CommerceStore {
    pub fn insert(
        &mut self,
        request_id: CommerceRequestId,
        record: CommerceRecord,
    ) -> Result<(), &'static str> {
        if self.records.insert(request_id, record).is_some() {
            return Err("duplicate request id");
        }
        Ok(())
    }

    pub fn get_mut(&mut self, request_id: &CommerceRequestId) -> Option<&mut CommerceRecord> {
        self.records.get_mut(request_id)
    }
}

pub fn reconcile(status: SettlementStatus) -> ReconciliationOutcome {
    match status {
        SettlementStatus::Settled => ReconciliationOutcome::AuthoritativeSuccess,
        SettlementStatus::Rejected => ReconciliationOutcome::AuthoritativeFailure,
        SettlementStatus::NotStarted => ReconciliationOutcome::ProvedNotApplied,
        SettlementStatus::Ambiguous => ReconciliationOutcome::StillUnknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> CommerceRequestId {
        CommerceRequestId::parse("req-001").unwrap()
    }

    #[test]
    fn exact_atomic_money_is_preserved() {
        let money = AtomicMoney::new(1_000, "USDC").unwrap();
        assert_eq!(money.amount(), 1_000);
        assert_eq!(money.asset(), "USDC");
    }

    #[test]
    fn duplicate_replay_key_is_rejected() {
        let mut guard = ReplayGuard::default();
        let key = ReplayKey::new(&request(), "credential-fingerprint").unwrap();
        guard.claim(key.clone()).unwrap();
        assert_eq!(guard.claim(key), Err("replay"));
    }

    #[test]
    fn execution_cannot_start_before_payment_verification() {
        let mut record = CommerceRecord::new(AtomicMoney::new(100, "USDC").unwrap());
        assert_eq!(
            record.claim_execution(),
            Err("payment must be verified before execution")
        );
    }

    #[test]
    fn execution_claim_is_once_only() {
        let mut record = CommerceRecord::new(AtomicMoney::new(100, "USDC").unwrap());
        record.verify_payment().unwrap();
        record.claim_execution().unwrap();
        assert_eq!(record.claim_execution(), Err("execution already claimed"));
    }

    #[test]
    fn settlement_ambiguity_is_not_failure() {
        assert_eq!(
            reconcile(SettlementStatus::Ambiguous),
            ReconciliationOutcome::StillUnknown
        );
        assert_ne!(
            reconcile(SettlementStatus::Ambiguous),
            ReconciliationOutcome::AuthoritativeFailure
        );
    }

    #[test]
    fn store_rejects_duplicate_request_identity() {
        let mut store = CommerceStore::default();
        let record = CommerceRecord::new(AtomicMoney::new(10, "USDC").unwrap());
        store.insert(request(), record.clone()).unwrap();
        assert_eq!(store.insert(request(), record), Err("duplicate request id"));
    }

    #[test]
    fn complete_happy_path_is_explicit() {
        let mut record = CommerceRecord::new(AtomicMoney::new(500, "USDC").unwrap());
        record.verify_payment().unwrap();
        record.claim_execution().unwrap();
        record.finish_execution(true).unwrap();
        record.settle(SettlementStatus::Settled).unwrap();

        assert_eq!(record.payment, PaymentStatus::Verified);
        assert_eq!(record.execution, ExecutionStatus::Succeeded);
        assert_eq!(record.settlement, SettlementStatus::Settled);
    }
}
