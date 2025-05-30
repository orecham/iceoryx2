// Copyright (c) 2025 Contributors to the Eclipse Foundation
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

use iceoryx2::service::service_id::ServiceId;

/// The zenoh key for discovering available service details.
pub fn discovery() -> String {
    "iox2/services/*".into()
}
/// The zenoh key at which the service details for the given service id can be received.
pub fn service_details(service_id: &ServiceId) -> String {
    format!("iox2/services/{}", service_id.as_str())
}

/// The zenoh key at which payloads for the given service id can be received.
pub fn publish_subscribe(service_id: &ServiceId) -> String {
    format!("iox2/services/{}/publish_subscribe", service_id.as_str())
}

/// The zenoh key at which events for the given service id can be received.
pub fn event(service_id: &ServiceId) -> String {
    format!("iox2/services/{}/event", service_id.as_str())
}
