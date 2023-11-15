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

use crate::commons::*;
use crate::z_closure_sample_call;
use crate::z_owned_closure_sample_t;
use crate::z_reliability_t;
use crate::LOG_INVALID_SESSION;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SessionDeclarations;
use zenoh::prelude::SplitBuffer;
use zenoh::query::ReplyKeyExpr;
use zenoh::subscriber::Reliability;
//use zenoh_ext::FetchingSubscriber;
//use zenoh_ext::SessionExt;
use zenoh_ext::*;
use zenoh_protocol::core::SubInfo;
use zenoh_util::core::zresult::ErrNo;

use crate::{
    impl_guarded_transmute, z_keyexpr_t, z_session_t, zc_locality_t, GuardedTransmute,
    UninitializedKeyExprError,
};

/**************************************/
/*            DECLARATION             */
/**************************************/
type FetchingSubscriber = Option<Box<zenoh_ext::FetchingSubscriber<'static, ()>>>;

/// An owned zenoh querying subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `ze_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
pub struct ze_owned_querying_subscriber_t([usize; 1]);

impl_guarded_transmute!(FetchingSubscriber, ze_owned_querying_subscriber_t);

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_t<'a>(&'a ze_owned_querying_subscriber_t);

impl From<FetchingSubscriber> for ze_owned_querying_subscriber_t {
    fn from(val: FetchingSubscriber) -> Self {
        val.transmute()
    }
}

impl AsRef<FetchingSubscriber> for ze_owned_querying_subscriber_t {
    fn as_ref(&self) -> &FetchingSubscriber {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> AsRef<FetchingSubscriber> for ze_querying_subscriber_t<'a> {
    fn as_ref(&self) -> &FetchingSubscriber {
        self.0.as_ref()
    }
}

impl AsMut<FetchingSubscriber> for ze_owned_querying_subscriber_t {
    fn as_mut(&mut self) -> &mut FetchingSubscriber {
        unsafe { std::mem::transmute(self) }
    }
}

impl ze_owned_querying_subscriber_t {
    pub fn new(sub: zenoh_ext::FetchingSubscriber<'static, ()>) -> Self {
        Some(Box::new(sub)).into()
    }
    pub fn null() -> Self {
        None.into()
    }
}

/// Constructs a null safe-to-drop value of 'ze_owned_querying_subscriber_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_querying_subscriber_null() -> ze_owned_querying_subscriber_t {
    ze_owned_querying_subscriber_t::null()
}

/// Represents the set of options that can be applied to a querying subscriber,
/// upon its declaration via :c:func:`ze_declare_querying_subscriber`.
///
/// Members:
///   z_reliability_t reliability: The subscription reliability.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_options_t {
    reliability: z_reliability_t,
}

/// Constructs the default value for :c:type:`ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default() -> ze_querying_subscriber_options_t {
    let info = SubInfo::default();
    ze_querying_subscriber_options_t {
        reliability: info.reliability.into(),
    }
}

/// Declares a querying subscriber for a given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to subscribe.
///     callback: The callback function that will be called each time a data matching the subscribed expression is received.
///     opts: additional options for the querying subscriber.
///
/// Returns:
///    A :c:type:`ze_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the querying subscriber is still valid,
///    you may use `ze_querying_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// Example:
///    Declaring a subscriber passing ``NULL`` for the options:
///
///    .. code-block:: C
///
///       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
///
///    is equivalent to initializing and passing the default subscriber options:
///
///    .. code-block:: C
///
///       z_subscriber_options_t opts = z_subscriber_options_default();
///       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///
///    Passing custom arguments to the **callback** can be done by defining a custom structure:
///
///    .. code-block:: C
///
///       typedef struct {
///         z_keyexpr_t forward;
///         z_session_t session;
///       } myargs_t;
///   
///       void callback(const z_sample_t sample, const void *arg)
///       {
///         myargs_t *myargs = (myargs_t *)arg;
///         z_put(myargs->session, myargs->forward, sample->value, NULL);
///       }
///
///       int main() {
///         myargs_t cargs = {
///           forward = z_keyexpr("forward"),
///           session = s,
///         };
///         ze_querying_subscriber_options_t opts = ze_querying_subscriber_options_default();
///         opts.cargs = (void *)&cargs;
///         ze_owned_querying_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///       }
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_declare_querying_subscriber(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    mut opts: *const ze_querying_subscriber_options_t,
) -> ze_owned_querying_subscriber_t {
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);

    match session.upgrade() {
        Some(s) => {
            if opts.is_null() {
                let default = ze_querying_subscriber_options_default();
                opts = &default;
            }
            let reliability: Reliability = (*opts).reliability.into();
            let res = s
                .declare_subscriber(keyexpr)
                .querying()
                .callback(move |sample| {
                    let payload = sample.payload.contiguous();
                    let owner = match payload {
                        std::borrow::Cow::Owned(v) => zenoh::buffers::ZBuf::from(v),
                        _ => sample.payload.clone(),
                    };
                    let sample = z_sample_t::new(&sample, &owner);
                    z_closure_sample_call(&closure, &sample)
                })
                .query_accept_replies(ReplyKeyExpr::Any)
                .reliability(reliability)
                .res();
            match res {
                Ok(sub) => ze_owned_querying_subscriber_t::new(sub),
                Err(e) => {
                    log::debug!("{}", e);
                    ze_owned_querying_subscriber_t::null()
                }
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            ze_owned_querying_subscriber_t::null()
        }
    }
}

/// Undeclares the given :c:type:`ze_owned_querying_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(sub: &mut ze_owned_querying_subscriber_t) -> i8 {
    if let Some(s) = sub.as_mut().take() {
        if let Err(e) = s.close().res_sync() {
            log::warn!("{}", e);
            return e.errno().get();
        }
    }
    0
}

/// Returns ``true`` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_check(sub: &ze_owned_querying_subscriber_t) -> bool {
    sub.as_ref().is_some()
}