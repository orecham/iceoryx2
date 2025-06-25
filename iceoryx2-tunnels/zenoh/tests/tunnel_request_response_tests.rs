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

mod testing;

#[generic_tests::define]
mod zenoh_tunnel_request_response {

    use std::time::Duration;

    use iceoryx2::prelude::*;
    use iceoryx2::service::static_config::StaticConfig;
    use iceoryx2::testing::*;
    use iceoryx2_bb_posix::unique_system_id::UniqueSystemId;
    use iceoryx2_bb_testing::{assert_that, test_fail};
    use iceoryx2_services_discovery::service_discovery::Config as DiscoveryConfig;
    use iceoryx2_services_discovery::service_discovery::Service as DiscoveryService;
    use iceoryx2_tunnels_zenoh::*;

    use zenoh::Wait;

    fn mock_service_name() -> ServiceName {
        ServiceName::new(&format!(
            "test_zenoh_tunnel_request_response_{}",
            UniqueSystemId::new().unwrap().value()
        ))
        .unwrap()
    }

    #[test]
    fn discovers_local_services_via_discovery_service<S: Service>() {
        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();
        let iox_config = generate_isolated_config();

        // [[ DISCOVERY SERVICE ]]
        let discovery_config = DiscoveryConfig {
            publish_events: true,
            include_internal: false,

            ..Default::default()
        };
        let mut discovery_service =
            DiscoveryService::<S>::create(&discovery_config, &iox_config).unwrap();

        // [[ HOST A ]]
        // Tunnel
        let z_config_a = zenoh::Config::default();
        let tunnel_config = TunnelConfig {
            discovery_service: Some("iox2://discovery/services/".into()),
        };

        let mut tunnel = Tunnel::<S>::create(&tunnel_config, &iox_config, &z_config_a).unwrap();
        assert_that!(tunnel.tunneled_services().len(), eq 0);

        // Service
        let iox_node = NodeBuilder::new()
            .config(&iox_config)
            .create::<S>()
            .unwrap();
        let iox_service = iox_node
            .service_builder(&iox_service_name)
            .request_response::<(), ()>()
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // [[ DISCOVERY SERVICE ]]
        // Discover
        discovery_service.spin(|_| {}, |_| {}).unwrap();

        // [[ HOST A ]]
        // Respond to discovered services
        tunnel.discover(Scope::Iceoryx).unwrap();
        assert_that!(tunnel.tunneled_ports().len(), eq 2);
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Server(String::from(iox_service.service_id().as_str()))),
            eq true
        );
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Client(String::from(iox_service.service_id().as_str()))),
            eq true
        );
    }

    #[test]
    fn discovers_local_services_via_tracker<S: Service>() {
        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let z_config = zenoh::Config::default();
        let iox_config = generate_isolated_config();
        let tunnel_config = TunnelConfig::default();
        let mut tunnel = Tunnel::<S>::create(&tunnel_config, &iox_config, &z_config).unwrap();
        assert_that!(tunnel.tunneled_services().len(), eq 0);

        // Service
        let iox_node = NodeBuilder::new()
            .config(&iox_config)
            .create::<S>()
            .unwrap();
        let iox_service = iox_node
            .service_builder(&iox_service_name)
            .request_response::<(), ()>()
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // [[ HOST A ]]
        // Discover
        tunnel.discover(Scope::Iceoryx).unwrap();
        assert_that!(tunnel.tunneled_ports().len(), eq 2);
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Server(String::from(iox_service.service_id().as_str()))),
            eq true
        );
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Client(String::from(iox_service.service_id().as_str()))),
            eq true
        );
    }

    #[test]
    fn announces_service_details_on_zenoh<S: Service>() {
        // ==================== SETUP ====================

        // [[ COMMON ]]
        let iox_service_name = mock_service_name();

        // [[ HOST A ]]
        // Tunnel
        let iox_config = generate_isolated_config();
        let z_config = zenoh::Config::default();
        let tunnel_config = TunnelConfig::default();

        let mut tunnel = Tunnel::<S>::create(&tunnel_config, &iox_config, &z_config).unwrap();

        // Service
        let iox_node = NodeBuilder::new()
            .config(&iox_config)
            .create::<S>()
            .unwrap();
        let iox_service = iox_node
            .service_builder(&iox_service_name)
            .request_response::<(), ()>()
            .open_or_create()
            .unwrap();

        // ==================== TEST =====================

        // Discover
        tunnel.discover(Scope::Iceoryx).unwrap();
        assert_that!(tunnel.tunneled_ports().len(), eq 2);
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Server(String::from(iox_service.service_id().as_str()))),
            eq true
        );
        assert_that!(
            tunnel.tunneled_ports().contains(&TunneledPort::Client(String::from(iox_service.service_id().as_str()))),
            eq true
        );

        // Query Zenoh for Services
        let z_config = zenoh::config::Config::default();
        let z_session = zenoh::open(z_config.clone()).wait().unwrap();
        let z_reply = z_session
            .get(keys::service_details(iox_service.service_id()))
            .wait()
            .unwrap();
        match z_reply.recv_timeout(Duration::from_millis(100)) {
            Ok(Some(reply)) => match reply.result() {
                Ok(sample) => {
                    let iox_static_details: StaticConfig =
                        serde_json::from_slice(&sample.payload().to_bytes()).unwrap();
                    assert_that!(iox_static_details.service_id(), eq iox_service.service_id());
                    assert_that!(iox_static_details.name(), eq & iox_service_name);
                    assert_that!(iox_static_details.request_response(), eq iox_service.static_config());
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
