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

use iceoryx2::port::publisher::Publisher as IceoryxPublisher;
use iceoryx2::port::subscriber::Subscriber as IceoryxSubscriber;
use iceoryx2::prelude::*;
use iceoryx2::service::builder::CustomHeaderMarker;
use iceoryx2::service::builder::CustomPayloadMarker;

use zenoh::bytes::ZBytes;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Publisher as ZenohPublisher;
use zenoh::pubsub::Subscriber as ZenohSubscriber;
use zenoh::sample::Sample;
use zenoh::Wait;

pub enum DataStream<'a> {
    Outbound {
        iox_subscriber: IceoryxSubscriber<ipc::Service, [CustomPayloadMarker], CustomHeaderMarker>,
        z_publisher: ZenohPublisher<'a>,
    },
    Inbound {
        iox_publisher: IceoryxPublisher<ipc::Service, [CustomPayloadMarker], CustomHeaderMarker>,
        z_subscriber: ZenohSubscriber<FifoChannelHandler<Sample>>,
    },
}

impl<'a> DataStream<'a> {
    pub fn new_outbound(
        iox_subscriber: IceoryxSubscriber<ipc::Service, [CustomPayloadMarker], CustomHeaderMarker>,
        z_publisher: ZenohPublisher<'a>,
    ) -> Self {
        Self::Outbound {
            iox_subscriber,
            z_publisher,
        }
    }

    pub fn new_inbound(
        iox_publisher: IceoryxPublisher<ipc::Service, [CustomPayloadMarker], CustomHeaderMarker>,
        z_subscriber: ZenohSubscriber<FifoChannelHandler<Sample>>,
    ) -> Self {
        Self::Inbound {
            iox_publisher,
            z_subscriber,
        }
    }

    pub fn propagate(&self) {
        match self {
            DataStream::Outbound {
                iox_subscriber,
                z_publisher,
            } => {
                while let Ok(Some(sample)) = unsafe { iox_subscriber.receive_custom_payload() } {
                    let ptr = sample.payload().as_ptr() as *const u8;
                    let len = sample.len();
                    let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };

                    let z_payload = ZBytes::from(bytes);
                    z_publisher.put(z_payload).wait().unwrap();
                }
            }
            DataStream::Inbound {
                iox_publisher,
                z_subscriber,
            } => {
                while let Ok(Some(z_sample)) = z_subscriber.try_recv() {
                    let z_payload = z_sample.payload();

                    // TODO: Need to divide length by payload size ...
                    unsafe {
                        let mut iox_sample =
                            iox_publisher.loan_custom_payload(z_payload.len()).unwrap();
                        std::ptr::copy_nonoverlapping(
                            z_payload.to_bytes().as_ptr(),
                            iox_sample.payload_mut().as_mut_ptr() as *mut u8,
                            z_payload.len(),
                        );
                        let iox_sample = iox_sample.assume_init();
                        iox_sample.send().unwrap();
                    }
                }
            }
        }
    }
}
