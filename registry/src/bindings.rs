// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod sputnik {
    pub mod registry {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        #[derive(Clone, Copy)]
        pub enum Error{
          DuplicateId(u64),
          NoSuchAsset(u64),
        }
        impl ::core::fmt::Debug for Error {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Error::DuplicateId(e) => {
                f.debug_tuple("Error::DuplicateId").field(e).finish()
              }
              Error::NoSuchAsset(e) => {
                f.debug_tuple("Error::NoSuchAsset").field(e).finish()
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
        #[derive(Clone)]
        pub struct Asset {
          pub id: u64,
          pub name: wit_bindgen::rt::string::String,
          pub decimals: u8,
        }
        impl ::core::fmt::Debug for Asset {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Asset").field("id", &self.id).field("name", &self.name).field("decimals", &self.decimals).finish()
          }
        }
        #[derive(Clone)]
        pub struct HydratedSpotPair {
          pub id: u64,
          pub name: wit_bindgen::rt::string::String,
          pub numerator: Asset,
          pub denominator: Asset,
        }
        impl ::core::fmt::Debug for HydratedSpotPair {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("HydratedSpotPair").field("id", &self.id).field("name", &self.name).field("numerator", &self.numerator).field("denominator", &self.denominator).finish()
          }
        }
        #[derive(Clone)]
        pub struct SpotPair {
          pub id: u64,
          pub name: wit_bindgen::rt::string::String,
          pub numerator_id: u64,
          pub denominator_id: u64,
        }
        impl ::core::fmt::Debug for SpotPair {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("SpotPair").field("id", &self.id).field("name", &self.name).field("numerator-id", &self.numerator_id).field("denominator-id", &self.denominator_id).finish()
          }
        }
        #[derive(Clone)]
        pub struct Trader {
          pub id: u64,
          pub name: wit_bindgen::rt::string::String,
        }
        impl ::core::fmt::Debug for Trader {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Trader").field("id", &self.id).field("name", &self.name).finish()
          }
        }
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#get-assets"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_assets() -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_assets();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec4 = result0;
            let len4 = vec4.len() as i32;
            let layout4 = alloc::Layout::from_size_align_unchecked(vec4.len() * 24, 8);
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
              let base = result4 as i32 + (i as i32) * 24;
              {
                let Asset{ id:id2, name:name2, decimals:decimals2, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (name2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((base + 12) as *mut i32) = len3;
                *((base + 8) as *mut i32) = ptr3;
                *((base + 16) as *mut u8) = (wit_bindgen::rt::as_i32(decimals2)) as u8;
              }
            }
            *((ptr1 + 4) as *mut i32) = len4;
            *((ptr1 + 0) as *mut i32) = result4 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#get-assets"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_assets(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *24;
                {
                  let l0 = *((base + 8) as *const i32);
                  let l1 = *((base + 12) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 24, 8);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#get-spot-pairs"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_spot_pairs() -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_spot_pairs();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec8 = result0;
            let len8 = vec8.len() as i32;
            let layout8 = alloc::Layout::from_size_align_unchecked(vec8.len() * 64, 8);
            let result8 = if layout8.size() != 0
            {
              let ptr = alloc::alloc(layout8);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout8);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec8.into_iter().enumerate() {
              let base = result8 as i32 + (i as i32) * 64;
              {
                let HydratedSpotPair{ id:id2, name:name2, numerator:numerator2, denominator:denominator2, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (name2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((base + 12) as *mut i32) = len3;
                *((base + 8) as *mut i32) = ptr3;
                let Asset{ id:id4, name:name4, decimals:decimals4, } = numerator2;
                *((base + 16) as *mut i64) = wit_bindgen::rt::as_i64(id4);
                let vec5 = (name4.into_bytes()).into_boxed_slice();
                let ptr5 = vec5.as_ptr() as i32;
                let len5 = vec5.len() as i32;
                ::core::mem::forget(vec5);
                *((base + 28) as *mut i32) = len5;
                *((base + 24) as *mut i32) = ptr5;
                *((base + 32) as *mut u8) = (wit_bindgen::rt::as_i32(decimals4)) as u8;
                let Asset{ id:id6, name:name6, decimals:decimals6, } = denominator2;
                *((base + 40) as *mut i64) = wit_bindgen::rt::as_i64(id6);
                let vec7 = (name6.into_bytes()).into_boxed_slice();
                let ptr7 = vec7.as_ptr() as i32;
                let len7 = vec7.len() as i32;
                ::core::mem::forget(vec7);
                *((base + 52) as *mut i32) = len7;
                *((base + 48) as *mut i32) = ptr7;
                *((base + 56) as *mut u8) = (wit_bindgen::rt::as_i32(decimals6)) as u8;
              }
            }
            *((ptr1 + 4) as *mut i32) = len8;
            *((ptr1 + 0) as *mut i32) = result8 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#get-spot-pairs"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_spot_pairs(arg0: i32,) {
              let l6 = *((arg0 + 0) as *const i32);
              let l7 = *((arg0 + 4) as *const i32);
              let base8 = l6;
              let len8 = l7;
              for i in 0..len8 {
                let base = base8 + i *64;
                {
                  let l0 = *((base + 8) as *const i32);
                  let l1 = *((base + 12) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                  let l2 = *((base + 24) as *const i32);
                  let l3 = *((base + 28) as *const i32);
                  wit_bindgen::rt::dealloc(l2, (l3) as usize, 1);
                  let l4 = *((base + 48) as *const i32);
                  let l5 = *((base + 52) as *const i32);
                  wit_bindgen::rt::dealloc(l4, (l5) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base8, (len8 as usize) * 64, 8);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#get-traders"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_traders() -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_traders();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec4 = result0;
            let len4 = vec4.len() as i32;
            let layout4 = alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 8);
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
              let base = result4 as i32 + (i as i32) * 16;
              {
                let Trader{ id:id2, name:name2, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(id2);
                let vec3 = (name2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((base + 12) as *mut i32) = len3;
                *((base + 8) as *mut i32) = ptr3;
              }
            }
            *((ptr1 + 4) as *mut i32) = len4;
            *((ptr1 + 0) as *mut i32) = result4 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#get-traders"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_traders(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *16;
                {
                  let l0 = *((base + 8) as *const i32);
                  let l1 = *((base + 12) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 16, 8);
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#add-asset"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add_asset(arg0: i64,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
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
            
            let len0 = arg2 as usize;
            let bytes0 = Vec::from_raw_parts(arg1 as *mut _, len0, len0);
            let result1 = <_GuestImpl as Guest>::add_asset(Asset{
              id: arg0 as u64,
              name: wit_bindgen::rt::string_lift(bytes0),
              decimals: arg3 as u8,
            });
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            match result1 {
              Ok(e) => { {
                *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                let Asset{ id:id3, name:name3, decimals:decimals3, } = e;
                *((ptr2 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id3);
                let vec4 = (name3.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                ::core::mem::forget(vec4);
                *((ptr2 + 20) as *mut i32) = len4;
                *((ptr2 + 16) as *mut i32) = ptr4;
                *((ptr2 + 24) as *mut u8) = (wit_bindgen::rt::as_i32(decimals3)) as u8;
              } },
              Err(e) => { {
                *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr2 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::NoSuchAsset(e) => {
                    *((ptr2 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                }
              } },
            };ptr2
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#add-asset"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_add_asset(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
                _ => (),
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#add-spot-pair"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add_spot_pair(arg0: i64,arg1: i32,arg2: i32,arg3: i64,arg4: i64,) -> i32 {
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
            
            let len0 = arg2 as usize;
            let bytes0 = Vec::from_raw_parts(arg1 as *mut _, len0, len0);
            let result1 = <_GuestImpl as Guest>::add_spot_pair(SpotPair{
              id: arg0 as u64,
              name: wit_bindgen::rt::string_lift(bytes0),
              numerator_id: arg3 as u64,
              denominator_id: arg4 as u64,
            });
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            match result1 {
              Ok(e) => { {
                *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                let HydratedSpotPair{ id:id3, name:name3, numerator:numerator3, denominator:denominator3, } = e;
                *((ptr2 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id3);
                let vec4 = (name3.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                ::core::mem::forget(vec4);
                *((ptr2 + 20) as *mut i32) = len4;
                *((ptr2 + 16) as *mut i32) = ptr4;
                let Asset{ id:id5, name:name5, decimals:decimals5, } = numerator3;
                *((ptr2 + 24) as *mut i64) = wit_bindgen::rt::as_i64(id5);
                let vec6 = (name5.into_bytes()).into_boxed_slice();
                let ptr6 = vec6.as_ptr() as i32;
                let len6 = vec6.len() as i32;
                ::core::mem::forget(vec6);
                *((ptr2 + 36) as *mut i32) = len6;
                *((ptr2 + 32) as *mut i32) = ptr6;
                *((ptr2 + 40) as *mut u8) = (wit_bindgen::rt::as_i32(decimals5)) as u8;
                let Asset{ id:id7, name:name7, decimals:decimals7, } = denominator3;
                *((ptr2 + 48) as *mut i64) = wit_bindgen::rt::as_i64(id7);
                let vec8 = (name7.into_bytes()).into_boxed_slice();
                let ptr8 = vec8.as_ptr() as i32;
                let len8 = vec8.len() as i32;
                ::core::mem::forget(vec8);
                *((ptr2 + 60) as *mut i32) = len8;
                *((ptr2 + 56) as *mut i32) = ptr8;
                *((ptr2 + 64) as *mut u8) = (wit_bindgen::rt::as_i32(decimals7)) as u8;
              } },
              Err(e) => { {
                *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr2 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::NoSuchAsset(e) => {
                    *((ptr2 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                }
              } },
            };ptr2
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#add-spot-pair"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_add_spot_pair(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                  let l3 = *((arg0 + 32) as *const i32);
                  let l4 = *((arg0 + 36) as *const i32);
                  wit_bindgen::rt::dealloc(l3, (l4) as usize, 1);
                  let l5 = *((arg0 + 56) as *const i32);
                  let l6 = *((arg0 + 60) as *const i32);
                  wit_bindgen::rt::dealloc(l5, (l6) as usize, 1);
                },
                _ => (),
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sputnik:registry/api#add-trader"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add_trader(arg0: i64,arg1: i32,arg2: i32,) -> i32 {
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
            
            let len0 = arg2 as usize;
            let bytes0 = Vec::from_raw_parts(arg1 as *mut _, len0, len0);
            let result1 = <_GuestImpl as Guest>::add_trader(Trader{
              id: arg0 as u64,
              name: wit_bindgen::rt::string_lift(bytes0),
            });
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            match result1 {
              Ok(e) => { {
                *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                let Trader{ id:id3, name:name3, } = e;
                *((ptr2 + 8) as *mut i64) = wit_bindgen::rt::as_i64(id3);
                let vec4 = (name3.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                ::core::mem::forget(vec4);
                *((ptr2 + 20) as *mut i32) = len4;
                *((ptr2 + 16) as *mut i32) = ptr4;
              } },
              Err(e) => { {
                *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                match e {
                  Error::DuplicateId(e) => {
                    *((ptr2 + 8) as *mut u8) = (0i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  Error::NoSuchAsset(e) => {
                    *((ptr2 + 8) as *mut u8) = (1i32) as u8;
                    *((ptr2 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                }
              } },
            };ptr2
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_sputnik:registry/api#add-trader"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_add_trader(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
                _ => (),
              }
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn get_assets() -> wit_bindgen::rt::vec::Vec::<Asset>;
          fn get_spot_pairs() -> wit_bindgen::rt::vec::Vec::<HydratedSpotPair>;
          fn get_traders() -> wit_bindgen::rt::vec::Vec::<Trader>;
          fn add_asset(asset: Asset,) -> Result<Asset,Error>;
          fn add_spot_pair(pair: SpotPair,) -> Result<HydratedSpotPair,Error>;
          fn add_trader(trader: Trader,) -> Result<Trader,Error>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(8))]
        struct _RetArea([u8; 72]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 72]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:registry"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1037] = [3, 0, 8, 114, 101, 103, 105, 115, 116, 114, 121, 0, 97, 115, 109, 13, 0, 1, 0, 7, 175, 3, 1, 65, 2, 1, 66, 28, 1, 113, 2, 12, 100, 117, 112, 108, 105, 99, 97, 116, 101, 45, 105, 100, 1, 119, 0, 13, 110, 111, 45, 115, 117, 99, 104, 45, 97, 115, 115, 101, 116, 1, 119, 0, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 0, 1, 114, 3, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 8, 100, 101, 99, 105, 109, 97, 108, 115, 125, 4, 0, 5, 97, 115, 115, 101, 116, 3, 0, 2, 1, 114, 4, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 9, 110, 117, 109, 101, 114, 97, 116, 111, 114, 3, 11, 100, 101, 110, 111, 109, 105, 110, 97, 116, 111, 114, 3, 4, 0, 18, 104, 121, 100, 114, 97, 116, 101, 100, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 3, 0, 4, 1, 114, 4, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 12, 110, 117, 109, 101, 114, 97, 116, 111, 114, 45, 105, 100, 119, 14, 100, 101, 110, 111, 109, 105, 110, 97, 116, 111, 114, 45, 105, 100, 119, 4, 0, 9, 115, 112, 111, 116, 45, 112, 97, 105, 114, 3, 0, 6, 1, 114, 2, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 4, 0, 6, 116, 114, 97, 100, 101, 114, 3, 0, 8, 1, 112, 3, 1, 64, 0, 0, 10, 4, 0, 10, 103, 101, 116, 45, 97, 115, 115, 101, 116, 115, 1, 11, 1, 112, 5, 1, 64, 0, 0, 12, 4, 0, 14, 103, 101, 116, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 115, 1, 13, 1, 112, 9, 1, 64, 0, 0, 14, 4, 0, 11, 103, 101, 116, 45, 116, 114, 97, 100, 101, 114, 115, 1, 15, 1, 106, 1, 3, 1, 1, 1, 64, 1, 5, 97, 115, 115, 101, 116, 3, 0, 16, 4, 0, 9, 97, 100, 100, 45, 97, 115, 115, 101, 116, 1, 17, 1, 106, 1, 5, 1, 1, 1, 64, 1, 4, 112, 97, 105, 114, 7, 0, 18, 4, 0, 13, 97, 100, 100, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 1, 19, 1, 106, 1, 9, 1, 1, 1, 64, 1, 6, 116, 114, 97, 100, 101, 114, 9, 0, 20, 4, 0, 10, 97, 100, 100, 45, 116, 114, 97, 100, 101, 114, 1, 21, 4, 1, 20, 115, 112, 117, 116, 110, 105, 107, 58, 114, 101, 103, 105, 115, 116, 114, 121, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 208, 3, 1, 65, 2, 1, 65, 2, 1, 66, 28, 1, 113, 2, 12, 100, 117, 112, 108, 105, 99, 97, 116, 101, 45, 105, 100, 1, 119, 0, 13, 110, 111, 45, 115, 117, 99, 104, 45, 97, 115, 115, 101, 116, 1, 119, 0, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 0, 1, 114, 3, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 8, 100, 101, 99, 105, 109, 97, 108, 115, 125, 4, 0, 5, 97, 115, 115, 101, 116, 3, 0, 2, 1, 114, 4, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 9, 110, 117, 109, 101, 114, 97, 116, 111, 114, 3, 11, 100, 101, 110, 111, 109, 105, 110, 97, 116, 111, 114, 3, 4, 0, 18, 104, 121, 100, 114, 97, 116, 101, 100, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 3, 0, 4, 1, 114, 4, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 12, 110, 117, 109, 101, 114, 97, 116, 111, 114, 45, 105, 100, 119, 14, 100, 101, 110, 111, 109, 105, 110, 97, 116, 111, 114, 45, 105, 100, 119, 4, 0, 9, 115, 112, 111, 116, 45, 112, 97, 105, 114, 3, 0, 6, 1, 114, 2, 2, 105, 100, 119, 4, 110, 97, 109, 101, 115, 4, 0, 6, 116, 114, 97, 100, 101, 114, 3, 0, 8, 1, 112, 3, 1, 64, 0, 0, 10, 4, 0, 10, 103, 101, 116, 45, 97, 115, 115, 101, 116, 115, 1, 11, 1, 112, 5, 1, 64, 0, 0, 12, 4, 0, 14, 103, 101, 116, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 115, 1, 13, 1, 112, 9, 1, 64, 0, 0, 14, 4, 0, 11, 103, 101, 116, 45, 116, 114, 97, 100, 101, 114, 115, 1, 15, 1, 106, 1, 3, 1, 1, 1, 64, 1, 5, 97, 115, 115, 101, 116, 3, 0, 16, 4, 0, 9, 97, 100, 100, 45, 97, 115, 115, 101, 116, 1, 17, 1, 106, 1, 5, 1, 1, 1, 64, 1, 4, 112, 97, 105, 114, 7, 0, 18, 4, 0, 13, 97, 100, 100, 45, 115, 112, 111, 116, 45, 112, 97, 105, 114, 1, 19, 1, 106, 1, 9, 1, 1, 1, 64, 1, 6, 116, 114, 97, 100, 101, 114, 9, 0, 20, 4, 0, 10, 97, 100, 100, 45, 116, 114, 97, 100, 101, 114, 1, 21, 4, 1, 20, 115, 112, 117, 116, 110, 105, 107, 58, 114, 101, 103, 105, 115, 116, 114, 121, 47, 97, 112, 105, 5, 0, 4, 1, 25, 115, 112, 117, 116, 110, 105, 107, 58, 114, 101, 103, 105, 115, 116, 114, 121, 47, 114, 101, 103, 105, 115, 116, 114, 121, 4, 0, 11, 14, 1, 0, 8, 114, 101, 103, 105, 115, 116, 114, 121, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
