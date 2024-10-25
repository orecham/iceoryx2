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

#ifndef IOX2_NODE_FAILURE_ENUMS_HPP
#define IOX2_NODE_FAILURE_ENUMS_HPP

#include <cstdint>

namespace iox2 {
/// All failures that can occur in [`Node::list()`].
enum class NodeListFailure : uint8_t {
    /// The process has insufficient permissions.
    InsufficientPermissions,
    /// Maybe the configuration/system is broken since someone has
    /// removed/modified internal resources
    InternalError,
    /// A SIGINT signal was received
    Interrupt,
};

/// All failures that can occur in [`NodeBuilder::create()`].
enum class NodeCreationFailure : uint8_t {
    /// The process has insufficient permissions.
    InsufficientPermissions,
    /// Maybe the configuration/system is broken since someone has
    /// removed/modified internal resources
    InternalError
};

enum class NodeCleanupFailure : uint8_t {
};

auto error_string(const iox2::NodeListFailure& error) -> const char*;
auto error_string(const iox2::NodeCreationFailure& error) -> const char*;

} // namespace iox2

#endif
