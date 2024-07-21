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

#ifndef IOX2_SAMPLE_MUT_HPP
#define IOX2_SAMPLE_MUT_HPP

#include "header_publish_subscribe.hpp"
#include "iox/assertions_addendum.hpp"
#include "iox/expected.hpp"
#include "iox/function.hpp"
#include "iox/slice.hpp"
#include "service_type.hpp"

#include <cstdint>

namespace iox2 {
/// Failure that can be emitted when a [`SampleMut`] is sent via
/// [`SampleMut::send()`].
enum PublisherSendError : uint8_t {
    /// [`SampleMut::send()`] was called but the corresponding [`Publisher`]
    /// went already out of
    /// scope.
    ConnectionBrokenSincePublisherNoLongerExists,
    /// A connection between a
    /// [`Subscriber`](crate::port::subscriber::Subscriber) and a
    /// [`Publisher`] is corrupted.
    ConnectionCorrupted,
    /// A failure occurred while acquiring memory for the payload
    LoanError,
    /// A failure occurred while establishing a connection to a
    /// [`Subscriber`](crate::port::subscriber::Subscriber)
    ConnectionError,
};

template <ServiceType S, typename Payload, typename UserHeader>
class SampleMut {
  public:
    SampleMut() = default;
    SampleMut(SampleMut&&) = default;
    auto operator=(SampleMut&&) -> SampleMut& = default;
    ~SampleMut() = default;

    SampleMut(const SampleMut&) = delete;
    auto operator=(const SampleMut&) -> SampleMut& = delete;

    auto header() const -> const HeaderPublishSubscribe& {
        IOX_TODO();
    }
    auto user_header() const -> const UserHeader& {
        IOX_TODO();
    }
    auto user_header_mut() -> UserHeader& {
        IOX_TODO();
    }
    auto payload() const -> const Payload& {
        IOX_TODO();
    }
    auto payload_mut() -> Payload& {
        IOX_TODO();
    }
    void write_payload(const Payload& payload) {
        IOX_TODO();
    }
};

template <ServiceType S, typename Payload>
class SampleMut<S, Payload, void> {
  public:
    SampleMut() = default;
    SampleMut(SampleMut&&) = default;
    auto operator=(SampleMut&&) -> SampleMut& = default;
    ~SampleMut() = default;

    SampleMut(const SampleMut&) = delete;
    auto operator=(const SampleMut&) -> SampleMut& = delete;

    auto header() const -> const HeaderPublishSubscribe& {
        IOX_TODO();
    }
    auto payload() const -> const Payload& {
        IOX_TODO();
    }
    auto payload_mut() -> Payload& {
        IOX_TODO();
    }
    void write_payload(const Payload& payload) {
        IOX_TODO();
    }
};

template <ServiceType S, typename Payload>
class SampleMut<S, iox::Slice<Payload>, void> {
  public:
    SampleMut() = default;
    SampleMut(SampleMut&&) = default;
    auto operator=(SampleMut&&) -> SampleMut& = default;
    ~SampleMut() = default;

    SampleMut(const SampleMut&) = delete;
    auto operator=(const SampleMut&) -> SampleMut& = delete;

    auto header() const -> const HeaderPublishSubscribe& {
        IOX_TODO();
    }
    auto payload() const -> const Payload& {
        IOX_TODO();
    }
    auto payload_mut() -> Payload& {
        IOX_TODO();
    }
    void write_from_fn(const iox::function<Payload(uint64_t)>& initializer) {
        IOX_TODO();
    }
};

template <ServiceType S, typename Payload, typename UserHeader>
auto send_sample(SampleMut<S, Payload, UserHeader>&& sample) -> iox::expected<uint64_t, PublisherSendError> {
    IOX_TODO();
}

} // namespace iox2

#endif