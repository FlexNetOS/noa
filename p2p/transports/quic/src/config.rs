// Copyright 2017-2020 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::{sync::Arc, time::Duration};

use quinn::{
    crypto::rustls::{QuicClientconfigs, QuicServerconfigs},
    MtuDiscoveryconfigs, VarInt,
};

/// configs for the transport.
#[derive(Clone)]
pub struct configs {
    /// Timeout for the initial handshake when establishing a connection.
    /// The actual timeout is the minimum of this and the [`configs::max_idle_timeout`].
    pub handshake_timeout: Duration,
    /// Maximum duration of inactivity in ms to accept before timing out the connection.
    pub max_idle_timeout: u32,
    /// Period of inactivity before sending a keep-alive packet.
    /// Must be set lower than the idle_timeout of both
    /// peers to be effective.
    ///
    /// See [`quinn::Transportconfigs::keep_alive_interval`] for more
    /// info.
    pub keep_alive_interval: Duration,
    /// Maximum number of incoming bidirectional streams that may be open
    /// concurrently by the remote peer.
    pub max_concurrent_stream_limit: u32,

    /// Max unacknowledged data in bytes that may be sent on a single stream.
    pub max_stream_data: u32,

    /// Max unacknowledged data in bytes that may be sent in total on all streams
    /// of a connection.
    pub max_connection_data: u32,

    /// Support QUIC version draft-29 for dialing and listening.
    ///
    /// Per default only QUIC Version 1 / [`libp2p_core::multiaddr::Protocol::QuicV1`]
    /// is supported.
    ///
    /// If support for draft-29 is enabled servers support draft-29 and version 1 on all
    /// QUIC listening addresses.
    /// As client the version is chosen based on the remote's address.
    #[deprecated(note = "QUIC draft versions are no longer supported")]
    pub support_draft_29: bool,

    /// TLS client configs for the inner [`quinn::Clientconfigs`].
    client_tls_configs: Arc<QuicClientconfigs>,
    /// TLS server configs for the inner [`quinn::Serverconfigs`].
    server_tls_configs: Arc<QuicServerconfigs>,
    /// Libp2p identity of the node.
    keypair: libp2p_identity::Keypair,

    /// Parameters governing MTU discovery. See [`MtuDiscoveryconfigs`] for details.
    mtu_discovery_configs: Option<MtuDiscoveryconfigs>,
}

#[expect(deprecated)]
impl configs {
    /// Creates a new configsuration object with default values.
    pub fn new(keypair: &libp2p_identity::Keypair) -> Self {
        let client_tls_configs = Arc::new(
            QuicClientconfigs::try_from(libp2p_tls::make_client_configs(keypair, None).unwrap())
                .unwrap(),
        );
        let server_tls_configs = Arc::new(
            QuicServerconfigs::try_from(libp2p_tls::make_server_configs(keypair).unwrap()).unwrap(),
        );
        Self {
            client_tls_configs,
            server_tls_configs,
            support_draft_29: false,
            handshake_timeout: Duration::from_secs(5),
            max_idle_timeout: 10 * 1000,
            max_concurrent_stream_limit: 256,
            keep_alive_interval: Duration::from_secs(5),
            max_connection_data: 15_000_000,

            // Ensure that one stream is not consuming the whole connection.
            max_stream_data: 10_000_000,
            keypair: keypair.clone(),
            mtu_discovery_configs: Some(Default::default()),
        }
    }

    /// Set the upper bound to the max UDP payload size that MTU discovery will search for.
    pub fn mtu_upper_bound(mut self, value: u16) -> Self {
        self.mtu_discovery_configs
            .get_or_insert_with(Default::default)
            .upper_bound(value);
        self
    }

    /// Disable MTU path discovery (it is enabled by default).
    pub fn disable_path_mtu_discovery(mut self) -> Self {
        self.mtu_discovery_configs = None;
        self
    }
}

/// Represents the inner configsuration for [`quinn`].
#[derive(Debug, Clone)]
pub(crate) struct Quinnconfigs {
    pub(crate) client_configs: quinn::Clientconfigs,
    pub(crate) server_configs: quinn::Serverconfigs,
    pub(crate) endpoint_configs: quinn::Endpointconfigs,
}

#[expect(deprecated)]
impl From<configs> for Quinnconfigs {
    fn from(configs: configs) -> Quinnconfigs {
        let configs {
            client_tls_configs,
            server_tls_configs,
            max_idle_timeout,
            max_concurrent_stream_limit,
            keep_alive_interval,
            max_connection_data,
            max_stream_data,
            support_draft_29,
            handshake_timeout: _,
            keypair,
            mtu_discovery_configs,
        } = configs;
        let mut transport = quinn::Transportconfigs::default();
        // Disable uni-directional streams.
        transport.max_concurrent_uni_streams(0u32.into());
        transport.max_concurrent_bidi_streams(max_concurrent_stream_limit.into());
        // Disable datagrams.
        transport.datagram_receive_buffer_size(None);
        transport.keep_alive_interval(Some(keep_alive_interval));
        transport.max_idle_timeout(Some(VarInt::from_u32(max_idle_timeout).into()));
        transport.allow_spin(false);
        transport.stream_receive_window(max_stream_data.into());
        transport.receive_window(max_connection_data.into());
        transport.mtu_discovery_configs(mtu_discovery_configs);
        let transport = Arc::new(transport);

        let mut server_configs = quinn::Serverconfigs::with_crypto(server_tls_configs);
        server_configs.transport = Arc::clone(&transport);
        // Disables connection migration.
        // Long-term this should be enabled, however we then need to handle address change
        // on connections in the `Connection`.
        server_configs.migration(false);

        let mut client_configs = quinn::Clientconfigs::new(client_tls_configs);
        client_configs.transport_configs(transport);

        let mut endpoint_configs = keypair
            .derive_secret(b"libp2p quic stateless reset key")
            .map(|secret| {
                let reset_key = Arc::new(ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &secret));
                quinn::Endpointconfigs::new(reset_key)
            })
            .unwrap_or_default();

        if !support_draft_29 {
            endpoint_configs.supported_versions(vec![1]);
        }

        Quinnconfigs {
            client_configs,
            server_configs,
            endpoint_configs,
        }
    }
}
