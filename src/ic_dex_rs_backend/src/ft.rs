// use candid::Principal;
// use ic_cdk;
// use ic_cdk_macros::*;

// pub struct FT {
//     principal: Principal,
// }

// impl FT {
//     #[query]
//     pub async fn icrc1_name(&self) -> String {
//         let call_result: Result<String, _> =
//             ic_cdk::api::call::call(self.principal, "icrc1_name", ()).await;
//         call_result.unwrap()
//     }
// }
