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

#[generic_tests::define]
mod zenoh_tunnel {

    use std::time::Duration;

    use iceoryx2::prelude::*;
    use iceoryx2::service::static_config::StaticConfig;
    use iceoryx2::testing::*;
    use iceoryx2_bb_posix::unique_system_id::UniqueSystemId;
    use iceoryx2_bb_testing::{assert_that, test_fail};
    use iceoryx2_tunnels_zenoh::*;

    use zenoh::Wait;

    fn mock_service_name() -> ServiceName {
        ServiceName::new(&format!(
            "test_tunnel_zenoh_{}",
            UniqueSystemId::new().unwrap().value()
        ))
        .unwrap()
    }

    #[test]
    fn discovers_local_services<S: Service>() {
        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config = generate_isolated_config();
        let tunnel_config = TunnelConfig::default();
        let mut tunnel = Tunnel::<S>::new(&tunnel_config, &iox_config);
        tunnel.initialize();
        assert_that!(tunnel.tunneled_services().len(), eq 0);

        // Service
        let iox_node = NodeBuilder::new()
            .config(&iox_config)
            .create::<S>()
            .unwrap();
        let iox_service = iox_node
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .history_size(10)
            .subscriber_max_buffer_size(10)
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // [[ HOST A ]]
        // Discover Services
        tunnel.discover();
        assert_that!(tunnel.tunneled_services().len(), eq 1);
        assert_that!(tunnel
            .tunneled_services()
            .contains(&String::from(iox_service.service_id().as_str())), eq true);
    }

    #[test]
    fn discovers_remote_services<S: Service>() {
        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config_a = generate_isolated_config();
        let tunnel_config_a = TunnelConfig::default();
        let mut tunnel_a = Tunnel::<S>::new(&tunnel_config_a, &iox_config_a);
        tunnel_a.initialize();
        assert_that!(tunnel_a.tunneled_services().len(), eq 0);

        // [[ HOST B ]]
        // Tunnel
        let iox_config_b = generate_isolated_config();
        let tunnel_config_b = TunnelConfig::default();
        let mut tunnel_b = Tunnel::<S>::new(&tunnel_config_b, &iox_config_b);
        tunnel_b.initialize();
        assert_that!(tunnel_b.tunneled_services().len(), eq 0);

        // Service
        let iox_node_b = NodeBuilder::new()
            .config(&iox_config_b)
            .create::<S>()
            .unwrap();
        let iox_service_b = iox_node_b
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .history_size(10)
            .subscriber_max_buffer_size(10)
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // [[ HOST A ]]
        // Discover Services - Nothing should be discovered
        tunnel_a.discover();
        assert_that!(tunnel_a.tunneled_services().len(), eq 0);

        // [[ HOST B ]]
        // Discover Services - Service should be announced
        tunnel_b.discover();
        assert_that!(tunnel_b.tunneled_services().len(), eq 1);
        assert_that!(tunnel_b
            .tunneled_services()
            .contains(&String::from(iox_service_b.service_id().as_str())), eq true);

        // [[ HOST A ]]
        // Discover Services - Announced service should be discovered
        let mut success = false;
        for _ in 0..3 {
            tunnel_a.discover();
            if tunnel_a.tunneled_services().len() == 1 {
                success = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        if !success {
            test_fail!("failed to discover remote service after 3 attempts");
        }
    }

    fn propagates_struct_payload_n_samples<S: Service>(sample_count: usize) {
        #[derive(Debug, Clone, PartialEq, ZeroCopySend)]
        #[repr(C)]
        struct MyType {
            id: u32,
            value: f64,
            active: bool,
        }

        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config_a = generate_isolated_config();
        let tunnel_config_a = TunnelConfig::default();
        let mut tunnel_a = Tunnel::<S>::new(&tunnel_config_a, &iox_config_a);
        tunnel_a.initialize();
        assert_that!(tunnel_a.tunneled_services().len(), eq 0);

        // Service
        let iox_node_a = NodeBuilder::new()
            .config(&iox_config_a)
            .create::<S>()
            .unwrap();
        let iox_service_a = iox_node_a
            .service_builder(&iox_service_name)
            .publish_subscribe::<MyType>()
            .open_or_create()
            .unwrap();

        // Publisher
        let iox_publisher_a = iox_service_a.publisher_builder().create().unwrap();

        // [[ HOST B ]]
        // Tunnel
        let iox_config_b = generate_isolated_config();
        let tunnel_config_b = TunnelConfig::default();
        let mut tunnel_b = Tunnel::<S>::new(&tunnel_config_b, &iox_config_b);
        tunnel_b.initialize();
        assert_that!(tunnel_b.tunneled_services().len(), eq 0);

        // Service
        let iox_node_b = NodeBuilder::new()
            .config(&iox_config_b)
            .create::<S>()
            .unwrap();
        let iox_service_b = iox_node_b
            .service_builder(&iox_service_name)
            .publish_subscribe::<MyType>()
            .open_or_create()
            .unwrap();

        // Subscriber
        let iox_subscriber_b = iox_service_b.subscriber_builder().create().unwrap();

        // [[ BOTH ]]
        // Discover Services
        tunnel_a.discover();
        let tunneled_services_a = tunnel_a.tunneled_services();
        assert_that!(tunneled_services_a.len(), eq 1);
        assert_that!(tunneled_services_a
            .contains(&String::from(iox_service_a.service_id().as_str())), eq true);

        tunnel_b.discover();
        let tunneled_services_b = tunnel_b.tunneled_services();
        assert_that!(tunneled_services_b.len(), eq 1);
        assert_that!(tunneled_services_b
            .contains(&String::from(iox_service_b.service_id().as_str())), eq true);

        // Discovered service should be the same ID in both hosts
        assert_that!(iox_service_a.service_id(), eq iox_service_b.service_id());

        // ==================== TEST =====================

        for i in 0..sample_count {
            // Publish
            let payload_data = MyType {
                id: 42 + i as u32,
                value: 3.14 + i as f64,
                active: i % 2 == 0,
            };

            let iox_sample_sent_a = iox_publisher_a.loan_uninit().unwrap();
            let iox_sample_sent_a = iox_sample_sent_a.write_payload(payload_data.clone());
            iox_sample_sent_a.send().unwrap();

            // Propagate over tunnels
            tunnel_a.propagate();
            tunnel_b.propagate();

            // Receive with retry
            let mut success = false;
            for retry in 0..3 {
                match iox_subscriber_b.receive().unwrap() {
                    Some(iox_sample_received_b) => {
                        let iox_payload_received_b = iox_sample_received_b.payload();

                        // Check if we received the expected sample for this iteration
                        if *iox_payload_received_b == payload_data {
                            success = true;
                            break;
                        } else {
                            test_fail!(
                                "received unexpected sample; expected: {:?}, got: {:?}",
                                payload_data,
                                iox_payload_received_b
                            );
                        }
                    }
                    None => {
                        // If no sample received, wait a bit and retry
                        // Don't sleep after last attempt
                        if retry < 2 {
                            std::thread::sleep(Duration::from_millis(100));
                            tunnel_a.propagate();
                            tunnel_b.propagate();
                        }
                    }
                }
            }

            if !success {
                test_fail!("failed to receive expected sample {} after 3 attempts", i);
            }
        }
    }

    #[test]
    fn propagates_struct_payload_single_sample<S: Service>() {
        propagates_struct_payload_n_samples::<S>(1);
    }

    #[test]
    fn propagates_struct_payload_two_samples<S: Service>() {
        propagates_struct_payload_n_samples::<S>(2);
    }

    #[test]
    fn propagates_struct_payload_ten_samples<S: Service>() {
        propagates_struct_payload_n_samples::<S>(10);
    }

    fn propagates_slice_payload_n_samples<S: Service>(sample_count: usize) {
        const PAYLOAD_DATA_LENGTH: usize = 256;

        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config_a = generate_isolated_config();
        let tunnel_config_a = TunnelConfig::default();
        let mut tunnel_a = Tunnel::<S>::new(&tunnel_config_a, &iox_config_a);
        tunnel_a.initialize();
        assert_that!(tunnel_a.tunneled_services().len(), eq 0);

        // Service
        let iox_node_a = NodeBuilder::new()
            .config(&iox_config_a)
            .create::<S>()
            .unwrap();
        let iox_service_a = iox_node_a
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .open_or_create()
            .unwrap();

        // Publisher
        let iox_publisher_a = iox_service_a
            .publisher_builder()
            .initial_max_slice_len(PAYLOAD_DATA_LENGTH)
            .create()
            .unwrap();

        // [[ HOST B ]]
        // Tunnel
        let iox_config_b = generate_isolated_config();
        let tunnel_config_b = TunnelConfig::default();
        let mut tunnel_b = Tunnel::<S>::new(&tunnel_config_b, &iox_config_b);
        tunnel_b.initialize();
        assert_that!(tunnel_b.tunneled_services().len(), eq 0);

        // Service
        let iox_node_b = NodeBuilder::new()
            .config(&iox_config_b)
            .create::<S>()
            .unwrap();
        let iox_service_b = iox_node_b
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .open_or_create()
            .unwrap();

        // Subscriber
        let iox_subscriber_b = iox_service_b.subscriber_builder().create().unwrap();

        // [[ BOTH ]]
        // Discover Services
        tunnel_a.discover();
        let tunneled_services_a = tunnel_a.tunneled_services();
        assert_that!(tunneled_services_a.len(), eq 1);
        assert_that!(tunneled_services_a
            .contains(&String::from(iox_service_a.service_id().as_str())), eq true);

        tunnel_b.discover();
        let tunneled_services_b = tunnel_b.tunneled_services();
        assert_that!(tunneled_services_b.len(), eq 1);
        assert_that!(tunneled_services_b
            .contains(&String::from(iox_service_b.service_id().as_str())), eq true);

        // Discovered service should be the same ID in both hosts
        assert_that!(iox_service_a.service_id(), eq iox_service_b.service_id());

        // ==================== TEST =====================

        for i in 0..sample_count {
            // Publish
            let mut payload_data = String::with_capacity(PAYLOAD_DATA_LENGTH);
            for j in 0..PAYLOAD_DATA_LENGTH {
                let char_index = ((i * 7 + j * 13) % 26) as u8;
                let char_value = (b'A' + char_index) as char;
                payload_data.push(char_value);
            }

            let iox_sample_sent_a = iox_publisher_a
                .loan_slice_uninit(PAYLOAD_DATA_LENGTH)
                .unwrap();
            let iox_sample_sent_a = iox_sample_sent_a.write_from_slice(payload_data.as_bytes());
            iox_sample_sent_a.send().unwrap();

            // Propagate
            tunnel_a.propagate();
            tunnel_b.propagate();

            // Receive with retry
            let mut success = false;
            for retry in 0..3 {
                match iox_subscriber_b.receive().unwrap() {
                    Some(iox_sample_received_b) => {
                        let iox_payload_received_b = iox_sample_received_b.payload();

                        // Check if we received the expected sample for this iteration
                        if *iox_payload_received_b == *payload_data.as_bytes() {
                            success = true;
                            break;
                        } else {
                            test_fail!(
                                "received unexpected sample; expected: {:?}, got: {:?}",
                                payload_data,
                                iox_payload_received_b
                            );
                        }
                    }
                    None => {
                        // If no sample received, wait a bit and retry
                        // Don't sleep after last attempt
                        if retry < 2 {
                            std::thread::sleep(Duration::from_millis(100));
                            tunnel_a.propagate();
                            tunnel_b.propagate();
                        }
                    }
                }
            }

            if !success {
                test_fail!("failed to receive expected sample {} after 3 attempts", i);
            }
        }
    }

    #[test]
    fn propagates_slice_payload_single_sample<S: Service>() {
        propagates_slice_payload_n_samples::<S>(1);
    }

    #[test]
    fn propagates_slice_payload_two_samples<S: Service>() {
        propagates_slice_payload_n_samples::<S>(2);
    }

    #[test]
    fn propagates_slice_payload_ten_samples<S: Service>() {
        propagates_slice_payload_n_samples::<S>(10);
    }

    #[test]
    fn propagated_payloads_do_not_loop_back_from_zenoh<S: Service>() {
        const PAYLOAD_DATA: &str = "WhenItRegisters";

        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config_a = generate_isolated_config();
        let tunnel_config_a = TunnelConfig::default();
        let mut tunnel_a = Tunnel::<S>::new(&tunnel_config_a, &iox_config_a);
        tunnel_a.initialize();

        // Service
        let iox_node_a = NodeBuilder::new()
            .config(&iox_config_a)
            .create::<S>()
            .unwrap();
        let iox_service_a = iox_node_a
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .open_or_create()
            .unwrap();

        // Publisher
        let iox_publisher_a = iox_service_a
            .publisher_builder()
            .initial_max_slice_len(PAYLOAD_DATA.len())
            .create()
            .unwrap();

        // Subscriber
        let iox_subscriber_a = iox_service_a.subscriber_builder().create().unwrap();

        // Discover Services
        tunnel_a.discover();
        let tunneled_services_a = tunnel_a.tunneled_services();
        assert_that!(tunneled_services_a.len(), eq 1);
        assert_that!(tunneled_services_a
            .contains(&String::from(iox_service_a.service_id().as_str())), eq true);

        // ==================== TEST =====================

        // [[ HOST A ]]
        // Publish
        let iox_sample_a = iox_publisher_a
            .loan_slice_uninit(PAYLOAD_DATA.len())
            .unwrap();
        let iox_sample_a = iox_sample_a.write_from_slice(PAYLOAD_DATA.as_bytes());
        iox_sample_a.send().unwrap();

        // Receive - Sample should be received from local publisher
        while let Ok(Some(_)) = iox_subscriber_a.receive() {}

        // Propagate
        tunnel_a.propagate();

        // Receive - Sample should not loop back and be received again
        if iox_subscriber_a.receive().unwrap().is_some() {
            test_fail!("sample looped back")
        }
    }

    #[test]
    fn announces_service_details_on_zenoh<S: Service>() {
        let iox_config = generate_isolated_config();
        let tunnel_config = TunnelConfig::default();

        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let mut tunnel_a = Tunnel::<S>::new(&tunnel_config, &iox_config);
        tunnel_a.initialize();

        // Service
        let iox_node = NodeBuilder::new()
            .config(&iox_config)
            .create::<S>()
            .unwrap();
        let iox_service_a = iox_node
            .service_builder(&iox_service_name)
            .publish_subscribe::<[u8]>()
            .history_size(10)
            .subscriber_max_buffer_size(10)
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // Discover Services
        tunnel_a.discover();
        let tunneled_services_a = tunnel_a.tunneled_services();
        assert_that!(tunneled_services_a.len(), eq 1);
        assert_that!(tunneled_services_a
            .contains(&String::from(iox_service_a.service_id().as_str())), eq true);

        // Query Zenoh for Services
        let z_config = zenoh::config::Config::default();
        let z_session = zenoh::open(z_config.clone()).wait().unwrap();
        let z_reply = z_session
            .get(keys::service(iox_service_a.service_id()))
            .wait()
            .unwrap();
        match z_reply.recv_timeout(Duration::from_millis(100)) {
            Ok(Some(reply)) => match reply.result() {
                Ok(sample) => {
                    let iox_static_details: StaticConfig =
                        serde_json::from_slice(&sample.payload().to_bytes()).unwrap();
                    assert_that!(iox_static_details.service_id(), eq iox_service_a.service_id());
                    assert_that!(iox_static_details.name(), eq & iox_service_name);
                    assert_that!(iox_static_details.publish_subscribe(), eq iox_service_a.static_config());
                }
                Err(e) => test_fail!("error reading reply to type details query: {}", e),
            },
            Ok(None) => test_fail!("no reply to type details query"),
            Err(e) => test_fail!("error querying message type details from zenoh: {}", e),
        }
    }

    #[instantiate_tests(<iceoryx2::service::ipc::Service>)]
    mod ipc {}

    #[instantiate_tests(<iceoryx2::service::local::Service>)]
    mod local {}
}
