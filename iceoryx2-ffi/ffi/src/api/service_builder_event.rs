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

use crate::api::{
    c_size_t, iox2_port_factory_event_h, iox2_port_factory_event_t, iox2_service_builder_event_h,
    iox2_service_builder_event_h_ref, iox2_service_type_e, AssertNonNullHandle, HandleToType,
    IntoCInt, PortFactoryEventUnion, ServiceBuilderUnion, IOX2_OK,
};

use iceoryx2::prelude::*;
use iceoryx2::service::builder::event::{
    Builder, EventCreateError, EventOpenError, EventOpenOrCreateError,
};
use iceoryx2::service::port_factory::event::PortFactory;
use iceoryx2_bb_derive_macros::StaticStringRepresentation;
use iceoryx2_bb_elementary::AsStaticString;

use core::ffi::{c_char, c_int};
use core::mem::ManuallyDrop;

// BEGIN types definition

#[repr(C)]
#[derive(Copy, Clone, StaticStringRepresentation)]
pub enum iox2_event_open_or_create_error_e {
    #[StaticString = "does not exist"]
    O_DOES_NOT_EXIST = IOX2_OK as isize + 1,
    #[StaticString = "insufficient permissions"]
    O_INSUFFICIENT_PERMISSIONS,
    #[StaticString = "service in corrupted state"]
    O_SERVICE_IN_CORRUPTED_STATE,
    #[StaticString = "incompatible messaging pattern"]
    O_INCOMPATIBLE_MESSAGING_PATTERN,
    #[StaticString = "incompatible attributes"]
    O_INCOMPATIBLE_ATTRIBUTES,
    #[StaticString = "internal failure"]
    O_INTERNAL_FAILURE,
    #[StaticString = "hangs in creation"]
    O_HANGS_IN_CREATION,
    #[StaticString = "does not support requested amount of notifiers"]
    O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_NOTIFIERS,
    #[StaticString = "does not support requested amount of listeners"]
    O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_LISTENERS,
    #[StaticString = "does not support requested max event id"]
    O_DOES_NOT_SUPPORT_REQUESTED_MAX_EVENT_ID,
    #[StaticString = "does not support requested amount of nodes"]
    O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_NODES,
    #[StaticString = "exceeds max number of nodes"]
    O_EXCEEDS_MAX_NUMBER_OF_NODES,
    #[StaticString = "is marked for destruction"]
    O_IS_MARKED_FOR_DESTRUCTION,
    #[StaticString = "service in corrupted state"]
    C_SERVICE_IN_CORRUPTED_STATE,
    #[StaticString = "internal failure"]
    C_INTERNAL_FAILURE,
    #[StaticString = "is being created by another instance"]
    C_IS_BEING_CREATED_BY_ANOTHER_INSTANCE,
    #[StaticString = "already exists"]
    C_ALREADY_EXISTS,
    #[StaticString = "hangs in creation"]
    C_HANGS_IN_CREATION,
    #[StaticString = "insufficient permissions"]
    C_INSUFFICIENT_PERMISSIONS,
}

impl IntoCInt for EventOpenError {
    fn into_c_int(self) -> c_int {
        (match self {
            EventOpenError::DoesNotExist => iox2_event_open_or_create_error_e::O_DOES_NOT_EXIST,
            EventOpenError::InsufficientPermissions => {
                iox2_event_open_or_create_error_e::O_INSUFFICIENT_PERMISSIONS
            }
            EventOpenError::ServiceInCorruptedState => {
                iox2_event_open_or_create_error_e::O_SERVICE_IN_CORRUPTED_STATE
            }
            EventOpenError::IncompatibleMessagingPattern => {
                iox2_event_open_or_create_error_e::O_INCOMPATIBLE_MESSAGING_PATTERN
            }
            EventOpenError::IncompatibleAttributes => {
                iox2_event_open_or_create_error_e::O_INCOMPATIBLE_ATTRIBUTES
            }
            EventOpenError::InternalFailure => {
                iox2_event_open_or_create_error_e::O_INTERNAL_FAILURE
            }
            EventOpenError::HangsInCreation => {
                iox2_event_open_or_create_error_e::O_HANGS_IN_CREATION
            }
            EventOpenError::DoesNotSupportRequestedAmountOfNotifiers => {
                iox2_event_open_or_create_error_e::O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_NOTIFIERS
            }
            EventOpenError::DoesNotSupportRequestedAmountOfListeners => {
                iox2_event_open_or_create_error_e::O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_LISTENERS
            }
            EventOpenError::DoesNotSupportRequestedMaxEventId => {
                iox2_event_open_or_create_error_e::O_DOES_NOT_SUPPORT_REQUESTED_MAX_EVENT_ID
            }
            EventOpenError::DoesNotSupportRequestedAmountOfNodes => {
                iox2_event_open_or_create_error_e::O_DOES_NOT_SUPPORT_REQUESTED_AMOUNT_OF_NODES
            }
            EventOpenError::ExceedsMaxNumberOfNodes => {
                iox2_event_open_or_create_error_e::O_EXCEEDS_MAX_NUMBER_OF_NODES
            }
            EventOpenError::IsMarkedForDestruction => {
                iox2_event_open_or_create_error_e::O_IS_MARKED_FOR_DESTRUCTION
            }
        }) as c_int
    }
}

impl IntoCInt for EventCreateError {
    fn into_c_int(self) -> c_int {
        (match self {
            EventCreateError::ServiceInCorruptedState => {
                iox2_event_open_or_create_error_e::C_SERVICE_IN_CORRUPTED_STATE
            }

            EventCreateError::InternalFailure => {
                iox2_event_open_or_create_error_e::C_INTERNAL_FAILURE
            }
            EventCreateError::IsBeingCreatedByAnotherInstance => {
                iox2_event_open_or_create_error_e::C_IS_BEING_CREATED_BY_ANOTHER_INSTANCE
            }
            EventCreateError::AlreadyExists => iox2_event_open_or_create_error_e::C_ALREADY_EXISTS,
            EventCreateError::HangsInCreation => {
                iox2_event_open_or_create_error_e::C_HANGS_IN_CREATION
            }
            EventCreateError::InsufficientPermissions => {
                iox2_event_open_or_create_error_e::C_INSUFFICIENT_PERMISSIONS
            }
        }) as c_int
    }
}

impl IntoCInt for EventOpenOrCreateError {
    fn into_c_int(self) -> c_int {
        match self {
            EventOpenOrCreateError::EventOpenError(error) => error.into_c_int(),
            EventOpenOrCreateError::EventCreateError(error) => error.into_c_int(),
        }
    }
}

// END type definition

// BEGIN C API

#[no_mangle]
pub unsafe extern "C" fn iox2_event_open_or_create_error_string(
    error: iox2_event_open_or_create_error_e,
) -> *const c_char {
    error.as_static_str().as_ptr() as *const c_char
}

/// Sets the max notifiers for the builder
///
/// # Arguments
///
/// * `service_builder_handle` - Must be a valid [`iox2_service_builder_event_h_ref`]
///   obtained by [`iox2_service_builder_event`](crate::iox2_service_builder_event).
/// * `value` - The value to set the max notifiers to
///
/// # Safety
///
/// * `service_builder_handle` must be valid handles
#[no_mangle]
pub unsafe extern "C" fn iox2_service_builder_event_set_max_notifiers(
    service_builder_handle: iox2_service_builder_event_h_ref,
    value: c_size_t,
) {
    service_builder_handle.assert_non_null();

    let service_builder_struct = unsafe { &mut *service_builder_handle.as_type() };

    match service_builder_struct.service_type {
        iox2_service_type_e::IPC => {
            let service_builder =
                ManuallyDrop::take(&mut service_builder_struct.value.as_mut().ipc);

            let service_builder = ManuallyDrop::into_inner(service_builder.event);
            service_builder_struct.set(ServiceBuilderUnion::new_ipc_event(
                service_builder.max_notifiers(value),
            ));
        }
        iox2_service_type_e::LOCAL => {
            let service_builder =
                ManuallyDrop::take(&mut service_builder_struct.value.as_mut().local);

            let service_builder = ManuallyDrop::into_inner(service_builder.event);
            service_builder_struct.set(ServiceBuilderUnion::new_local_event(
                service_builder.max_notifiers(value),
            ));
        }
    }
}

/// Sets the max listeners for the builder
///
/// # Arguments
///
/// * `service_builder_handle` - Must be a valid [`iox2_service_builder_event_h_ref`]
///   obtained by [`iox2_service_builder_event`](crate::iox2_service_builder_event).
/// * `value` - The value to set the max listeners to
///
/// # Safety
///
/// * `service_builder_handle` must be valid handles
#[no_mangle]
pub unsafe extern "C" fn iox2_service_builder_event_set_max_listeners(
    service_builder_handle: iox2_service_builder_event_h_ref,
    value: c_size_t,
) {
    service_builder_handle.assert_non_null();

    let service_builder_struct = unsafe { &mut *service_builder_handle.as_type() };

    match service_builder_struct.service_type {
        iox2_service_type_e::IPC => {
            let service_builder =
                ManuallyDrop::take(&mut service_builder_struct.value.as_mut().ipc);

            let service_builder = ManuallyDrop::into_inner(service_builder.event);
            service_builder_struct.set(ServiceBuilderUnion::new_ipc_event(
                service_builder.max_listeners(value),
            ));
        }
        iox2_service_type_e::LOCAL => {
            let service_builder =
                ManuallyDrop::take(&mut service_builder_struct.value.as_mut().local);

            let service_builder = ManuallyDrop::into_inner(service_builder.event);
            service_builder_struct.set(ServiceBuilderUnion::new_local_event(
                service_builder.max_listeners(value),
            ));
        }
    }
}

// TODO [#210] add all the other setter methods

/// Opens an event service or creates the service if it does not exist and returns a port factory to create notifiers and listeners.
///
/// # Arguments
///
/// * `service_builder_handle` - Must be a valid [`iox2_service_builder_event_h`]
///   obtained by [`iox2_service_builder_event`](crate::iox2_service_builder_event)
/// * `port_factory_struct_ptr` - Must be either a NULL pointer or a pointer to a valid
///   [`iox2_port_factory_event_t`]). If it is a NULL pointer, the storage will be allocated on the heap.
/// * `port_factory_handle_ptr` - An uninitialized or dangling [`iox2_port_factory_event_h`] handle which will be initialized by this function call.
///
/// Returns IOX2_OK on success, an [`iox2_event_open_or_create_error_e`] otherwise.
///
/// # Safety
///
/// * The `service_builder_handle` is invalid after the return of this function and leads to undefined behavior if used in another function call!
/// * The corresponding [`iox2_service_builder_t`](crate::iox2_service_builder_t) can be re-used with
///   a call to [`iox2_node_service_builder`](crate::iox2_node_service_builder)!
#[no_mangle]
pub unsafe extern "C" fn iox2_service_builder_event_open_or_create(
    service_builder_handle: iox2_service_builder_event_h,
    port_factory_struct_ptr: *mut iox2_port_factory_event_t,
    port_factory_handle_ptr: *mut iox2_port_factory_event_h,
) -> c_int {
    iox2_service_builder_event_open_create_impl(
        service_builder_handle,
        port_factory_struct_ptr,
        port_factory_handle_ptr,
        |service_builder| service_builder.open_or_create(),
        |service_builder| service_builder.open_or_create(),
    )
}

/// Opens an event service and returns a port factory to create notifiers and listeners.
///
/// # Arguments
///
/// * `service_builder_handle` - Must be a valid [`iox2_service_builder_event_h`]
///   obtained by [`iox2_service_builder_event`](crate::iox2_service_builder_event)
/// * `port_factory_struct_ptr` - Must be either a NULL pointer or a pointer to a valid
///   [`iox2_port_factory_event_t`]. If it is a NULL pointer, the storage will be allocated on the heap.
/// * `port_factory_handle_ptr` - An uninitialized or dangling [`iox2_port_factory_event_h`] handle which will be initialized by this function call.
///
/// Returns IOX2_OK on success, an [`iox2_event_open_or_create_error_e`] otherwise. Note, only the errors annotated with `O_` are relevant.
///
/// # Safety
///
/// * The `service_builder_handle` is invalid after the return of this function and leads to undefined behavior if used in another function call!
/// * The corresponding [`iox2_service_builder_t`](crate::iox2_service_builder_t) can be re-used with
///   a call to [`iox2_node_service_builder`](crate::iox2_node_service_builder)!
#[no_mangle]
pub unsafe extern "C" fn iox2_service_builder_event_open(
    service_builder_handle: iox2_service_builder_event_h,
    port_factory_struct_ptr: *mut iox2_port_factory_event_t,
    port_factory_handle_ptr: *mut iox2_port_factory_event_h,
) -> c_int {
    iox2_service_builder_event_open_create_impl(
        service_builder_handle,
        port_factory_struct_ptr,
        port_factory_handle_ptr,
        |service_builder| service_builder.open(),
        |service_builder| service_builder.open(),
    )
}

/// Creates an event service and returns a port factory to create notifiers and listeners.
///
/// # Arguments
///
/// * `service_builder_handle` - Must be a valid [`iox2_service_builder_event_h`](crate::iox2_service_builder_event_h)
///   obtained by [`iox2_service_builder_event`](crate::iox2_service_builder_event)
/// * `port_factory_struct_ptr` - Must be either a NULL pointer or a pointer to a valid
///   [`iox2_port_factory_event_t`](crate::iox2_port_factory_event_t). If it is a NULL pointer, the storage will be allocated on the heap.
/// * `port_factory_handle_ptr` - An uninitialized or dangling [`iox2_port_factory_event_h`] handle which will be initialized by this function call.
///
/// Returns IOX2_OK on success, an [`iox2_event_open_or_create_error_e`] otherwise. Note, only the errors annotated with `O_` are relevant.
///
/// # Safety
///
/// * The `service_builder_handle` is invalid after the return of this function and leads to undefined behavior if used in another function call!
/// * The corresponding [`iox2_service_builder_t`](crate::iox2_service_builder_t) can be re-used with
///   a call to [`iox2_node_service_builder`](crate::iox2_node_service_builder)!
#[no_mangle]
pub unsafe extern "C" fn iox2_service_builder_event_create(
    service_builder_handle: iox2_service_builder_event_h,
    port_factory_struct_ptr: *mut iox2_port_factory_event_t,
    port_factory_handle_ptr: *mut iox2_port_factory_event_h,
) -> c_int {
    iox2_service_builder_event_open_create_impl(
        service_builder_handle,
        port_factory_struct_ptr,
        port_factory_handle_ptr,
        |service_builder| service_builder.create(),
        |service_builder| service_builder.create(),
    )
}

unsafe fn iox2_service_builder_event_open_create_impl<E: IntoCInt>(
    service_builder_handle: iox2_service_builder_event_h,
    port_factory_struct_ptr: *mut iox2_port_factory_event_t,
    port_factory_handle_ptr: *mut iox2_port_factory_event_h,
    func_ipc: impl FnOnce(Builder<ipc::Service>) -> Result<PortFactory<ipc::Service>, E>,
    func_local: impl FnOnce(Builder<local::Service>) -> Result<PortFactory<local::Service>, E>,
) -> c_int {
    debug_assert!(!service_builder_handle.is_null());
    debug_assert!(!port_factory_handle_ptr.is_null());

    let init_port_factory_struct_ptr = |port_factory_struct_ptr: *mut iox2_port_factory_event_t| {
        let mut port_factory_struct_ptr = port_factory_struct_ptr;
        fn no_op(_: *mut iox2_port_factory_event_t) {}
        let mut deleter: fn(*mut iox2_port_factory_event_t) = no_op;
        if port_factory_struct_ptr.is_null() {
            port_factory_struct_ptr = iox2_port_factory_event_t::alloc();
            deleter = iox2_port_factory_event_t::dealloc;
        }
        debug_assert!(!port_factory_struct_ptr.is_null());

        (port_factory_struct_ptr, deleter)
    };

    let service_builder_struct = unsafe { &mut *service_builder_handle.as_type() };
    let service_type = service_builder_struct.service_type;
    let service_builder = service_builder_struct
        .value
        .as_option_mut()
        .take()
        .unwrap_or_else(|| {
            panic!("Trying to use an invalid 'iox2_service_builder_event_h'!");
        });
    (service_builder_struct.deleter)(service_builder_struct);

    match service_type {
        iox2_service_type_e::IPC => {
            let service_builder = ManuallyDrop::into_inner(service_builder.ipc);
            let service_builder = ManuallyDrop::into_inner(service_builder.event);

            match func_ipc(service_builder) {
                Ok(port_factory) => {
                    let (port_factory_struct_ptr, deleter) =
                        init_port_factory_struct_ptr(port_factory_struct_ptr);
                    (*port_factory_struct_ptr).init(
                        service_type,
                        PortFactoryEventUnion::new_ipc(port_factory),
                        deleter,
                    );
                    *port_factory_handle_ptr = (*port_factory_struct_ptr).as_handle();
                }
                Err(error) => {
                    return error.into_c_int();
                }
            }
        }
        iox2_service_type_e::LOCAL => {
            let service_builder = ManuallyDrop::into_inner(service_builder.local);
            let service_builder = ManuallyDrop::into_inner(service_builder.event);

            match func_local(service_builder) {
                Ok(port_factory) => {
                    let (port_factory_struct_ptr, deleter) =
                        init_port_factory_struct_ptr(port_factory_struct_ptr);
                    (*port_factory_struct_ptr).init(
                        service_type,
                        PortFactoryEventUnion::new_local(port_factory),
                        deleter,
                    );
                    *port_factory_handle_ptr = (*port_factory_struct_ptr).as_handle();
                }
                Err(error) => {
                    return error.into_c_int();
                }
            }
        }
    }

    IOX2_OK
}

// END C API
