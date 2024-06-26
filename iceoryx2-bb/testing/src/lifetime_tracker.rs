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

use iceoryx2_pal_concurrency_sync::iox_atomic::IoxAtomicUsize;
use std::sync::atomic::Ordering;

static CREATION_COUNTER: IoxAtomicUsize = IoxAtomicUsize::new(0);
static DROP_COUNTER: IoxAtomicUsize = IoxAtomicUsize::new(0);

#[derive(Debug)]
pub struct LifetimeTracker {}

impl Default for LifetimeTracker {
    fn default() -> Self {
        CREATION_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {}
    }
}

impl LifetimeTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_tracking() {
        CREATION_COUNTER.store(0, Ordering::Relaxed);
        DROP_COUNTER.store(0, Ordering::Relaxed);
    }

    pub fn number_of_living_instances() -> usize {
        CREATION_COUNTER.load(Ordering::Relaxed) - DROP_COUNTER.load(Ordering::Relaxed)
    }
}

impl Clone for LifetimeTracker {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Drop for LifetimeTracker {
    fn drop(&mut self) {
        DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}
