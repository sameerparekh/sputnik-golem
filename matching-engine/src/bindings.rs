// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod sputnik {
    pub mod matching_engine {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        #[derive(Clone, Copy)]
        pub enum Error{
          DuplicateId(u64),
          MissingOrder(u64),
          AlreadyIntialized,
        }
        impl ::core::fmt::Debug for Error {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Error::DuplicateId(e) => {
                f.debug_tuple("Error::DuplicateId").field(e).finish()
              }
              Error::MissingOrder(e) => {
                f.debug_tuple("Error::MissingOrder").field(e).finish()
              }
              Error::AlreadyIntialized => {
                f.debug_tuple("Error::AlreadyIntialized").finish()
              }
            }
          }
        }
        impl ::core::fmt::Display for Error {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            write!(f, "{:?}", self)
          }
        }
        
        impl std::error::Error for Error {}
        #[repr(u8)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub enum Side {
          Buy,
          Sell,
        }
        impl ::core::fmt::Debug for Side {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Side::Buy => {
                f.debug_tuple("Side::Buy").finish()
              }
              Side::Sell => {
                f.debug_tuple("Side::Sell").finish()
              }
            }
          }
        }
        
        impl Side{
          pub(crate) unsafe fn _lift(val: u8) -> Side{
            if !cfg!(debug_assertions) {
              return ::core::mem::transmute(val);
            }
            
            match val {
              0 => Side::Buy,
              1 => Side::Sell,
              
              _ => panic!("invalid enum discriminant"),
            }
          }
        }
        
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct Order {
          pub id: u64,
          pub timestamp: u64,
          pub side: Side,
          pub price: u64,
          pub size: u64,
          pub trader: u64,
        }
        impl ::core::fmt::Debug for Order {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Order").field("id", &self.id).field("timestamp", &self.timestamp).field("side", &self.side).field("price", &self.price).field("size", &self.size).field("trader", &self.trader).finish()
          }
        }
        #[derive(Clone)]
        pub struct OrderBook {
          pub bids: wit_bindgen::rt::vec::Vec::<Order>,
          pub asks: wit_bindgen::rt::vec::Vec::<Order>,
        }
        impl ::core::fmt::Debug for OrderBook {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("OrderBook").field("bids", &self.bids).field("asks", &self.asks).finish()
          }
        }
        #[repr(u8)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub enum Status {
          Open,
          Filled,
          PartialFilled,
          Canceled,
        }
        impl ::core::fmt::Debug for Status {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Status::Open => {
                f.debug_tuple("Status::Open").finish()
              }
              Status::Filled => {
                f.debug_tuple("Status::Filled").finish()
              }
              Status::PartialFilled => {
                f.debug_tuple("Status::PartialFilled").finish()
              }
              Status::Canceled => {
                f.debug_tuple("Status::Canceled").finish()
              }
            }
          }
        }
        
        impl Status{
          pub(crate) unsafe fn _lift(val: u8) -> Status{
            if !cfg!(debug_assertions) {
              return ::core::mem::transmute(val);
            }
            
            match val {
              0 => Status::Open,
              1 => Status::Filled,
              2 => Status::PartialFilled,
              3 => Status::Canceled,
              
              _ => panic!("invalid enum discriminant"),
            }
          }
        }
        
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct Fill {
          pub price: u64,
          pub size: u64,
          pub taker_order_id: u64,
          pub maker_order_id: u64,
          pub timestamp: u64,
        }
        impl ::core::fmt::Debug for Fill {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Fill").field("price", &self.price).field("size", &self.size).field("taker-order-id", &self.taker_order_id).field("maker-order-id", &self.maker_order_id).field("timestamp", &self.timestamp).finish()
          }
        }
        #[derive(Clone)]
        pub struct OrderStatus {
          pub id: u64,
          pub fills: wit_bindgen::rt::vec::Vec::<Fill>,
          pub status: Status,
          pub original_size: u64,
        }
        impl ::core::fmt::Debug for OrderStatus {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("OrderStatus").field("id", &self.id).field("fills", &self.fills).field("status", &self.status).field("original-size", &self.original_size).finish()
          }
        }
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:matching-engine/api#init"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_init() -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::init();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            match result0 {
              Ok(_) => { {
                *((ptr1 + 0) as *mut u8) = (0i32) as u8;
              } },
              Err(e) => { {
                *((ptr1 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr1 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::MissingOrder(e) => {
                    *((ptr1 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::AlreadyIntialized=> {
                    {
                      *((ptr1 + 8) as *mut u8) = (2i32) as u8;
                    }
                  }
                }
              } },
            };ptr1
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:matching-engine/api#place-order"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_place_order(arg0: i64,arg1: i64,arg2: i32,arg3: i64,arg4: i64,arg5: i64,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::place_order(Order{
              id: arg0 as u64,
              timestamp: arg1 as u64,
              side: Side::_lift(arg2 as u8),
              price: arg3 as u64,
              size: arg4 as u64,
              trader: arg5 as u64,
            });
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            match result0 {
              Ok(e) => { {
                *((ptr1 + 0) as *mut u8) = (0i32) as u8;
                let OrderStatus{ id:id2, fills:fills2, status:status2, original_size:original_size2, } = e;
                *((ptr1 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (fills2).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr1 + 20) as *mut i32) = len3;
                *((ptr1 + 16) as *mut i32) = ptr3;
                *((ptr1 + 24) as *mut u8) = (status2.clone() as i32) as u8;
                *((ptr1 + 32) as *mut i64) = wit_bindgen::rt::as_i64(original_size2);
              } },
              Err(e) => { {
                *((ptr1 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr1 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::MissingOrder(e) => {
                    *((ptr1 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::AlreadyIntialized=> {
                    {
                      *((ptr1 + 8) as *mut u8) = (2i32) as u8;
                    }
                  }
                }
              } },
            };ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:matching-engine/api#place-order"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_place_order(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  let base3 = l1;
                  let len3 = l2;
                  wit_bindgen::rt::dealloc(base3, (len3 as usize) * 40, 8);
                },
                _ => (),
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:matching-engine/api#cancel-order"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_cancel_order(arg0: i64,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::cancel_order(arg0 as u64);
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            match result0 {
              Ok(e) => { {
                *((ptr1 + 0) as *mut u8) = (0i32) as u8;
                let OrderStatus{ id:id2, fills:fills2, status:status2, original_size:original_size2, } = e;
                *((ptr1 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (fills2).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr1 + 20) as *mut i32) = len3;
                *((ptr1 + 16) as *mut i32) = ptr3;
                *((ptr1 + 24) as *mut u8) = (status2.clone() as i32) as u8;
                *((ptr1 + 32) as *mut i64) = wit_bindgen::rt::as_i64(original_size2);
              } },
              Err(e) => { {
                *((ptr1 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr1 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::MissingOrder(e) => {
                    *((ptr1 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::AlreadyIntialized=> {
                    {
                      *((ptr1 + 8) as *mut u8) = (2i32) as u8;
                    }
                  }
                }
              } },
            };ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:matching-engine/api#cancel-order"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_cancel_order(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  let base3 = l1;
                  let len3 = l2;
                  wit_bindgen::rt::dealloc(base3, (len3 as usize) * 40, 8);
                },
                _ => (),
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:matching-engine/api#get-order-book"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_order_book() -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::get_order_book();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let OrderBook{ bids:bids2, asks:asks2, } = result0;
            let vec4 = bids2;
            let len4 = vec4.len() as i32;
            let layout4 = alloc::Layout::from_size_align_unchecked(vec4.len() * 48, 8);
            let result4 = if layout4.size() != 0
            {
              let ptr = alloc::alloc(layout4);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout4);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec4.into_iter().enumerate() {
              let base = result4 as i32 + (i as i32) * 48;
              {
                let Order{ id:id3, timestamp:timestamp3, side:side3, price:price3, size:size3, trader:trader3, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(id3);
                *((base + 8) as *mut i64) = wit_bindgen::rt::as_i64(timestamp3);
                *((base + 16) as *mut u8) = (side3.clone() as i32) as u8;
                *((base + 24) as *mut i64) = wit_bindgen::rt::as_i64(price3);
                *((base + 32) as *mut i64) = wit_bindgen::rt::as_i64(size3);
                *((base + 40) as *mut i64) = wit_bindgen::rt::as_i64(trader3);
              }
            }
            *((ptr1 + 4) as *mut i32) = len4;
            *((ptr1 + 0) as *mut i32) = result4 as i32;
            let vec6 = asks2;
            let len6 = vec6.len() as i32;
            let layout6 = alloc::Layout::from_size_align_unchecked(vec6.len() * 48, 8);
            let result6 = if layout6.size() != 0
            {
              let ptr = alloc::alloc(layout6);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout6);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec6.into_iter().enumerate() {
              let base = result6 as i32 + (i as i32) * 48;
              {
                let Order{ id:id5, timestamp:timestamp5, side:side5, price:price5, size:size5, trader:trader5, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(id5);
                *((base + 8) as *mut i64) = wit_bindgen::rt::as_i64(timestamp5);
                *((base + 16) as *mut u8) = (side5.clone() as i32) as u8;
                *((base + 24) as *mut i64) = wit_bindgen::rt::as_i64(price5);
                *((base + 32) as *mut i64) = wit_bindgen::rt::as_i64(size5);
                *((base + 40) as *mut i64) = wit_bindgen::rt::as_i64(trader5);
              }
            }
            *((ptr1 + 12) as *mut i32) = len6;
            *((ptr1 + 8) as *mut i32) = result6 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:matching-engine/api#get-order-book"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_order_book(arg0: i32,) {
              let l0 = *((arg0 + 0) as *const i32);
              let l1 = *((arg0 + 4) as *const i32);
              let base2 = l0;
              let len2 = l1;
              wit_bindgen::rt::dealloc(base2, (len2 as usize) * 48, 8);
              let l3 = *((arg0 + 8) as *const i32);
              let l4 = *((arg0 + 12) as *const i32);
              let base5 = l3;
              let len5 = l4;
              wit_bindgen::rt::dealloc(base5, (len5 as usize) * 48, 8);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:matching-engine/api#get-order-status"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_order_status(arg0: i64,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::get_order_status(arg0 as u64);
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            match result0 {
              Some(e) => {
                *((ptr1 + 0) as *mut u8) = (1i32) as u8;
                let OrderStatus{ id:id2, fills:fills2, status:status2, original_size:original_size2, } = e;
                *((ptr1 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (fills2).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr1 + 20) as *mut i32) = len3;
                *((ptr1 + 16) as *mut i32) = ptr3;
                *((ptr1 + 24) as *mut u8) = (status2.clone() as i32) as u8;
                *((ptr1 + 32) as *mut i64) = wit_bindgen::rt::as_i64(original_size2);
              },
              None => {
                {
                  *((ptr1 + 0) as *mut u8) = (0i32) as u8;
                }
              },
            };ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:matching-engine/api#get-order-status"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_order_status(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => (),
                _ => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  let base3 = l1;
                  let len3 = l2;
                  wit_bindgen::rt::dealloc(base3, (len3 as usize) * 40, 8);
                },
              }
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn init() -> Result<(),Error>;
          fn place_order(order: Order,) -> Result<OrderStatus,Error>;
          fn cancel_order(id: u64,) -> Result<OrderStatus,Error>;
          fn get_order_book() -> OrderBook;
          fn get_order_status(id: u64,) -> Option<OrderStatus>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(8))]
        struct _RetArea([u8; 40]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 40]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:matching-engine"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1267] = [3, 0, 15, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 0, 97, 115, 109, 13, 0, 1, 0, 7, 148, 4, 1, 65, 2, 1, 66, 29, 1, 113, 3, 12, 100, 117, 112, 108, 105, 99, 97, 116, 101, 45, 105, 100, 1, 119, 0, 13, 109, 105, 115, 115, 105, 110, 103, 45, 111, 114, 100, 101, 114, 1, 119, 0, 18, 97, 108, 114, 101, 97, 100, 121, 45, 105, 110, 116, 105, 97, 108, 105, 122, 101, 100, 0, 0, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 0, 1, 109, 2, 3, 98, 117, 121, 4, 115, 101, 108, 108, 4, 0, 4, 115, 105, 100, 101, 3, 0, 2, 1, 114, 6, 2, 105, 100, 119, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 119, 4, 115, 105, 100, 101, 3, 5, 112, 114, 105, 99, 101, 119, 4, 115, 105, 122, 101, 119, 6, 116, 114, 97, 100, 101, 114, 119, 4, 0, 5, 111, 114, 100, 101, 114, 3, 0, 4, 1, 112, 5, 1, 114, 2, 4, 98, 105, 100, 115, 6, 4, 97, 115, 107, 115, 6, 4, 0, 10, 111, 114, 100, 101, 114, 45, 98, 111, 111, 107, 3, 0, 7, 1, 109, 4, 4, 111, 112, 101, 110, 6, 102, 105, 108, 108, 101, 100, 14, 112, 97, 114, 116, 105, 97, 108, 45, 102, 105, 108, 108, 101, 100, 8, 99, 97, 110, 99, 101, 108, 101, 100, 4, 0, 6, 115, 116, 97, 116, 117, 115, 3, 0, 9, 1, 114, 5, 5, 112, 114, 105, 99, 101, 119, 4, 115, 105, 122, 101, 119, 14, 116, 97, 107, 101, 114, 45, 111, 114, 100, 101, 114, 45, 105, 100, 119, 14, 109, 97, 107, 101, 114, 45, 111, 114, 100, 101, 114, 45, 105, 100, 119, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 119, 4, 0, 4, 102, 105, 108, 108, 3, 0, 11, 1, 112, 12, 1, 114, 4, 2, 105, 100, 119, 5, 102, 105, 108, 108, 115, 13, 6, 115, 116, 97, 116, 117, 115, 10, 13, 111, 114, 105, 103, 105, 110, 97, 108, 45, 115, 105, 122, 101, 119, 4, 0, 12, 111, 114, 100, 101, 114, 45, 115, 116, 97, 116, 117, 115, 3, 0, 14, 1, 106, 0, 1, 1, 1, 64, 0, 0, 16, 4, 0, 4, 105, 110, 105, 116, 1, 17, 1, 106, 1, 15, 1, 1, 1, 64, 1, 5, 111, 114, 100, 101, 114, 5, 0, 18, 4, 0, 11, 112, 108, 97, 99, 101, 45, 111, 114, 100, 101, 114, 1, 19, 1, 64, 1, 2, 105, 100, 119, 0, 18, 4, 0, 12, 99, 97, 110, 99, 101, 108, 45, 111, 114, 100, 101, 114, 1, 20, 1, 64, 0, 0, 8, 4, 0, 14, 103, 101, 116, 45, 111, 114, 100, 101, 114, 45, 98, 111, 111, 107, 1, 21, 1, 107, 15, 1, 64, 1, 2, 105, 100, 119, 0, 22, 4, 0, 16, 103, 101, 116, 45, 111, 114, 100, 101, 114, 45, 115, 116, 97, 116, 117, 115, 1, 23, 4, 1, 27, 115, 112, 117, 116, 110, 105, 107, 58, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 195, 4, 1, 65, 2, 1, 65, 2, 1, 66, 29, 1, 113, 3, 12, 100, 117, 112, 108, 105, 99, 97, 116, 101, 45, 105, 100, 1, 119, 0, 13, 109, 105, 115, 115, 105, 110, 103, 45, 111, 114, 100, 101, 114, 1, 119, 0, 18, 97, 108, 114, 101, 97, 100, 121, 45, 105, 110, 116, 105, 97, 108, 105, 122, 101, 100, 0, 0, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 0, 1, 109, 2, 3, 98, 117, 121, 4, 115, 101, 108, 108, 4, 0, 4, 115, 105, 100, 101, 3, 0, 2, 1, 114, 6, 2, 105, 100, 119, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 119, 4, 115, 105, 100, 101, 3, 5, 112, 114, 105, 99, 101, 119, 4, 115, 105, 122, 101, 119, 6, 116, 114, 97, 100, 101, 114, 119, 4, 0, 5, 111, 114, 100, 101, 114, 3, 0, 4, 1, 112, 5, 1, 114, 2, 4, 98, 105, 100, 115, 6, 4, 97, 115, 107, 115, 6, 4, 0, 10, 111, 114, 100, 101, 114, 45, 98, 111, 111, 107, 3, 0, 7, 1, 109, 4, 4, 111, 112, 101, 110, 6, 102, 105, 108, 108, 101, 100, 14, 112, 97, 114, 116, 105, 97, 108, 45, 102, 105, 108, 108, 101, 100, 8, 99, 97, 110, 99, 101, 108, 101, 100, 4, 0, 6, 115, 116, 97, 116, 117, 115, 3, 0, 9, 1, 114, 5, 5, 112, 114, 105, 99, 101, 119, 4, 115, 105, 122, 101, 119, 14, 116, 97, 107, 101, 114, 45, 111, 114, 100, 101, 114, 45, 105, 100, 119, 14, 109, 97, 107, 101, 114, 45, 111, 114, 100, 101, 114, 45, 105, 100, 119, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 119, 4, 0, 4, 102, 105, 108, 108, 3, 0, 11, 1, 112, 12, 1, 114, 4, 2, 105, 100, 119, 5, 102, 105, 108, 108, 115, 13, 6, 115, 116, 97, 116, 117, 115, 10, 13, 111, 114, 105, 103, 105, 110, 97, 108, 45, 115, 105, 122, 101, 119, 4, 0, 12, 111, 114, 100, 101, 114, 45, 115, 116, 97, 116, 117, 115, 3, 0, 14, 1, 106, 0, 1, 1, 1, 64, 0, 0, 16, 4, 0, 4, 105, 110, 105, 116, 1, 17, 1, 106, 1, 15, 1, 1, 1, 64, 1, 5, 111, 114, 100, 101, 114, 5, 0, 18, 4, 0, 11, 112, 108, 97, 99, 101, 45, 111, 114, 100, 101, 114, 1, 19, 1, 64, 1, 2, 105, 100, 119, 0, 18, 4, 0, 12, 99, 97, 110, 99, 101, 108, 45, 111, 114, 100, 101, 114, 1, 20, 1, 64, 0, 0, 8, 4, 0, 14, 103, 101, 116, 45, 111, 114, 100, 101, 114, 45, 98, 111, 111, 107, 1, 21, 1, 107, 15, 1, 64, 1, 2, 105, 100, 119, 0, 22, 4, 0, 16, 103, 101, 116, 45, 111, 114, 100, 101, 114, 45, 115, 116, 97, 116, 117, 115, 1, 23, 4, 1, 27, 115, 112, 117, 116, 110, 105, 107, 58, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 47, 97, 112, 105, 5, 0, 4, 1, 39, 115, 112, 117, 116, 110, 105, 107, 58, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 47, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 4, 0, 11, 21, 1, 0, 15, 109, 97, 116, 99, 104, 105, 110, 103, 45, 101, 110, 103, 105, 110, 101, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
