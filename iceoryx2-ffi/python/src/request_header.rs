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

use pyo3::prelude::*;
use pyo3_stub_gen_derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::unique_client_id::UniqueClientId;

#[gen_stub_pyclass]
#[pyclass(str = "{0:?}")]
/// Request header used by `MessagingPattern::RequestResponse`
pub struct RequestHeader(pub(crate) iceoryx2::service::header::request_response::RequestHeader);

#[gen_stub_pymethods]
#[pymethods]
impl RequestHeader {
    #[getter]
    /// Returns the `UniqueClientId` of the `Client` which sent the `RequestMut`
    pub fn client_id(&self) -> UniqueClientId {
        UniqueClientId(self.0.client_id())
    }

    #[getter]
    /// Returns how many elements are stored inside the requests's payload.
    pub fn number_of_elements(&self) -> u64 {
        self.0.number_of_elements()
    }
}
