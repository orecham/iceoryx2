// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(non_camel_case_types)]

use iceoryx2::prelude::*;
use iceoryx2_bb_container::semantic_string::SemanticStringError;

use core::ffi::{c_int, c_void};

mod config;
mod error;
mod event_id;
mod file_descriptor;
mod iceoryx2_settings;
mod listener;
mod log;
mod message_type_details;
mod node;
mod node_builder;
mod node_name;
mod notifier;
mod port_factory_event;
mod port_factory_listener_builder;
mod port_factory_notifier_builder;
mod port_factory_pub_sub;
mod port_factory_publisher_builder;
mod port_factory_subscriber_builder;
mod publish_subscribe_header;
mod publisher;
mod quirks_correction;
mod sample;
mod sample_mut;
mod service;
mod service_builder;
mod service_builder_event;
mod service_builder_pub_sub;
mod service_name;
mod static_config;
mod static_config_event;
mod static_config_publish_subscribe;
mod subscriber;
mod unique_listener_id;
mod unique_notifier_id;
mod unique_publisher_id;
mod unique_subscriber_id;
mod waitset;
mod waitset_attachment_id;
mod waitset_builder;
mod waitset_guard;

pub use config::*;
pub use error::*;
pub use event_id::*;
pub use file_descriptor::*;
pub use iceoryx2_settings::*;
pub use listener::*;
pub use message_type_details::*;
pub use node::*;
pub use node_builder::*;
pub use node_name::*;
pub use notifier::*;
pub use port_factory_event::*;
pub use port_factory_listener_builder::*;
pub use port_factory_notifier_builder::*;
pub use port_factory_pub_sub::*;
pub use port_factory_publisher_builder::*;
pub use port_factory_subscriber_builder::*;
pub use publish_subscribe_header::*;
pub use publisher::*;
pub use quirks_correction::*;
pub use sample::*;
pub use sample_mut::*;
pub use service::*;
pub use service_builder::*;
pub use service_builder_event::*;
pub use service_builder_pub_sub::*;
pub use service_name::*;
pub use static_config::*;
pub use static_config_event::*;
pub use static_config_publish_subscribe::*;
pub use subscriber::*;
pub use unique_listener_id::*;
pub use unique_notifier_id::*;
pub use unique_publisher_id::*;
pub use unique_subscriber_id::*;
pub use waitset::*;
pub use waitset_attachment_id::*;
pub use waitset_builder::*;
pub use waitset_guard::*;

/// This constant signals an successful function call
pub const IOX2_OK: c_int = 0;

/// An alias to a `void *` which can be used to pass arbitrary data to the callback
pub type iox2_callback_context = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iox2_callback_progression_e {
    STOP = 0,
    CONTINUE,
}

impl From<iox2_callback_progression_e> for CallbackProgression {
    fn from(value: iox2_callback_progression_e) -> Self {
        match value {
            iox2_callback_progression_e::STOP => CallbackProgression::Stop,
            iox2_callback_progression_e::CONTINUE => CallbackProgression::Continue,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iox2_semantic_string_error_e {
    INVALID_CONTENT = IOX2_OK as isize + 1,
    EXCEEDS_MAXIMUM_LENGTH,
}

impl IntoCInt for SemanticStringError {
    fn into_c_int(self) -> c_int {
        (match self {
            SemanticStringError::InvalidContent => iox2_semantic_string_error_e::INVALID_CONTENT,
            SemanticStringError::ExceedsMaximumLength => {
                iox2_semantic_string_error_e::EXCEEDS_MAXIMUM_LENGTH
            }
        }) as c_int
    }
}

/// This is a trait to convert a Rust error enum into the corresponding C error enum and then to a c_int in one go
///
/// # Example
///
/// ```no_run
/// use core::ffi::c_int;
/// use iceoryx2_ffi::IOX2_OK;
///
/// trait IntoCInt {
///     fn into_c_int(self) -> c_int;
/// }
///
/// enum FooError {
///     BAR,
///     BAZ
/// }
///
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub enum iox2_foo_error_e {
///     BAR = IOX2_OK as isize + 1, // start `IOX2_OK + 1` to prevent ambiguous values
///     BAZ,
/// }
///
/// impl IntoCInt for FooError {
///     fn into_c_int(self) -> c_int {
///         (match self {
///             FooError::BAR => iox2_foo_error_e::BAR,
///             FooError::BAZ => iox2_foo_error_e::BAZ,
///         }) as c_int
///     }
/// }
/// ```
trait IntoCInt {
    fn into_c_int(self) -> c_int;
}

trait HandleToType {
    type Target;

    // NOTE in this case, the handle `self` is already a `*mut`. Passing by value means a copy
    // of the pointer; passing by reference make the implementation more error prone since one
    // has to remember to de-reference `self` in order to get the `*mut`
    #[allow(clippy::wrong_self_convention)]
    fn as_type(self) -> Self::Target;
}

trait AssertNonNullHandle {
    fn assert_non_null(self);
}
