use std::{borrow::Cow, cell::UnsafeCell, collections::HashMap};

use libc::c_void;

use crate::{z_bytes_null, z_bytes_t};

use zenoh::sample::Attachment;

/// The body of a loop over an attachment's key-value pairs.
///
/// `key` and `value` are loaned to the body for the duration of a single call.
/// `context` is passed transparently through the iteration driver.
///
/// Returning `0` is treated as `continue`.
/// Returning any other value is treated as `break`.
pub type z_attachment_iter_body_t =
    extern "C" fn(key: z_bytes_t, value: z_bytes_t, context: *mut c_void) -> i8;

/// The driver of a loop over an attachment's key-value pairs.
///
/// This function is expected to call `loop_body` once for each key-value pair
/// within `iterator`, passing `context`, and returning any non-zero value immediately (breaking iteration).
pub type z_attachment_iter_driver_t = extern "C" fn(
    iterator: *const c_void,
    loop_body: z_attachment_iter_body_t,
    context: *mut c_void,
) -> i8;

/// Returns the number of key-value pairs within the attachment.
pub type z_attachment_len_t = extern "C" fn(*const c_void) -> usize;

/// The v-table for an attachment.
#[repr(C)]
pub struct z_attachment_vtable_t {
    /// See `z_attachment_iteration_driver_t`'s documentation.
    iteration_driver: z_attachment_iter_driver_t,
    /// Returns the number of key-value pairs within the attachment.
    len: z_attachment_len_t,
}

/// Constructs a specific :c:type:`z_attachment_vtable_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_attachment_vtable(
    iteration_driver: z_attachment_iter_driver_t,
    len: z_attachment_len_t,
) -> z_attachment_vtable_t {
    z_attachment_vtable_t {
        iteration_driver,
        len,
    }
}

/// A v-table based map of byte slice to byte slice.
///
/// `vtable == NULL` marks the gravestone value, as this type is often optional.
/// Users are encouraged to use `z_attachment_null` and `z_attachment_check` to interact.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_attachment_t {
    data: *const c_void,
    vtable: Option<&'static z_attachment_vtable_t>,
}

/// Returns the gravestone value for `z_attachment_t`.
#[no_mangle]
pub extern "C" fn z_attachment_check(this: &z_attachment_t) -> bool {
    this.vtable.is_some()
}

/// Returns the gravestone value for `z_attachment_t`.
#[no_mangle]
pub extern "C" fn z_attachment_null() -> z_attachment_t {
    z_attachment_t {
        data: core::ptr::null_mut(),
        vtable: None,
    }
}

/// Constructs a specific :c:type:`z_attachment_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_attachment(
    data: *const c_void,
    vtable: &'static z_attachment_vtable_t,
) -> z_attachment_t {
    z_attachment_t {
        data,
        vtable: Some(vtable),
    }
}

/// Iterate over `this`'s key-value pairs, breaking if `body` returns a non-zero
/// value for a key-value pair, and returning the latest return value.
///
/// `context` is passed to `body` to allow stateful closures.
///
/// This function takes no ownership whatsoever.
#[no_mangle]
pub extern "C" fn z_attachment_iterate(
    this: z_attachment_t,
    body: z_attachment_iter_body_t,
    context: *mut c_void,
) -> i8 {
    (this.vtable.unwrap().iteration_driver)(this.data, body, context)
}

/// Returns the value associated with the key.
#[no_mangle]
pub extern "C" fn z_attachment_get(this: z_attachment_t, key: z_bytes_t) -> z_bytes_t {
    struct attachment_get_iterator_context {
        key: z_bytes_t,
        value: z_bytes_t,
    }

    extern "C" fn attachment_get_iterator(
        key: z_bytes_t,
        value: z_bytes_t,
        context: *mut c_void,
    ) -> i8 {
        unsafe {
            let context = &mut *(context as *mut attachment_get_iterator_context);
            if context.key.as_slice() == key.as_slice() {
                context.value = value;
                1
            } else {
                0
            }
        }
    }

    let mut context = attachment_get_iterator_context {
        key: key,
        value: z_bytes_null(),
    };

    if this.vtable.map_or(false, |vtable| {
        (vtable.iteration_driver)(
            this.data,
            attachment_get_iterator,
            &mut context as *mut _ as *mut c_void,
        ) != 0
    }) {
        context.value
    } else {
        z_bytes_null()
    }
}

/// Returns the number of key-value pairs in `this`.
#[no_mangle]
pub extern "C" fn z_attachment_len(this: z_attachment_t) -> usize {
    (this.vtable.unwrap().len)(this.data)
}

/// A map of maybe-owned vector of bytes to owned vector of bytes.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
#[repr(C)]
pub struct z_owned_bytes_map_t {
    _0: [u64; 2],
    _1: [usize; 4],
}
impl core::ops::Deref for z_owned_bytes_map_t {
    type Target = UnsafeCell<Option<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

/// Constructs a new map.
#[no_mangle]
pub extern "C" fn z_bytes_map_new() -> z_owned_bytes_map_t {
    unsafe { core::mem::transmute(Some(HashMap::<Cow<[u8]>, Cow<[u8]>>::new())) }
}

/// Constructs the gravestone value for `z_owned_bytes_map_t`
#[no_mangle]
pub extern "C" fn z_bytes_map_null() -> z_owned_bytes_map_t {
    unsafe { core::mem::transmute(None::<HashMap<Cow<[u8]>, Cow<[u8]>>>) }
}

/// Returns `true` if the map is not in its gravestone state
#[no_mangle]
pub extern "C" fn z_bytes_map_check(this: &z_owned_bytes_map_t) -> bool {
    unsafe { &*this.get() }.is_some()
}
/// Destroys the map, resetting `this` to its gravestone value.
///
/// This function is double-free safe, passing a pointer to the gravestone value will have no effect.
#[no_mangle]
pub extern "C" fn z_bytes_map_drop(this: &mut z_owned_bytes_map_t) {
    let this = unsafe { &mut *this.get() };
    this.take();
}

/// Returns the value associated with `key`, returning a gravestone value if:
/// - `this` or `key` is in gravestone state.
/// - `this` has no value associated to `key`
#[no_mangle]
pub extern "C" fn z_bytes_map_get(this: &z_owned_bytes_map_t, key: z_bytes_t) -> z_bytes_t {
    let this = unsafe { &*this.get() };
    let (Some(this), Some(key)) = (this.as_ref(), key.as_slice()) else {
        return z_bytes_null();
    };
    if let Some(value) = this.get(key) {
        value.as_ref().into()
    } else {
        z_bytes_null()
    }
}

/// Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_insert_by_copy(
    this: &z_owned_bytes_map_t,
    key: z_bytes_t,
    value: z_bytes_t,
) {
    let this = unsafe { &mut *this.get() };
    if let (Some(this), Some(key), Some(value)) = (this.as_mut(), key.as_slice(), value.as_slice())
    {
        this.insert(Cow::Owned(key.to_owned()), Cow::Owned(value.to_owned()));
    }
}

/// Associates `value` to `key` in the map, aliasing them.
///
/// Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_insert_by_alias(
    this: &z_owned_bytes_map_t,
    key: z_bytes_t,
    value: z_bytes_t,
) {
    let this = unsafe { &mut *this.get() };
    if let (Some(this), Some(key), Some(value)) = (this.as_mut(), key.as_slice(), value.as_slice())
    {
        unsafe {
            this.insert(
                Cow::Borrowed(core::mem::transmute(key)),
                Cow::Borrowed(core::mem::transmute(value)),
            )
        };
    }
}

/// Returns the number of key-value pairs in the map.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
extern "C" fn z_bytes_map_len(this: &z_owned_bytes_map_t) -> usize {
    let this = unsafe { &*this.get() };
    this.as_ref().map_or(0, |this| this.len())
}

/// Iterates over the key-value pairs in the map.
///
/// `body` will be called once per pair, with `ctx` as its last argument.
/// If `body` returns a non-zero value, the iteration will stop immediately and the value will be returned.
/// Otherwise, this will return 0 once all pairs have been visited.
/// `body` is not given ownership of the key nor value, which alias the pairs in the map.
/// It is safe to keep these aliases until existing keys are modified/removed, or the map is destroyed.
/// Note that this map is unordered.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_iter(
    this: &z_owned_bytes_map_t,
    body: z_attachment_iter_body_t,
    ctx: *mut c_void,
) -> i8 {
    let this = unsafe { &*this.get() };
    if let Some(this) = this.as_ref() {
        for (key, value) in this.iter() {
            let result = body(key.as_ref().into(), value.as_ref().into(), ctx);
            if result != 0 {
                return result;
            }
        }
    }
    0
}

const Z_BYTES_MAP_VTABLE: z_attachment_vtable_t = z_attachment_vtable_t {
    len: unsafe { core::mem::transmute(z_bytes_map_len as extern "C" fn(_) -> usize) },
    iteration_driver: unsafe {
        core::mem::transmute(z_bytes_map_iter as extern "C" fn(_, _, _) -> i8)
    },
};

//TODO(sashacmc): avoid to export it to the API, how?
#[no_mangle]
pub extern "C" fn insert_in_attachment(key: z_bytes_t, value: z_bytes_t, ctx: *mut c_void) -> i8 {
    let attachments_ref: &mut Attachment = unsafe { &mut *(ctx as *mut Attachment) };
    attachments_ref.insert(key.as_slice().unwrap(), value.as_slice().unwrap());
    0
}

#[no_mangle]
extern "C" fn attachment_len(this: *const c_void) -> usize {
    let attachments_ref: &mut Attachment = unsafe { &mut *(this as *mut Attachment) };
    return attachments_ref.len();
}

#[no_mangle]
pub extern "C" fn attachment_iter(
    this: *const c_void,
    body: z_attachment_iter_body_t,
    ctx: *mut c_void,
) -> i8 {
    let attachments_ref: &mut Attachment = unsafe { &mut *(this as *mut Attachment) };
    //if let Some(attachments_ref) = attachments_ref.as_ref() {
    for (key, value) in attachments_ref.iter() {
        let result = body(key.as_ref().into(), value.as_ref().into(), ctx);
        if result != 0 {
            return result;
        }
    }
    //}
    0
}

pub const ATTACHMENT_VTABLE: z_attachment_vtable_t = z_attachment_vtable_t {
    len: attachment_len,
    iteration_driver: attachment_iter,
};

/// Aliases `this` into a generic `z_attachment_t`, allowing it to be passed to corresponding APIs.
#[no_mangle]
pub extern "C" fn z_bytes_map_as_attachment(this: &z_owned_bytes_map_t) -> z_attachment_t {
    if z_bytes_map_check(this) {
        z_attachment_t {
            data: this as *const z_owned_bytes_map_t as *mut _,
            vtable: Some(&Z_BYTES_MAP_VTABLE),
        }
    } else {
        z_attachment_t {
            data: core::ptr::null_mut(),
            vtable: None,
        }
    }
}

extern "C" fn bytes_map_from_attachment_iterator(
    key: z_bytes_t,
    value: z_bytes_t,
    ctx: *mut c_void,
) -> i8 {
    let map = unsafe { &*ctx.cast::<z_owned_bytes_map_t>() };
    z_bytes_map_insert_by_copy(map, key, value);
    0
}
extern "C" fn bytes_map_from_attachment_iterator_by_alias(
    key: z_bytes_t,
    value: z_bytes_t,
    ctx: *mut c_void,
) -> i8 {
    let map = unsafe { &*ctx.cast::<z_owned_bytes_map_t>() };
    z_bytes_map_insert_by_alias(map, key, value);
    0
}

/// Constructs a map from the provided attachment, copying keys and values.
///
/// If `this` is at gravestone value, the returned value will also be at gravestone value.
#[no_mangle]
pub extern "C" fn z_bytes_map_from_attachment(this: z_attachment_t) -> z_owned_bytes_map_t {
    if z_attachment_check(&this) {
        let mut map = z_bytes_map_new();
        z_attachment_iterate(
            this,
            bytes_map_from_attachment_iterator,
            &mut map as *mut _ as *mut _,
        );
        map
    } else {
        z_bytes_map_null()
    }
}

/// Constructs a map from the provided attachment, aliasing the attachment's keys and values.
///
/// If `this` is at gravestone value, the returned value will also be at gravestone value.
#[no_mangle]
pub extern "C" fn z_bytes_map_from_attachment_aliasing(
    this: z_attachment_t,
) -> z_owned_bytes_map_t {
    if z_attachment_check(&this) {
        let mut map = z_bytes_map_new();
        z_attachment_iterate(
            this,
            bytes_map_from_attachment_iterator_by_alias,
            &mut map as *mut _ as *mut _,
        );
        map
    } else {
        z_bytes_map_null()
    }
}
