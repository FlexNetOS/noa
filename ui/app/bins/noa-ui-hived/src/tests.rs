//! Tests for noa-ui-hived daemon.

#[cfg(test)]
mod cli_tests {
    use clap::Parser;
    use crate::cli::{Cli, Command};

    #[test]
    fn test_cli_start_default() {
        let args = vec!["noa-hived", "start"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Command::Start { port, data_dir } => {
                assert_eq!(port, 9999);
                assert!(data_dir.is_none());
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_start_with_port() {
        let args = vec!["noa-hived", "start", "--port", "8888"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Command::Start { port, .. } => {
                assert_eq!(port, 8888);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_status() {
        let args = vec!["noa-hived", "status"];
        let cli = Cli::parse_from(args);
        
        assert!(matches!(cli.command, Command::Status));
    }

    #[test]
    fn test_cli_stop() {
        let args = vec!["noa-hived", "stop"];
        let cli = Cli::parse_from(args);
        
        assert!(matches!(cli.command, Command::Stop));
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::{DaemonConfig, P2pConfig, StateConfig};
    use std::path::PathBuf;

    #[test]
    fn test_config_creation() {
        let config = DaemonConfig::new(9999, Some(PathBuf::from("/tmp/noa-test")));
        
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.port, 9999);
    }

    #[test]
    fn test_p2p_config_defaults() {
        let config = P2pConfig {
            enabled: true,
            port: 0,
            bootstrap_peers: vec![],
            mdns: true,
        };
        
        assert!(config.enabled);
        assert!(config.mdns);
        assert!(config.bootstrap_peers.is_empty());
    }

    #[test]
    fn test_state_config_defaults() {
        let config = StateConfig {
            enabled: true,
            sync_interval_secs: 30,
            max_state_size: 10 * 1024 * 1024,
        };
        
        assert!(config.enabled);
        assert_eq!(config.sync_interval_secs, 30);
        assert_eq!(config.max_state_size, 10 * 1024 * 1024);
    }
}

#[cfg(test)]
mod state_tests {
    use crate::state::{DaemonState, PeerInfo, AgentState, AgentStatus};

    #[test]
    fn test_daemon_state_default() {
        let state = DaemonState::default();
        
        assert_eq!(state.version, 0);
        assert!(state.peers.is_empty());
        assert!(state.documents.is_empty());
        assert!(state.agents.is_empty());
        assert!(state.last_sync.is_none());
    }

    #[test]
    fn test_peer_info() {
        let peer = PeerInfo {
            id: "peer-001".to_string(),
            address: "/ip4/127.0.0.1/tcp/9999".to_string(),
            connected_at: 1704067200,
            last_seen: 1704067300,
        };
        
        assert_eq!(peer.id, "peer-001");
        assert!(peer.last_seen >= peer.connected_at);
    }

    #[test]
    fn test_agent_state() {
        let agent = AgentState {
            id: "agent-planner".to_string(),
            name: "Planner Agent".to_string(),
            status: AgentStatus::Running,
            last_heartbeat: 1704067200,
        };
        
        assert_eq!(agent.id, "agent-planner");
        assert!(matches!(agent.status, AgentStatus::Running));
    }

    #[test]
    fn test_agent_status_variants() {
        let statuses = vec![
            AgentStatus::Idle,
            AgentStatus::Running,
            AgentStatus::Paused,
            AgentStatus::Error,
        ];
        
        assert_eq!(statuses.len(), 4);
    }

    #[test]
    fn test_state_with_peers() {
        let mut state = DaemonState::default();
        
        state.peers.insert("peer-1".to_string(), PeerInfo {
            id: "peer-1".to_string(),
            address: "/ip4/192.168.1.1/tcp/9999".to_string(),
            connected_at: 1704067200,
            last_seen: 1704067200,
        });
        
        assert_eq!(state.peers.len(), 1);
        assert!(state.peers.contains_key("peer-1"));
    }
}
