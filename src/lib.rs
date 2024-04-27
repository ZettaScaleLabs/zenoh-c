//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

#![allow(non_camel_case_types)]

use std::cmp::min;
use std::slice;

#[cfg(feature = "shared-memory")]
mod context;
#[cfg(feature = "shared-memory")]
pub use crate::context::*;
mod collections;
pub use crate::collections::*;
mod config;
pub use crate::config::*;
mod commons;
pub use crate::commons::*;
mod keyexpr;
pub use crate::keyexpr::*;
mod info;
pub use crate::info::*;
mod get;
pub use crate::get::*;
mod queryable;
pub use crate::queryable::*;
mod put;
pub use crate::put::*;
mod scouting;
pub use crate::scouting::*;
mod session;
pub use crate::session::*;
mod subscriber;
pub use crate::subscriber::*;
mod pull_subscriber;
pub use crate::pull_subscriber::*;
mod publisher;
pub use crate::publisher::*;
mod closures;
pub use closures::*;
mod liveliness;
use libc::c_void;
pub use liveliness::*;
mod publication_cache;
pub use publication_cache::*;
mod querying_subscriber;
pub use querying_subscriber::*;
pub mod attachment;
pub use platform::*;
pub mod platform;
#[cfg(feature = "shared-memory")]
pub mod shm;
#[cfg(feature = "shared-memory")]
pub use crate::shm::*;

trait GuardedTransmute<D> {
    fn transmute(self) -> D;
    fn transmute_ref(&self) -> &D;
    fn transmute_mut(&mut self) -> &mut D;
}

#[macro_export]
macro_rules! decl_rust_copy_type {
    (zenoh:($zenoh_type:ty), c:($c_type:ty)) => {
        impl_guarded_transmute!(noderefs $zenoh_type, $c_type);
        impl_guarded_transmute!(noderefs $c_type, $zenoh_type);
    };
}

#[macro_export]
macro_rules! decl_rust_new_owned_type {
    (zenoh:($zenoh_type:ty), c:($c_type:ty)) => {
        impl_guarded_transmute!(noderefs $zenoh_type, $c_type);
        impl_guarded_transmute!(noderefs $c_type, $zenoh_type);

        impl $c_type {
            pub fn check(&mut self) -> bool {
                self.transmute_mut().is_some()
            }

            pub fn make_null(&mut self) {
                *self.transmute_mut() = None;
            }

            pub fn delete(&mut self) {
                let _ = self.transmute_mut().take();
            }
        }
    };
}

#[macro_export]
macro_rules! prepare_memory_to_init {
    ($owned_c_type:ident) => {{
        let owned_zenoh_type = $owned_c_type.transmute_mut();
        if owned_zenoh_type.is_some() {
            return -1; // todo: error type E_DOUBLE_INIT
        }
        owned_zenoh_type
    }};
}

#[macro_export]
macro_rules! access_loaned_memory {
    ($loaned_c_obj_mut:expr, $acess_expr:expr) => {
        access_owned_memory!($loaned_c_obj_mut.0, $acess_expr)
    };
}

#[macro_export]
macro_rules! access_owned_memory {
    ($owned_c_obj_mut:expr, $acess_expr:expr) => {
        match $owned_c_obj_mut.transmute_mut() {
            Some(val) => $acess_expr(val),
            None => -2, // todo: error type E_ACCESS_NULL
        }
    };
}

#[macro_export]
macro_rules! move_owned_memory {
    ($owned_c_obj_mut:expr, $acess_expr:expr) => {
        match $owned_c_obj_mut.transmute_mut().take() {
            Some(val) => $acess_expr(val),
            None => -3, // todo: error type E_MOVE_NULL
        }
    };
}

/// For internal use only.
///
/// This macro is used to establish the equivalence between a Rust type (first parameter) and a C layout (second parameter).
///
/// It automatically implements `From`, `Deref` and `DerefMut` to make writing code around these equivalent types.
///
/// Because carrying around the proper semantics of lifetimes is hard, this macro fails to produce working code when lifetimes are
/// present in either parameter. You may then call it with the `noderefs` prefix to avoid the offending implementations being defined.
#[macro_export]
macro_rules! impl_guarded_transmute {
    ($src_type:ty, $dst_type:ty) => {
        impl_guarded_transmute!(noderefs $src_type, $dst_type);
        impl From<$src_type> for $dst_type {
            fn from(value: $src_type) -> $dst_type {
                unsafe { core::mem::transmute(value) }
            }
        }
        impl core::ops::Deref for $dst_type {
            type Target = $src_type;
            fn deref(&self) -> &$src_type {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl core::ops::DerefMut for $dst_type {
            fn deref_mut(&mut self) -> &mut $src_type {
                unsafe { core::mem::transmute(self) }
            }
        }

    };
    (noderefs $src_type:ty, $dst_type:ty) => {
        const _: () = {
            let src = std::mem::align_of::<$src_type>();
            let dst = std::mem::align_of::<$dst_type>();
            if src != dst {
                let mut msg: [u8; 20] = *b"src:     , dst:     ";
                let mut i = 0;
                while i < 4 {
                    msg[i as usize + 5] = b'0' + ((src / 10u32.pow(3 - i) as usize) % 10) as u8;
                    msg[i as usize + 16] = b'0' + ((dst / 10u32.pow(3 - i) as usize) % 10) as u8;
                    i += 1;
                }
                panic!("{}", unsafe {
                    std::str::from_utf8_unchecked(msg.as_slice())
                });
            }
        };

        impl $crate::GuardedTransmute<$dst_type> for $src_type {
            fn transmute(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }

            fn transmute_ref(&self) -> &$dst_type {
                unsafe { std::mem::transmute::<&$src_type, &$dst_type>(self) }
            }

            fn transmute_mut(&mut self) -> &mut $dst_type {
                unsafe { std::mem::transmute::<&mut $src_type, &mut $dst_type>(self) }
            }
        }
    };
    ($src_type:ty, $dst_type:ty, $($gen: tt)*) => {
        impl<$($gen)*>  $crate::GuardedTransmute<$dst_type> for $src_type {
            fn transmute(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }
        }
        impl<$($gen)*> From<$src_type> for $dst_type {
            fn from(value: $src_type) -> $dst_type {
                unsafe { core::mem::transmute(value) }
            }
        }
        impl<$($gen)*> core::ops::Deref for $dst_type {
            type Target = $src_type;
            fn deref(&self) -> &$src_type {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl<$($gen)*> core::ops::DerefMut for $dst_type {
            fn deref_mut(&mut self) -> &mut $src_type {
                unsafe { core::mem::transmute(self) }
            }
        }

    };
}

pub(crate) const LOG_INVALID_SESSION: &str = "Invalid session";

/// Initialises the zenoh runtime logger.
///
/// Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
/// this will be performed automatically by `z_open` and `z_scout`.
#[no_mangle]
pub extern "C" fn zc_init_logger() {
    let _ = env_logger::try_init();
}

// Test should be runned with `cargo test --no-default-features`
#[test]
#[cfg(not(feature = "default"))]
fn test_no_default_features() {
    assert_eq!(
        zenoh::FEATURES,
        concat!(
            // " zenoh/auth_pubkey",
            // " zenoh/auth_usrpwd",
            // " zenoh/complete_n",
            " zenoh/shared-memory",
            // " zenoh/stats",
            // " zenoh/transport_multilink",
            // " zenoh/transport_quic",
            // " zenoh/transport_serial",
            // " zenoh/transport_unixpipe",
            // " zenoh/transport_tcp",
            // " zenoh/transport_tls",
            // " zenoh/transport_udp",
            // " zenoh/transport_unixsock-stream",
            // " zenoh/transport_ws",
            " zenoh/unstable",
            // " zenoh/default",
        )
    );
}

trait CopyableToCArray {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize;
}

impl CopyableToCArray for &[u8] {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize {
        if buf.is_null() || (len == 0 && !self.is_empty()) {
            return 0;
        }

        let max_len = min(len, self.len());
        let b = unsafe { slice::from_raw_parts_mut(buf as *mut u8, max_len) };
        b[0..max_len].copy_from_slice(&self[0..max_len]);
        max_len
    }
}

impl CopyableToCArray for &str {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize {
        self.as_bytes().copy_to_c_array(buf, len)
    }
}
