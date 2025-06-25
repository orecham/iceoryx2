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

use iceoryx2::node::NodeId as IceoryxNodeId;
use iceoryx2::port::client::Client as IceoryxClient;
use iceoryx2::port::server::Server as IceoryxServer;
use iceoryx2::service::builder::CustomHeaderMarker;
use iceoryx2::service::builder::CustomPayloadMarker;
use iceoryx2::service::port_factory::request_response::PortFactory as IceoryxRequestResponseService;
use iceoryx2::service::static_config::StaticConfig as IceoryxServiceConfig;

use zenoh::handlers::FifoChannelHandler;
use zenoh::query::Querier as ZenohQuerier;
use zenoh::query::Query;
use zenoh::query::Queryable as ZenohQueryable;
use zenoh::Session as ZenohSession;

use crate::iox_create_client;
use crate::iox_create_server;
use crate::z_create_client;
use crate::z_create_server;

use super::Connection;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CreationError {
    Error,
}

impl core::fmt::Display for CreationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        core::write!(f, "CreationError::{:?}", self)
    }
}

impl core::error::Error for CreationError {}

/// A connection for propagating `iceoryx2` request payloads to remote hosts.
pub(crate) struct ClientTunnel<'a, ServiceType: iceoryx2::service::Service> {
    iox_node_id: IceoryxNodeId,
    iox_service_config: IceoryxServiceConfig,
    iox_server: IceoryxServer<
        ServiceType,
        [CustomPayloadMarker],
        CustomHeaderMarker,
        [CustomPayloadMarker],
        CustomHeaderMarker,
    >,
    z_client: ZenohQuerier<'a>,
}

impl<ServiceType: iceoryx2::service::Service> ClientTunnel<'_, ServiceType> {
    pub fn create(
        iox_node_id: &IceoryxNodeId,
        iox_service_config: &IceoryxServiceConfig,
        iox_service: &IceoryxRequestResponseService<
            ServiceType,
            [CustomPayloadMarker],
            CustomHeaderMarker,
            [CustomPayloadMarker],
            CustomHeaderMarker,
        >,
        z_session: &ZenohSession,
    ) -> Result<Self, CreationError> {
        let iox_server = iox_create_server::<ServiceType>(iox_service, iox_service_config)
            .map_err(|_e| CreationError::Error)?;

        let z_client =
            z_create_client(z_session, iox_service_config).map_err(|_e| CreationError::Error)?;

        Ok(Self {
            iox_node_id: iox_node_id.clone(),
            iox_service_config: iox_service_config.clone(),
            iox_server,
            z_client,
        })
    }
}

impl<ServiceType: iceoryx2::service::Service> Connection for ClientTunnel<'_, ServiceType> {
    fn propagate(&self) -> Result<(), super::PropagationError> {
        Ok(())
    }
}

/// A connection for propagating `iceoryx2` request payloads from remote hosts.
pub(crate) struct ServerTunnel<ServiceType: iceoryx2::service::Service> {
    iox_node_id: IceoryxNodeId,
    iox_service_config: IceoryxServiceConfig,
    iox_client: IceoryxClient<
        ServiceType,
        [CustomPayloadMarker],
        CustomHeaderMarker,
        [CustomPayloadMarker],
        CustomHeaderMarker,
    >,
    z_server: ZenohQueryable<FifoChannelHandler<Query>>,
}

impl<ServiceType: iceoryx2::service::Service> ServerTunnel<ServiceType> {
    pub fn create(
        iox_node_id: &IceoryxNodeId,
        iox_service_config: &IceoryxServiceConfig,
        iox_service: &IceoryxRequestResponseService<
            ServiceType,
            [CustomPayloadMarker],
            CustomHeaderMarker,
            [CustomPayloadMarker],
            CustomHeaderMarker,
        >,
        z_session: &ZenohSession,
    ) -> Result<Self, CreationError> {
        let iox_client = iox_create_client(iox_service, iox_service_config)
            .map_err(|_e| CreationError::Error)?;

        let z_server =
            z_create_server(z_session, iox_service_config).map_err(|_e| CreationError::Error)?;

        Ok(Self {
            iox_node_id: iox_node_id.clone(),
            iox_service_config: iox_service_config.clone(),
            iox_client,
            z_server,
        })
    }
}

impl<ServiceType: iceoryx2::service::Service> Connection for ServerTunnel<ServiceType> {
    fn propagate(&self) -> Result<(), super::PropagationError> {
        Ok(())
    }
}
