#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::sputnik::accountant_stub::stub_accountant::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn initialize(
        &self,
        id: u64,
        matching_engine_component_id: String,
        registry_component_id: String,
        environment: String,
    ) -> Result<u64, crate::bindings::sputnik::accountant::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:accountant/api/initialize",
                &[
                    WitValue::builder().u64(id),
                    WitValue::builder().string(&matching_engine_component_id),
                    WitValue::builder().string(&registry_component_id),
                    WitValue::builder().string(&environment),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/initialize"
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
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .u64()
                            .expect("u64 not found"),
                    )
                }
                Err(err_value) => {
                    Err({
                        let (case_idx, inner) = err_value
                            .expect("result err value not found")
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
            }
        })
    }
    fn get_balances(
        &self,
    ) -> Vec<crate::bindings::sputnik::accountant::api::AssetBalance> {
        let result = self
            .rpc
            .invoke_and_await("sputnik:accountant/api/get-balances", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/get-balances"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
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
            .expect("list not found"))
    }
    fn place_order(
        &self,
        order: crate::bindings::sputnik::accountant::api::Order,
    ) -> Result<
        crate::bindings::sputnik::accountant::api::OrderStatus,
        crate::bindings::sputnik::accountant::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:accountant/api/place-order",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(order.id)
                        .item()
                        .u64(order.spot_pair)
                        .item()
                        .u64(order.timestamp)
                        .item()
                        .enum_value(
                            match order.side {
                                crate::bindings::sputnik::matching_engine::api::Side::Buy => {
                                    0u32
                                }
                                crate::bindings::sputnik::matching_engine::api::Side::Sell => {
                                    1u32
                                }
                            },
                        )
                        .item()
                        .u64(order.price)
                        .item()
                        .u64(order.size)
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/place-order"
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
                        crate::bindings::sputnik::accountant::api::OrderStatus {
                            id: record
                                .field(0usize)
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
            }
        })
    }
    fn deposit(
        &self,
        asset: u64,
        amount: u64,
    ) -> Result<
        crate::bindings::sputnik::accountant::api::AssetBalance,
        crate::bindings::sputnik::accountant::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:accountant/api/deposit",
                &[WitValue::builder().u64(asset), WitValue::builder().u64(amount)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/deposit"
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
            }
        })
    }
    fn withdraw(
        &self,
        asset: u64,
        amount: u64,
    ) -> Result<
        crate::bindings::sputnik::accountant::api::AssetBalance,
        crate::bindings::sputnik::accountant::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:accountant/api/withdraw",
                &[WitValue::builder().u64(asset), WitValue::builder().u64(amount)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/withdraw"
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
            }
        })
    }
    fn blocking_process_maker_fill(
        &self,
        fill: crate::bindings::sputnik::accountant::api::Fill,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:accountant/api/process-maker-fill",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(fill.price)
                        .item()
                        .u64(fill.size)
                        .item()
                        .u64(fill.taker_order_id)
                        .item()
                        .u64(fill.maker_order_id)
                        .item()
                        .u64(fill.timestamp)
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/process-maker-fill"
                ),
            );
        ()
    }
    fn process_maker_fill(
        &self,
        fill: crate::bindings::sputnik::accountant::api::Fill,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "sputnik:accountant/api/process-maker-fill",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(fill.price)
                        .item()
                        .u64(fill.size)
                        .item()
                        .u64(fill.taker_order_id)
                        .item()
                        .u64(fill.maker_order_id)
                        .item()
                        .u64(fill.timestamp)
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "sputnik:accountant/api/process-maker-fill"
                ),
            );
        ()
    }
    fn get_orders(
        &self,
    ) -> Vec<crate::bindings::sputnik::accountant::api::OrderAndStatus> {
        let result = self
            .rpc
            .invoke_and_await("sputnik:accountant/api/get-orders", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:accountant/api/get-orders"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::sputnik::accountant::api::OrderAndStatus {
                    order: {
                        let record = record
                            .field(0usize)
                            .expect("record field not found");
                        crate::bindings::sputnik::accountant::api::Order {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            spot_pair: record
                                .field(1usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            timestamp: record
                                .field(2usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            side: {
                                let case_idx = record
                                    .field(3usize)
                                    .expect("record field not found")
                                    .enum_value()
                                    .expect("enum not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Side::Buy
                                    }
                                    1u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Side::Sell
                                    }
                                    _ => unreachable!("invalid enum case index"),
                                }
                            },
                            price: record
                                .field(4usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            size: record
                                .field(5usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    },
                    status: {
                        let record = record
                            .field(1usize)
                            .expect("record field not found");
                        crate::bindings::sputnik::matching_engine::api::OrderStatus {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            fills: record
                                .field(1usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::sputnik::matching_engine::api::Fill {
                                        price: record
                                            .field(0usize)
                                            .expect("record field not found")
                                            .u64()
                                            .expect("u64 not found"),
                                        size: record
                                            .field(1usize)
                                            .expect("record field not found")
                                            .u64()
                                            .expect("u64 not found"),
                                        taker_order_id: record
                                            .field(2usize)
                                            .expect("record field not found")
                                            .u64()
                                            .expect("u64 not found"),
                                        maker_order_id: record
                                            .field(3usize)
                                            .expect("record field not found")
                                            .u64()
                                            .expect("u64 not found"),
                                        timestamp: record
                                            .field(4usize)
                                            .expect("record field not found")
                                            .u64()
                                            .expect("u64 not found"),
                                    }
                                })
                                .expect("list not found"),
                            status: {
                                let case_idx = record
                                    .field(2usize)
                                    .expect("record field not found")
                                    .enum_value()
                                    .expect("enum not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Status::Open
                                    }
                                    1u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Status::Filled
                                    }
                                    2u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Status::PartialFilled
                                    }
                                    3u32 => {
                                        crate::bindings::sputnik::matching_engine::api::Status::Canceled
                                    }
                                    _ => unreachable!("invalid enum case index"),
                                }
                            },
                            original_size: record
                                .field(3usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    },
                }
            })
            .expect("list not found"))
    }
}
