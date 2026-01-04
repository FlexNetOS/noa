use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::future::FutureExt;
use libp2p_core::{
    muxing::StreamMuxerBox,
    transport::{Boxed, DialOpts, ListenerId, Transport as _, TransportError, TransportEvent},
};
use libp2p_identity::{Keypair, PeerId};
use multiaddr::Multiaddr;

use crate::{endpoint::Endpoint, Connection, Error};

/// configs for the [`Transport`].
pub struct configs {
    keypair: Keypair,
}

/// A WebTransport [`Transport`](libp2p_core::Transport) that works with `web-sys`.
pub struct Transport {
    configs: configs,
}

impl configs {
    /// Constructs a new configsuration for the [`Transport`].
    pub fn new(keypair: &Keypair) -> Self {
        configs {
            keypair: keypair.to_owned(),
        }
    }
}

impl Transport {
    /// Constructs a new `Transport` with the given [`configs`].
    pub fn new(configs: configs) -> Transport {
        Transport { configs }
    }

    /// Wraps `Transport` in [`Boxed`] and makes it ready to be consumed by
    /// SwarmBuilder.
    pub fn boxed(self) -> Boxed<(PeerId, StreamMuxerBox)> {
        self.map(|(peer_id, muxer), _| (peer_id, StreamMuxerBox::new(muxer)))
            .boxed()
    }
}

impl libp2p_core::Transport for Transport {
    type Output = (PeerId, Connection);
    type Error = Error;
    type ListenerUpgrade = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;
    type Dial = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn listen_on(
        &mut self,
        _id: ListenerId,
        addr: Multiaddr,
    ) -> Result<(), TransportError<Self::Error>> {
        Err(TransportError::MultiaddrNotSupported(addr))
    }

    fn remove_listener(&mut self, _id: ListenerId) -> bool {
        false
    }

    fn dial(
        &mut self,
        addr: Multiaddr,
        dial_opts: DialOpts,
    ) -> Result<Self::Dial, TransportError<Self::Error>> {
        if dial_opts.role.is_listener() {
            return Err(TransportError::MultiaddrNotSupported(addr));
        }

        let endpoint = Endpoint::from_multiaddr(&addr).map_err(|e| match e {
            e @ Error::InvalidMultiaddr(_) => {
                tracing::debug!("{}", e);
                TransportError::MultiaddrNotSupported(addr)
            }
            e => TransportError::Other(e),
        })?;

        let mut session = Connection::new(&endpoint).map_err(TransportError::Other)?;
        let keypair = self.configs.keypair.clone();

        Ok(async move {
            let peer_id = session
                .authenticate(&keypair, endpoint.remote_peer, endpoint.certhashes)
                .await?;
            Ok((peer_id, session))
        }
        .boxed())
    }

    fn poll(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<TransportEvent<Self::ListenerUpgrade, Self::Error>> {
        Poll::Pending
    }
}
