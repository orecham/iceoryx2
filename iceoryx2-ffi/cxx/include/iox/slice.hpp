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

#ifndef IOX_SLICE_HPP
#define IOX_SLICE_HPP

#include "iox/assertions_addendum.hpp"

#include <cstdint>
#include <type_traits>

namespace iox {
template <typename T>
class Slice {
  public:
    using Iterator = T*;
    using ConstIterator = const T*;
    using ValueType = T;

    template <typename U>
    Slice(U* data, uint64_t len)
        : m_data { const_cast<T*>(static_cast<const std::remove_pointer_t<U>*>(data)) }
        , m_len { len } {
    }

    auto len() const -> uint64_t {
        return m_len;
    }
    auto operator[](const uint64_t n) const -> const T& {
        IOX_ASSERT(n < m_len, "Index out of bounds");
        return *(m_data + n);
    }

    auto operator[](const uint64_t n) -> T& {
        IOX_ASSERT(n < m_len, "Index out of bounds");
        return *(m_data + n);
    }

    auto begin() -> Iterator {
        return m_data;
    }

    auto begin() const -> ConstIterator {
        return m_data;
    }

    auto end() -> Iterator {
        return m_data + m_len;
    }

    auto end() const -> ConstIterator {
        return m_data + m_len;
    }

    auto data() -> T* {
        return m_data;
    };

    auto data() const -> const T* {
        return m_data;
    };

  private:
    T* m_data;
    uint64_t m_len;
};

template <typename>
struct IsSlice {
    static constexpr bool VALUE = false;
};

template <typename T>
struct IsSlice<Slice<T>> {
    static constexpr bool VALUE = true;
};
} // namespace iox

#endif
