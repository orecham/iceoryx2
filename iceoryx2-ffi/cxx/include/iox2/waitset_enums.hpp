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

#ifndef IOX2_WAITSET_ENUMS_HPP
#define IOX2_WAITSET_ENUMS_HPP

#include <cstdint>

namespace iox2 {
/// Defines the failures that can occur when calling [`WaitSetBuilder::create()`].
enum class WaitSetCreateError : uint8_t {
    /// An internal error has occurred.
    InternalError
};

/// States why the [`WaitSet::run()`] method returned.
enum class WaitSetRunResult : uint8_t {
    /// A termination signal `SIGTERM` was received.
    TerminationRequest,
    /// An interrupt signal `SIGINT` was received.
    Interrupt,
    /// The user explicitly called [`WaitSet::stop()`].
    StopRequest
};

/// Defines the failures that can occur when attaching something with
/// [`WaitSet::attach_notification()`], [`WaitSet::attach_interval()`] or [`WaitSet::attach_deadline()`].
enum class WaitSetAttachmentError : uint8_t {
    /// The [`WaitSet`]s capacity is exceeded.
    InsufficientCapacity,
    /// The attachment is already attached.
    AlreadyAttached,
    /// An internal error has occurred.
    InternalError
};

/// Defines the failures that can occur when calling [`WaitSet::run()`].
enum class WaitSetRunError : uint8_t {
    /// The process has not sufficient permissions to wait on the attachments.
    InsufficientPermissions,
    /// An internal error has occurred.
    InternalError,
    /// Waiting on an empty [`WaitSet`] would lead to a deadlock therefore it causes an error.
    NoAttachments,
    /// A termination signal `SIGTERM` was received.
    TerminationRequest,
    /// An interrupt signal `SIGINT` was received.
    Interrupt
};

auto error_string(const iox2::WaitSetCreateError& error) -> const char*;
auto error_string(const iox2::WaitSetAttachmentError& error) -> const char*;
auto error_string(const iox2::WaitSetRunError& error) -> const char*;

} // namespace iox2

#endif
