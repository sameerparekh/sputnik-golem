#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::sputnik::ethereummonitor_stub::stub_ethereummonitor::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn process_deposit(
        &self,
        address: String,
        tx: String,
        amount: u64,
        token_address: String,
        block_height: u64,
    ) -> Result<
        crate::bindings::sputnik::ethereummonitor::api::AssetBalance,
        crate::bindings::sputnik::ethereummonitor::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:ethereummonitor/api/process-deposit",
                &[
                    WitValue::builder().string(&address),
                    WitValue::builder().string(&tx),
                    WitValue::builder().u64(amount),
                    WitValue::builder().string(&token_address),
                    WitValue::builder().u64(block_height),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:ethereummonitor/api/process-deposit"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok({
                        let record = ok_value.expect("result ok value not found");
                        crate::bindings::sputnik::accountant::api::AssetBalance {
                            asset: {
                                let record = record
                                    .field(0usize)
                                    .expect("record field not found");
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
                            balance: record
                                .field(1usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            available_balance: record
                                .field(2usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    })
                }
                Err(err_value) => {
                    Err({
                        let (case_idx, inner) = err_value
                            .expect("result err value not found")
                            .variant()
                            .expect("variant not found");
                        match case_idx {
                            0u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::WrongBlock(
                                    inner
                                        .expect("variant case not found")
                                        .u64()
                                        .expect("u64 not found"),
                                )
                            }
                            1u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TxSeen(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            2u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::AccountantError({
                                    let (case_idx, inner) = inner
                                        .expect("variant case not found")
                                        .variant()
                                        .expect("variant not found");
                                    match case_idx {
                                        0u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::DuplicateId(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        1u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InsufficientFunds(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        2u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::AlreadyInitialized(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        3u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::NotInitialized
                                        }
                                        4u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidAsset(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        5u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidSpotPair(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        6u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::MatchingEngineError({
                                                let (case_idx, inner) = inner
                                                    .expect("variant case not found")
                                                    .variant()
                                                    .expect("variant not found");
                                                match case_idx {
                                                    0u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::DuplicateId(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    1u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::MissingOrder(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    2u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::AlreadyIntialized
                                                    }
                                                    _ => unreachable!("invalid variant case index"),
                                                }
                                            })
                                        }
                                        _ => unreachable!("invalid variant case index"),
                                    }
                                })
                            }
                            3u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::UnknownAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            4u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::InvalidAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            5u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TokenExists(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            _ => unreachable!("invalid variant case index"),
                        }
                    })
                }
            }
        })
    }
    fn complete_block(
        &self,
        block: u64,
    ) -> Result<(), crate::bindings::sputnik::ethereummonitor::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:ethereummonitor/api/complete-block",
                &[WitValue::builder().u64(block)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:ethereummonitor/api/complete-block"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => {
                    Err({
                        let (case_idx, inner) = err_value
                            .expect("result err value not found")
                            .variant()
                            .expect("variant not found");
                        match case_idx {
                            0u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::WrongBlock(
                                    inner
                                        .expect("variant case not found")
                                        .u64()
                                        .expect("u64 not found"),
                                )
                            }
                            1u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TxSeen(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            2u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::AccountantError({
                                    let (case_idx, inner) = inner
                                        .expect("variant case not found")
                                        .variant()
                                        .expect("variant not found");
                                    match case_idx {
                                        0u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::DuplicateId(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        1u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InsufficientFunds(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        2u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::AlreadyInitialized(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        3u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::NotInitialized
                                        }
                                        4u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidAsset(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        5u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidSpotPair(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        6u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::MatchingEngineError({
                                                let (case_idx, inner) = inner
                                                    .expect("variant case not found")
                                                    .variant()
                                                    .expect("variant not found");
                                                match case_idx {
                                                    0u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::DuplicateId(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    1u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::MissingOrder(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    2u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::AlreadyIntialized
                                                    }
                                                    _ => unreachable!("invalid variant case index"),
                                                }
                                            })
                                        }
                                        _ => unreachable!("invalid variant case index"),
                                    }
                                })
                            }
                            3u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::UnknownAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            4u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::InvalidAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            5u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TokenExists(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            _ => unreachable!("invalid variant case index"),
                        }
                    })
                }
            }
        })
    }
    fn block_height(&self) -> u64 {
        let result = self
            .rpc
            .invoke_and_await("sputnik:ethereummonitor/api/block-height", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:ethereummonitor/api/block-height"
                ),
            );
        (result.tuple_element(0).expect("tuple not found").u64().expect("u64 not found"))
    }
    fn new_address_for_trader(&self, trader: u64) -> String {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:ethereummonitor/api/new-address-for-trader",
                &[WitValue::builder().u64(trader)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:ethereummonitor/api/new-address-for-trader"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .string()
            .expect("string not found")
            .to_string())
    }
    fn add_token(
        &self,
        address: String,
        asset_id: u64,
    ) -> Result<(), crate::bindings::sputnik::ethereummonitor::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:ethereummonitor/api/add-token",
                &[
                    WitValue::builder().string(&address),
                    WitValue::builder().u64(asset_id),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:ethereummonitor/api/add-token"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(()),
                Err(err_value) => {
                    Err({
                        let (case_idx, inner) = err_value
                            .expect("result err value not found")
                            .variant()
                            .expect("variant not found");
                        match case_idx {
                            0u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::WrongBlock(
                                    inner
                                        .expect("variant case not found")
                                        .u64()
                                        .expect("u64 not found"),
                                )
                            }
                            1u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TxSeen(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            2u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::AccountantError({
                                    let (case_idx, inner) = inner
                                        .expect("variant case not found")
                                        .variant()
                                        .expect("variant not found");
                                    match case_idx {
                                        0u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::DuplicateId(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        1u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InsufficientFunds(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        2u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::AlreadyInitialized(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        3u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::NotInitialized
                                        }
                                        4u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidAsset(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        5u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::InvalidSpotPair(
                                                inner
                                                    .expect("variant case not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                            )
                                        }
                                        6u32 => {
                                            crate::bindings::sputnik::accountant::api::Error::MatchingEngineError({
                                                let (case_idx, inner) = inner
                                                    .expect("variant case not found")
                                                    .variant()
                                                    .expect("variant not found");
                                                match case_idx {
                                                    0u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::DuplicateId(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    1u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::MissingOrder(
                                                            inner
                                                                .expect("variant case not found")
                                                                .u64()
                                                                .expect("u64 not found"),
                                                        )
                                                    }
                                                    2u32 => {
                                                        crate::bindings::sputnik::matching_engine::api::Error::AlreadyIntialized
                                                    }
                                                    _ => unreachable!("invalid variant case index"),
                                                }
                                            })
                                        }
                                        _ => unreachable!("invalid variant case index"),
                                    }
                                })
                            }
                            3u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::UnknownAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            4u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::InvalidAddress(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            5u32 => {
                                crate::bindings::sputnik::ethereummonitor::api::Error::TokenExists(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            _ => unreachable!("invalid variant case index"),
                        }
                    })
                }
            }
        })
    }
}
