// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod sputnik {
    pub mod ethereummonitor {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:ethereummonitor/api#process-deposit"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_process_deposit(arg0: i32,arg1: i32,arg2: i64,arg3: i64,arg4: i64,) {
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
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            <_GuestImpl as Guest>::process_deposit(wit_bindgen::rt::string_lift(bytes0), arg2 as u64, arg3 as u64, arg4 as u64);
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:ethereummonitor/api#new-address-for-trader"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_new_address_for_trader(arg0: i64,) -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::new_address_for_trader(arg0 as u64);
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec2 = (result0.into_bytes()).into_boxed_slice();
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            ::core::mem::forget(vec2);
            *((ptr1 + 4) as *mut i32) = len2;
            *((ptr1 + 0) as *mut i32) = ptr2;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:ethereummonitor/api#new-address-for-trader"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_new_address_for_trader(arg0: i32,) {
              let l0 = *((arg0 + 0) as *const i32);
              let l1 = *((arg0 + 4) as *const i32);
              wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn process_deposit(address: wit_bindgen::rt::string::String,amount: u64,asset_id: u64,block_height: u64,);
          fn new_address_for_trader(trader: u64,) -> wit_bindgen::rt::string::String;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 8]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:ethereummonitor"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 491] = [3, 0, 15, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 0, 97, 115, 109, 13, 0, 1, 0, 7, 144, 1, 1, 65, 2, 1, 66, 4, 1, 64, 4, 7, 97, 100, 100, 114, 101, 115, 115, 115, 6, 97, 109, 111, 117, 110, 116, 119, 8, 97, 115, 115, 101, 116, 45, 105, 100, 119, 12, 98, 108, 111, 99, 107, 45, 104, 101, 105, 103, 104, 116, 119, 1, 0, 4, 0, 15, 112, 114, 111, 99, 101, 115, 115, 45, 100, 101, 112, 111, 115, 105, 116, 1, 0, 1, 64, 1, 6, 116, 114, 97, 100, 101, 114, 119, 0, 115, 4, 0, 22, 110, 101, 119, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 111, 114, 45, 116, 114, 97, 100, 101, 114, 1, 1, 4, 1, 27, 115, 112, 117, 116, 110, 105, 107, 58, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 191, 1, 1, 65, 2, 1, 65, 2, 1, 66, 4, 1, 64, 4, 7, 97, 100, 100, 114, 101, 115, 115, 115, 6, 97, 109, 111, 117, 110, 116, 119, 8, 97, 115, 115, 101, 116, 45, 105, 100, 119, 12, 98, 108, 111, 99, 107, 45, 104, 101, 105, 103, 104, 116, 119, 1, 0, 4, 0, 15, 112, 114, 111, 99, 101, 115, 115, 45, 100, 101, 112, 111, 115, 105, 116, 1, 0, 1, 64, 1, 6, 116, 114, 97, 100, 101, 114, 119, 0, 115, 4, 0, 22, 110, 101, 119, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 111, 114, 45, 116, 114, 97, 100, 101, 114, 1, 1, 4, 1, 27, 115, 112, 117, 116, 110, 105, 107, 58, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 47, 97, 112, 105, 5, 0, 4, 1, 39, 115, 112, 117, 116, 110, 105, 107, 58, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 47, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 4, 0, 11, 21, 1, 0, 15, 101, 116, 104, 101, 114, 101, 117, 109, 109, 111, 110, 105, 116, 111, 114, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
