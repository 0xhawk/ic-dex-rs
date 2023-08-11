use candid::{CandidType, Deserialize, Nat, Principal};
pub struct ICRC2 {
    principal: Principal,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct TransferFrom {
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<String>,
    pub created_at_time: Option<Nat>,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxErr {
    Other,
}

pub type TxReceipt = Result<Nat, TxErr>;

impl ICRC2 {
    pub fn new(principal: Principal) -> Self {
        ICRC2 { principal }
    }

    pub async fn icrc1_symbol(&self) -> String {
        let call_result: Result<(String,), _> =
            ic_cdk::api::call::call(self.principal, "icrc1_symbol", ()).await;
        call_result.unwrap().0
    }

    pub async fn icrc2_transfer_from(&self, args: TransferFrom) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            ic_cdk::api::call::call(self.principal, "ic_transfer_from", (args,)).await;
        call_result.unwrap().0
    }
}

pub fn new_transfer_from(
    from: Principal,
    to: Principal,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<String>,
    created_at_time: Option<Nat>,
) -> TransferFrom {
    TransferFrom {
        from,
        to,
        amount,
        fee,
        memo,
        created_at_time,
    }
}
