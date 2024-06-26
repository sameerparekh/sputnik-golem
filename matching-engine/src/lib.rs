use std::cell::RefCell;
use std::cmp::{min, Reverse};
use std::collections::HashMap;

use mockall::automock;
use priority_queue::PriorityQueue;

use sputnik::matching_engine::api::Error::MissingOrder;
use sputnik::matching_engine::api::Side::{Buy, Sell};
use sputnik::matching_engine::api::Status::{Canceled, Filled, Open, PartialFilled};
use sputnik::matching_engine::api::{Error, Fill, Guest, Order, OrderBook, OrderStatus};

use crate::bindings::exports::sputnik;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::accountant_stub::stub_accountant;
use crate::bindings::sputnik::accountant_stub::stub_accountant::Fill as AccountantFill;
use crate::bindings::sputnik::registry::api::Trader;

mod bindings;

struct Component;

type OrderId = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Price {
    AskPrice(Reverse<u64>),
    BidPrice(u64),
}

#[derive(Clone)]
struct Configuration {
    environment: String,
    accountant_component_id: String,
}

struct State {
    bids: PriorityQueue<OrderId, Price>,
    asks: PriorityQueue<OrderId, Price>,
    orders: HashMap<OrderId, Order>,
    order_statuses: HashMap<OrderId, OrderStatus>,
    fills: Vec<Fill>,
    configuration: Option<Configuration>,
    external_service_api: Box<dyn ExternalServiceApi>,
}

#[automock]
trait ExternalServiceApi {
    fn get_accountant(&self, configuration: Configuration, trader: u64) -> stub_accountant::Api;

    fn process_maker_fill(&self, configuration: Configuration, trader: u64, fill: Fill);
}
pub struct ExternalServiceApiProd;

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_accountant(&self, configuration: Configuration, trader: u64) -> stub_accountant::Api {
        let environment = configuration.environment;
        let accountant_component_id = configuration.accountant_component_id;
        let uri = Uri {
            value: format!("worker://{accountant_component_id}/{environment}-{trader}"),
        };

        stub_accountant::Api::new(&uri)
    }

    fn process_maker_fill(&self, configuration: Configuration, trader: u64, fill: Fill) {
        self.get_accountant(configuration, trader)
            .process_maker_fill(AccountantFill {
                price: fill.price,
                size: fill.size,
                taker_order_id: fill.taker_order_id,
                maker_order_id: fill.maker_order_id,
                timestamp: fill.timestamp,
            })
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        bids: PriorityQueue::new(),
        asks: PriorityQueue::new(),
        orders: HashMap::new(),
        order_statuses: HashMap::new(),
        fills: vec![],
        configuration: None,
        external_service_api: Box::new(ExternalServiceApiProd)
    });
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

impl Guest for Component {
    fn init(accountant_component_id: String, environment: String) -> Result<(), Error> {
        with_state(|state| match state.configuration {
            None => {
                state.configuration = Some(Configuration {
                    environment: environment.to_string(),
                    accountant_component_id: accountant_component_id.to_string(),
                });
                Ok(())
            }
            Some(_) => Err(Error::AlreadyIntialized),
        })
    }

    fn place_order(order: Order) -> Result<OrderStatus, Error> {
        with_state(|state| {
            if state.orders.contains_key(&order.id) {
                return Err(Error::DuplicateId(order.id));
            }
            state.orders.insert(order.id, order);
            let new_status = OrderStatus {
                id: order.id,
                status: Open,
                fills: vec![],
                original_size: order.size,
            };
            state.order_statuses.insert(order.id, new_status);
            let (&mut ref mut my_side, &mut ref mut other_side, order_price) = match order.side {
                Buy => (
                    &mut state.bids,
                    &mut state.asks,
                    Price::BidPrice(order.price),
                ),
                Sell => (
                    &mut state.asks,
                    &mut state.bids,
                    Price::AskPrice(Reverse(order.price)),
                ),
            };
            let mut remaining_size = order.size;
            while remaining_size > 0 {
                let matched_order_id = match other_side.peek() {
                    Some((order_id, Price::AskPrice(Reverse(ask_price))))
                        if order.price >= *ask_price =>
                    {
                        Some(order_id)
                    }
                    Some((order_id, Price::BidPrice(bid_price))) if order.price <= *bid_price => {
                        Some(order_id)
                    }
                    _ => None,
                };
                remaining_size = match matched_order_id {
                    Some(matched_id) => {
                        let matched_order = state.orders.get(matched_id).unwrap();
                        let matched_size = min(matched_order.size, remaining_size);
                        let fill = Fill {
                            price: matched_order.price,
                            size: matched_size,
                            taker_order_id: order.id,
                            maker_order_id: *matched_id,
                            timestamp: order.timestamp,
                        };
                        state.fills.push(fill);
                        let configuration = state.configuration.clone().expect("Not initialized");
                        state.external_service_api.process_maker_fill(
                            configuration,
                            matched_order.trader,
                            fill,
                        );
                        state
                            .order_statuses
                            .entry(*matched_id)
                            .and_modify(|status| {
                                status.fills.push(fill);
                                status.status = PartialFilled;
                            });
                        state.order_statuses.entry(order.id).and_modify(|status| {
                            status.fills.push(fill);
                            status.status = PartialFilled;
                        });
                        state
                            .orders
                            .entry(*matched_id)
                            .and_modify(|order| order.size -= matched_size);
                        state
                            .orders
                            .entry(order.id)
                            .and_modify(|order| order.size -= matched_size);
                        if state.orders.get(matched_id).unwrap().size == 0 {
                            state
                                .order_statuses
                                .entry(*matched_id)
                                .and_modify(|status| status.status = Filled);
                            other_side.pop();
                        }
                        if state.orders.get(&order.id).unwrap().size == 0 {
                            state
                                .order_statuses
                                .entry(order.id)
                                .and_modify(|status| status.status = Filled);
                        }
                        state.orders.get(&order.id).unwrap().size
                    }
                    None => 0,
                };
            }
            if state.orders.get(&order.id).unwrap().size > 0 {
                my_side.push(order.id, order_price);
            };
            Ok(state.order_statuses.get(&order.id).unwrap().clone())
        })
    }

    fn cancel_order(id: u64) -> Result<OrderStatus, Error> {
        with_state(|state| match state.orders.get(&id) {
            None => Err(MissingOrder(id)),
            Some(order) => {
                state
                    .order_statuses
                    .entry(id)
                    .and_modify(|status| status.status = Canceled);
                match order.side {
                    Buy => state.bids.remove(&id),
                    Sell => state.asks.remove(&id),
                };
                state.orders.entry(id).and_modify(|order| order.size = 0);
                Ok(state.order_statuses.get(&id).unwrap().clone())
            }
        })
    }

    fn get_order_book() -> OrderBook {
        with_state(|state| {
            let bids = state
                .bids
                .clone()
                .into_sorted_iter()
                .filter_map(|(order_id, _)| state.orders.get(&order_id).copied())
                .collect();
            let asks = state
                .asks
                .clone()
                .into_sorted_iter()
                .filter_map(|(order_id, _)| state.orders.get(&order_id).copied())
                .collect();
            OrderBook { bids, asks }
        })
    }

    fn get_order_status(id: u64) -> Option<OrderStatus> {
        with_state(|state| state.order_statuses.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use crate::bindings::exports::sputnik::matching_engine::api::Side::{Buy, Sell};
    use crate::bindings::exports::sputnik::matching_engine::api::Status::{
        Canceled, Filled, Open, PartialFilled,
    };
    use crate::bindings::exports::sputnik::matching_engine::api::{
        Fill, Order, OrderBook, OrderStatus,
    };
    use crate::{with_state, Component, Guest, MockExternalServiceApi};

    fn init() {
        <Component as Guest>::init("".to_string(), "".to_string())
            .expect("successful initialization");
    }

    fn setup_mock() {
        let mut mock_external_api = MockExternalServiceApi::new();
        mock_external_api
            .expect_process_maker_fill()
            .return_const(());
        with_state(|state| state.external_service_api = Box::new(mock_external_api))
    }
    impl PartialEq for Order {
        fn eq(&self, other: &Self) -> bool {
            self.timestamp == other.timestamp
                && self.side == other.side
                && self.price == other.price
                && self.id == other.id
                && self.size == other.size
                && self.trader == other.trader
        }
    }

    impl PartialEq for OrderBook {
        fn eq(&self, other: &Self) -> bool {
            self.bids == other.bids && self.asks == other.asks
        }
    }

    impl PartialEq for OrderStatus {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
                && self.fills == other.fills
                && self.status == other.status
                && self.original_size == other.original_size
        }
    }

    impl PartialEq for Fill {
        fn eq(&self, other: &Self) -> bool {
            self.timestamp == other.timestamp
                && self.size == other.size
                && self.price == other.price
                && self.size == other.size
                && self.taker_order_id == other.taker_order_id
                && self.maker_order_id == other.maker_order_id
        }
    }

    fn assert_order_book(expected: OrderBook) {
        let order_book = <Component as Guest>::get_order_book();
        assert_eq!(order_book, expected);
    }

    fn place_expect_status(order: Order, expected_status: OrderStatus) {
        let status_returned = <Component as Guest>::place_order(order).expect("returns status");
        assert_eq!(status_returned, expected_status,);
        assert_order_status(order.id, expected_status);
    }

    fn assert_order_status(id: u64, expected_status: OrderStatus) {
        assert_eq!(
            <Component as Guest>::get_order_status(id).unwrap(),
            expected_status
        )
    }

    #[test]
    fn place_a_buy_order() {
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![Order {
                id: 0,
                timestamp: 0,
                side: Buy,
                price: 51,
                size: 1,
                trader: 0,
            }],
            asks: vec![],
        });
    }

    #[test]
    fn cancel_an_order() {
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        <Component as Guest>::cancel_order(0).expect("cancel order succeeds");
        assert_order_status(
            0,
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Canceled,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![],
            asks: vec![],
        });
    }
    #[test]
    fn place_multiple_buy_orders() {
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 55,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 2,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![
                Order {
                    id: 2,
                    timestamp: 0,
                    side: Buy,
                    price: 55,
                    size: 1,
                    trader: 0,
                },
                Order {
                    id: 0,
                    timestamp: 0,
                    side: Buy,
                    price: 51,
                    size: 1,
                    trader: 0,
                },
            ],
            asks: vec![],
        });
    }

    #[test]
    fn place_multiple_sell_orders() {
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 55,
                size: 1,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 2,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![],
            asks: vec![
                Order {
                    id: 0,
                    timestamp: 0,
                    side: Sell,
                    price: 51,
                    size: 1,
                    trader: 0,
                },
                Order {
                    id: 2,
                    timestamp: 0,
                    side: Sell,
                    price: 55,
                    size: 1,
                    trader: 0,
                },
            ],
        });
    }

    #[test]
    fn place_multiple_buy_and_sell_orders_with_no_matches() {
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 55,
                size: 1,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 1,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 50,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 2,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        place_expect_status(
            Order {
                id: 3,
                timestamp: 0,
                price: 45,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 3,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![
                Order {
                    id: 2,
                    timestamp: 0,
                    side: Buy,
                    price: 50,
                    size: 1,
                    trader: 0,
                },
                Order {
                    id: 3,
                    timestamp: 0,
                    side: Buy,
                    price: 45,
                    size: 1,
                    trader: 0,
                },
            ],
            asks: vec![
                Order {
                    id: 0,
                    timestamp: 0,
                    side: Sell,
                    price: 51,
                    size: 1,
                    trader: 0,
                },
                Order {
                    id: 1,
                    timestamp: 0,
                    side: Sell,
                    price: 55,
                    size: 1,
                    trader: 0,
                },
            ],
        });
    }

    #[test]
    fn place_a_buy_and_matching_sell() {
        init();
        setup_mock();
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 1,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![Order {
                id: 0,
                timestamp: 0,
                side: Buy,
                price: 51,
                size: 1,
                trader: 0,
            }],
            asks: vec![],
        });
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 30,
                size: 1,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 1,
                fills: vec![Fill {
                    price: 51,
                    size: 1,
                    taker_order_id: 1,
                    maker_order_id: 0,
                    timestamp: 0,
                }],
                status: Filled,
                original_size: 1,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![],
            asks: vec![],
        });
    }
    #[test]
    fn place_two_buys_and_matching_sell() {
        init();
        setup_mock();
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 2,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 53,
                size: 2,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 1,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 51,
                size: 3,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 2,
                fills: vec![
                    Fill {
                        price: 53,
                        size: 2,
                        taker_order_id: 2,
                        maker_order_id: 1,
                        timestamp: 0,
                    },
                    Fill {
                        price: 51,
                        size: 1,
                        taker_order_id: 2,
                        maker_order_id: 0,
                        timestamp: 0,
                    },
                ],
                status: Filled,
                original_size: 3,
            },
        );
        assert_order_status(
            0,
            OrderStatus {
                id: 0,
                fills: vec![Fill {
                    price: 51,
                    size: 1,
                    taker_order_id: 2,
                    maker_order_id: 0,
                    timestamp: 0,
                }],
                status: PartialFilled,
                original_size: 2,
            },
        );
        assert_order_status(
            1,
            OrderStatus {
                id: 1,
                fills: vec![Fill {
                    price: 53,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: Filled,
                original_size: 2,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![Order {
                id: 0,
                timestamp: 0,
                side: Buy,
                price: 51,
                size: 1,
                trader: 0,
            }],
            asks: vec![],
        })
    }
    #[test]
    fn place_two_sells_and_matching_buy() {
        init();
        setup_mock();
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 53,
                size: 2,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 51,
                size: 2,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 1,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 53,
                size: 3,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 2,
                fills: vec![
                    Fill {
                        price: 51,
                        size: 2,
                        taker_order_id: 2,
                        maker_order_id: 1,
                        timestamp: 0,
                    },
                    Fill {
                        price: 53,
                        size: 1,
                        taker_order_id: 2,
                        maker_order_id: 0,
                        timestamp: 0,
                    },
                ],
                status: Filled,
                original_size: 3,
            },
        );
        assert_order_status(
            0,
            OrderStatus {
                id: 0,
                fills: vec![Fill {
                    price: 53,
                    size: 1,
                    taker_order_id: 2,
                    maker_order_id: 0,
                    timestamp: 0,
                }],
                status: PartialFilled,
                original_size: 2,
            },
        );
        assert_order_status(
            1,
            OrderStatus {
                id: 1,
                fills: vec![Fill {
                    price: 51,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: Filled,
                original_size: 2,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![],
            asks: vec![Order {
                id: 0,
                timestamp: 0,
                side: Sell,
                price: 53,
                size: 1,
                trader: 0,
            }],
        })
    }

    #[test]
    fn place_two_buys_and_a_partial_matching_sell() {
        init();
        setup_mock();
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 51,
                size: 2,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 53,
                size: 2,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 1,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 52,
                size: 3,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 2,
                fills: vec![Fill {
                    price: 53,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: PartialFilled,
                original_size: 3,
            },
        );
        assert_order_status(
            0,
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        assert_order_status(
            1,
            OrderStatus {
                id: 1,
                fills: vec![Fill {
                    price: 53,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: Filled,
                original_size: 2,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![Order {
                id: 0,
                timestamp: 0,
                side: Buy,
                price: 51,
                size: 2,
                trader: 0,
            }],
            asks: vec![Order {
                id: 2,
                timestamp: 0,
                side: Sell,
                price: 52,
                size: 1,
                trader: 0,
            }],
        })
    }
    #[test]
    fn place_two_sells_and_a_partial_matching_buy() {
        init();
        setup_mock();
        place_expect_status(
            Order {
                id: 0,
                timestamp: 0,
                price: 53,
                size: 2,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 1,
                timestamp: 0,
                price: 51,
                size: 2,
                trader: 0,
                side: Sell,
            },
            OrderStatus {
                id: 1,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        place_expect_status(
            Order {
                id: 2,
                timestamp: 0,
                price: 52,
                size: 3,
                trader: 0,
                side: Buy,
            },
            OrderStatus {
                id: 2,
                fills: vec![Fill {
                    price: 51,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: PartialFilled,
                original_size: 3,
            },
        );
        assert_order_status(
            0,
            OrderStatus {
                id: 0,
                fills: vec![],
                status: Open,
                original_size: 2,
            },
        );
        assert_order_status(
            1,
            OrderStatus {
                id: 1,
                fills: vec![Fill {
                    price: 51,
                    size: 2,
                    taker_order_id: 2,
                    maker_order_id: 1,
                    timestamp: 0,
                }],
                status: Filled,
                original_size: 2,
            },
        );
        assert_order_book(OrderBook {
            bids: vec![Order {
                id: 2,
                timestamp: 0,
                side: Buy,
                price: 52,
                size: 1,
                trader: 0,
            }],
            asks: vec![Order {
                id: 0,
                timestamp: 0,
                side: Sell,
                price: 53,
                size: 2,
                trader: 0,
            }],
        })
    }
}
