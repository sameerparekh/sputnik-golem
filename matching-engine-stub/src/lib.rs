#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::sputnik::matching_engine_stub::stub_matching_engine::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn init(
        &self,
        accountant_component_id: String,
        environment: String,
    ) -> Result<(), crate::bindings::sputnik::matching_engine::api::Error> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:matching-engine/api/init",
                &[
                    WitValue::builder().string(&accountant_component_id),
                    WitValue::builder().string(&environment),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:matching-engine/api/init"
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
            }
        })
    }
    fn place_order(
        &self,
        order: crate::bindings::sputnik::matching_engine::api::Order,
    ) -> Result<
        crate::bindings::sputnik::matching_engine::api::OrderStatus,
        crate::bindings::sputnik::matching_engine::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:matching-engine/api/place-order",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(order.id)
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
                        .item()
                        .u64(order.trader)
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:matching-engine/api/place-order"
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
            }
        })
    }
    fn cancel_order(
        &self,
        id: u64,
    ) -> Result<
        crate::bindings::sputnik::matching_engine::api::OrderStatus,
        crate::bindings::sputnik::matching_engine::api::Error,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:matching-engine/api/cancel-order",
                &[WitValue::builder().u64(id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:matching-engine/api/cancel-order"
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
            }
        })
    }
    fn get_order_book(
        &self,
    ) -> crate::bindings::sputnik::matching_engine::api::OrderBook {
        let result = self
            .rpc
            .invoke_and_await("sputnik:matching-engine/api/get-order-book", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:matching-engine/api/get-order-book"
                ),
            );
        ({
            let record = result.tuple_element(0).expect("tuple not found");
            crate::bindings::sputnik::matching_engine::api::OrderBook {
                bids: record
                    .field(0usize)
                    .expect("record field not found")
                    .list_elements(|item| {
                        let record = item;
                        crate::bindings::sputnik::matching_engine::api::Order {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            timestamp: record
                                .field(1usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            side: {
                                let case_idx = record
                                    .field(2usize)
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
                                .field(3usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            size: record
                                .field(4usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            trader: record
                                .field(5usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    })
                    .expect("list not found"),
                asks: record
                    .field(1usize)
                    .expect("record field not found")
                    .list_elements(|item| {
                        let record = item;
                        crate::bindings::sputnik::matching_engine::api::Order {
                            id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            timestamp: record
                                .field(1usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            side: {
                                let case_idx = record
                                    .field(2usize)
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
                                .field(3usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            size: record
                                .field(4usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            trader: record
                                .field(5usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                        }
                    })
                    .expect("list not found"),
            }
        })
    }
    fn get_order_status(
        &self,
        id: u64,
    ) -> Option<crate::bindings::sputnik::matching_engine::api::OrderStatus> {
        let result = self
            .rpc
            .invoke_and_await(
                "sputnik:matching-engine/api/get-order-status",
                &[WitValue::builder().u64(id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "sputnik:matching-engine/api/get-order-status"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| {
                let record = inner;
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
            }))
    }
}
