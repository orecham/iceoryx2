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

#include "iox/slice.hpp"
#include "test.hpp"

namespace {

struct DummyData {
    static constexpr uint64_t DEFAULT_VALUE_A = 42;
    static constexpr bool DEFAULT_VALUE_Z { false };
    uint64_t a { DEFAULT_VALUE_A };
    bool z { DEFAULT_VALUE_Z };
};

TEST(SliceTest, const_correctness_is_maintained) {
    constexpr uint64_t SLICE_MAX_LENGTH = 10;

    auto elements = std::array<DummyData, SLICE_MAX_LENGTH> {};

    auto slice = iox::MutableSlice<DummyData>(elements.begin(), SLICE_MAX_LENGTH);
    ASSERT_FALSE(std::is_const<std::remove_pointer_t<decltype(slice.begin())>>::value);
    ASSERT_FALSE(std::is_const<std::remove_pointer_t<decltype(slice.end())>>::value);
    ASSERT_FALSE(std::is_const<std::remove_pointer_t<decltype(slice.data())>>::value);
    ASSERT_FALSE(std::is_const<std::remove_reference_t<decltype(slice[0])>>::value);

    auto const_slice = iox::ImmutableSlice<DummyData>(elements.begin(), SLICE_MAX_LENGTH);
    ASSERT_TRUE(std::is_const<std::remove_pointer_t<decltype(const_slice.begin())>>::value);
    ASSERT_TRUE(std::is_const<std::remove_pointer_t<decltype(const_slice.end())>>::value);
    ASSERT_TRUE(std::is_const<std::remove_pointer_t<decltype(const_slice.data())>>::value);
    ASSERT_TRUE(std::is_const<std::remove_reference_t<decltype(const_slice[0])>>::value);
}

} // namespace
