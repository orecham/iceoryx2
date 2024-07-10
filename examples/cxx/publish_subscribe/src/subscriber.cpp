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

#include <iostream>

#include "iox/duration.hpp"
#include "iox2/node.hpp"
#include "transmission_data.hpp"

constexpr iox::units::Duration CYCLE_TIME =
    iox::units::Duration::fromSeconds(1);

int main() {
    using namespace iox2;
    auto node = NodeBuilder().template create<ServiceType::Ipc>().expect(
        "successful node creation");

    auto service =
        node.service_builder(ServiceName::create("My/Funk/ServiceName")
                                 .expect("valid service name"))
            .publish_subscribe<TransmissionData>()
            .open_or_create()
            .expect("successful service creation/opening");

    auto subscriber = service.subscriber_builder().create().expect(
        "successful publisher creation");

    while (node.wait(CYCLE_TIME) == NodeEvent::Tick) {
        auto sample = subscriber.receive().expect("receive succeeds");
        while (sample.has_value()) {
            std::cout << "received: " << sample->payload() << std::endl;
            sample = subscriber.receive().expect("receive succeeds");
        }
    }

    std::cout << "exit" << std::endl;

    return 0;
}
