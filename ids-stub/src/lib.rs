#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::sputnik::ids_stub::stub_ids::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn get_new_id(&self) -> u64 {
        let result = self
            .rpc
            .invoke_and_await("sputnik:ids/api/get-new-id", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "sputnik:ids/api/get-new-id"
                ),
            );
        (result.tuple_element(0).expect("tuple not found").u64().expect("u64 not found"))
    }
    fn get_new_ids(&self, qty: u8) -> Vec<u64> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:ids/api/get-new-ids",
                &[WitValue::builder().u8(qty)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "sputnik:ids/api/get-new-ids"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.u64().expect("u64 not found"))
            .expect("list not found"))
    }
}
