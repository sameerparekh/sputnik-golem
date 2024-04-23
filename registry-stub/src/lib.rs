#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::sputnik::registry_stub::stub_registry::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn get_assets(&self) -> Vec<crate::bindings::sputnik::registry::api::Asset> {
        let result = self
            .rpc
            .invoke_and_await("sputnik:registry/api/get-assets", &[])
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/get-assets"
            ));
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::sputnik::registry::api::Asset {
                    id: record
                        .field(0usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                    name: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    decimals: record
                        .field(2usize)
                        .expect("record field not found")
                        .u8()
                        .expect("u8 not found"),
                }
            })
            .expect("list not found"))
    }
    fn get_spot_pairs(&self) -> Vec<crate::bindings::sputnik::registry::api::HydratedSpotPair> {
        let result = self
            .rpc
            .invoke_and_await("sputnik:registry/api/get-spot-pairs", &[])
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/get-spot-pairs"
            ));
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::sputnik::registry::api::HydratedSpotPair {
                    id: record
                        .field(0usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                    name: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    numerator: {
                        let record = record.field(2usize).expect("record field not found");
                        crate::bindings::sputnik::registry::api::Asset {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            name: record
                                .field(1usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            decimals: record
                                .field(2usize)
                                .expect("record field not found")
                                .u8()
                                .expect("u8 not found"),
                        }
                    },
                    denominator: {
                        let record = record.field(3usize).expect("record field not found");
                        crate::bindings::sputnik::registry::api::Asset {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            name: record
                                .field(1usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            decimals: record
                                .field(2usize)
                                .expect("record field not found")
                                .u8()
                                .expect("u8 not found"),
                        }
                    },
                }
            })
            .expect("list not found"))
    }
    fn get_traders(&self) -> Vec<crate::bindings::sputnik::registry::api::Trader> {
        let result = self
            .rpc
            .invoke_and_await("sputnik:registry/api/get-traders", &[])
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/get-traders"
            ));
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::sputnik::registry::api::Trader {
                    id: record
                        .field(0usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                    name: record
                        .field(1usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                }
            })
            .expect("list not found"))
    }
    fn add_asset(
        &self,
        asset: crate::bindings::sputnik::registry::api::Asset,
    ) -> Result<
        crate::bindings::sputnik::registry::api::Asset,
        crate::bindings::sputnik::registry::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:registry/api/add-asset",
                &[WitValue::builder()
                    .record()
                    .item()
                    .u64(asset.id)
                    .item()
                    .string(&asset.name)
                    .item()
                    .u8(asset.decimals)
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/add-asset"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok({
                    let record = ok_value.expect("result ok value not found");
                    crate::bindings::sputnik::registry::api::Asset {
                        id: record
                            .field(0usize)
                            .expect("record field not found")
                            .u64()
                            .expect("u64 not found"),
                        name: record
                            .field(1usize)
                            .expect("record field not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                        decimals: record
                            .field(2usize)
                            .expect("record field not found")
                            .u8()
                            .expect("u8 not found"),
                    }
                }),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::sputnik::registry::api::Error::DuplicateId(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        1u32 => crate::bindings::sputnik::registry::api::Error::NoSuchAsset(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn add_spot_pair(
        &self,
        pair: crate::bindings::sputnik::registry::api::SpotPair,
    ) -> Result<
        crate::bindings::sputnik::registry::api::HydratedSpotPair,
        crate::bindings::sputnik::registry::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:registry/api/add-spot-pair",
                &[WitValue::builder()
                    .record()
                    .item()
                    .u64(pair.id)
                    .item()
                    .string(&pair.name)
                    .item()
                    .u64(pair.numerator_id)
                    .item()
                    .u64(pair.denominator_id)
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/add-spot-pair"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok({
                    let record = ok_value.expect("result ok value not found");
                    crate::bindings::sputnik::registry::api::HydratedSpotPair {
                        id: record
                            .field(0usize)
                            .expect("record field not found")
                            .u64()
                            .expect("u64 not found"),
                        name: record
                            .field(1usize)
                            .expect("record field not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                        numerator: {
                            let record = record.field(2usize).expect("record field not found");
                            crate::bindings::sputnik::registry::api::Asset {
                                id: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .u64()
                                    .expect("u64 not found"),
                                name: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                decimals: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .u8()
                                    .expect("u8 not found"),
                            }
                        },
                        denominator: {
                            let record = record.field(3usize).expect("record field not found");
                            crate::bindings::sputnik::registry::api::Asset {
                                id: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .u64()
                                    .expect("u64 not found"),
                                name: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                decimals: record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .u8()
                                    .expect("u8 not found"),
                            }
                        },
                    }
                }),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::sputnik::registry::api::Error::DuplicateId(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        1u32 => crate::bindings::sputnik::registry::api::Error::NoSuchAsset(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
    fn add_trader(
        &self,
        trader: crate::bindings::sputnik::registry::api::Trader,
    ) -> Result<
        crate::bindings::sputnik::registry::api::Trader,
        crate::bindings::sputnik::registry::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:registry/api/add-trader",
                &[WitValue::builder()
                    .record()
                    .item()
                    .u64(trader.id)
                    .item()
                    .string(&trader.name)
                    .finish()],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "sputnik:registry/api/add-trader"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok({
                    let record = ok_value.expect("result ok value not found");
                    crate::bindings::sputnik::registry::api::Trader {
                        id: record
                            .field(0usize)
                            .expect("record field not found")
                            .u64()
                            .expect("u64 not found"),
                        name: record
                            .field(1usize)
                            .expect("record field not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    }
                }),
                Err(err_value) => Err({
                    let (case_idx, inner) = err_value
                        .expect("result err value not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => crate::bindings::sputnik::registry::api::Error::DuplicateId(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        1u32 => crate::bindings::sputnik::registry::api::Error::NoSuchAsset(
                            inner
                                .expect("variant case not found")
                                .u64()
                                .expect("u64 not found"),
                        ),
                        _ => unreachable!("invalid variant case index"),
                    }
                }),
            }
        })
    }
}
