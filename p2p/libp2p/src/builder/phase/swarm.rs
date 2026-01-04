#[allow(unused_imports)]
use super::*;

#[allow(unused)] // used below but due to feature flag combinations, clippy gives an unnecessary warning.
const DEFAULT_CONNECTION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[allow(dead_code)]
pub struct SwarmPhase<T, B> {
    pub(crate) behaviour: B,
    pub(crate) transport: T,
}

macro_rules! impl_with_swarm_configs {
    ($providerKebabCase:literal, $providerPascalCase:ty, $configs:expr) => {
        #[cfg(feature = $providerKebabCase)]
        impl<T, B> SwarmBuilder<$providerPascalCase, SwarmPhase<T, B>> {
            pub fn with_swarm_configs(
                self,
                constructor: impl FnOnce(libp2p_swarm::configs) -> libp2p_swarm::configs,
            ) -> SwarmBuilder<$providerPascalCase, BuildPhase<T, B>> {
                SwarmBuilder {
                    phase: BuildPhase {
                        behaviour: self.phase.behaviour,
                        transport: self.phase.transport,
                        swarm_configs: constructor($configs),
                        connection_timeout: DEFAULT_CONNECTION_TIMEOUT,
                    },
                    keypair: self.keypair,
                    phantom: std::marker::PhantomData,
                }
            }

            // Shortcuts
            pub fn build(self) -> libp2p_swarm::Swarm<B>
            where
                B: libp2p_swarm::NetworkBehaviour,
                T: AuthenticatedMultiplexedTransport,
            {
                self.with_swarm_configs(std::convert::identity).build()
            }
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
impl_with_swarm_configs!(
    "tokio",
    super::provider::Tokio,
    libp2p_swarm::configs::with_tokio_executor()
);

#[cfg(target_arch = "wasm32")]
impl_with_swarm_configs!(
    "wasm-bindgen",
    super::provider::WasmBindgen,
    libp2p_swarm::configs::with_wasm_executor()
);
