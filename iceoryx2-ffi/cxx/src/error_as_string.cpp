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

#include "iox2/enum_translation.hpp"
#include "iox2/service_builder_event_error.hpp"
#include "iox2/service_builder_publish_subscribe_error.hpp"

namespace iox2 {

auto as_string(const iox2::PublishSubscribeOpenError& error) -> const char* {
    return iox2_pub_sub_open_or_create_error_string(iox::into<iox2_pub_sub_open_or_create_error_e>(error));
}

auto as_string(const iox2::PublishSubscribeCreateError& error) -> const char* {
    return iox2_pub_sub_open_or_create_error_string(iox::into<iox2_pub_sub_open_or_create_error_e>(error));
}

auto as_string(const iox2::PublishSubscribeOpenOrCreateError& error) -> const char* {
    return iox2_pub_sub_open_or_create_error_string(iox::into<iox2_pub_sub_open_or_create_error_e>(error));
}

auto as_string(const iox2::EventOpenError& error) -> const char* {
    return iox2_event_open_or_create_error_string(iox::into<iox2_event_open_or_create_error_e>(error));
}

auto as_string(const iox2::EventCreateError& error) -> const char* {
    return iox2_event_open_or_create_error_string(iox::into<iox2_event_open_or_create_error_e>(error));
}

auto as_string(const iox2::EventOpenOrCreateError& error) -> const char* {
    return iox2_event_open_or_create_error_string(iox::into<iox2_event_open_or_create_error_e>(error));
}

} // namespace iox2
