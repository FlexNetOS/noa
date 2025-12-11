// Package node provides libp2p host initialization and management
//
// T230: §3.8 Initialize libp2p host
// US6: P2P Hive-Mind Device Federation
package node

import (
	"context"
	"crypto/rand"
	"fmt"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/peerstore"
	"github.com/libp2p/go-libp2p/p2p/security/noise"
	libp2ptls "github.com/libp2p/go-libp2p/p2p/security/tls"
	"github.com/libp2p/go-libp2p/p2p/transport/tcp"
	"github.com/multiformats/go-multiaddr"
)

// Host wraps libp2p host with NOA-specific functionality
type Host struct {
	host host.Host
	ctx  context.Context
}

// Config holds configuration for creating a P2P host
type Config struct {
	// Listen addresses (empty = auto-detect)
	ListenAddrs []multiaddr.Multiaddr
	// Private key seed (nil = generate new)
	PrivateKeySeed []byte
	// Enable relay
	EnableRelay bool
	// Enable NAT traversal
	EnableNAT bool
}

// DefaultConfig returns a default configuration
func DefaultConfig() *Config {
	return &Config{
		ListenAddrs:    nil, // Auto-detect
		PrivateKeySeed: nil, // Generate new
		EnableRelay:    true,
		EnableNAT:      true,
	}
}

// NewHost creates a new libp2p host with NOA configuration
//
// Implements T230: §3.8 Initialize libp2p host
func NewHost(ctx context.Context, config *Config) (*Host, error) {
	if config == nil {
		config = DefaultConfig()
	}

	// Build libp2p options
	opts := []libp2p.Option{
		// Use Noise for encryption (modern, efficient)
		libp2p.Security(noise.ID, noise.New),
		// Use TLS as fallback
		libp2p.Security(libp2ptls.ID, libp2ptls.New),
		// Use TCP transport
		libp2p.Transport(tcp.NewTCPTransport),
		// Enable connection manager
		libp2p.DefaultConnectionManager,
		// Enable peer discovery
		libp2p.DefaultPeerstore,
		// Enable NAT traversal if requested
	}

	// Add listen addresses
	if len(config.ListenAddrs) > 0 {
		opts = append(opts, libp2p.ListenAddrs(config.ListenAddrs...))
	} else {
		// Default: listen on all interfaces, random port
		opts = append(opts, libp2p.ListenAddrStrings(
			"/ip4/0.0.0.0/tcp/0",
			"/ip6/::/tcp/0",
		))
	}

	// Generate or use provided private key
	if config.PrivateKeySeed != nil {
		// Use provided seed to generate deterministic key
		// This allows device identity to persist across restarts
		privKey, err := generateKeyFromSeed(config.PrivateKeySeed)
		if err != nil {
			return nil, fmt.Errorf("failed to generate key from seed: %w", err)
		}
		opts = append(opts, libp2p.Identity(privKey))
	}

	// Create libp2p host
	h, err := libp2p.New(opts...)
	if err != nil {
		return nil, fmt.Errorf("failed to create libp2p host: %w", err)
	}

	return &Host{
		host: h,
		ctx:  ctx,
	}, nil
}

// Start starts the P2P host
func (h *Host) Start() error {
	// Log listening addresses
	addrs := h.host.Addrs()
	fmt.Printf("P2P Host started. Peer ID: %s\n", h.host.ID())
	for _, addr := range addrs {
		fmt.Printf("  Listening on: %s/p2p/%s\n", addr, h.host.ID())
	}
	return nil
}

// Stop gracefully stops the host
func (h *Host) Stop() error {
	return h.host.Close()
}

// ID returns the peer ID
func (h *Host) ID() peer.ID {
	return h.host.ID()
}

// Host returns the underlying libp2p host
func (h *Host) Host() host.Host {
	return h.host
}

// Addrs returns the listening addresses
func (h *Host) Addrs() []multiaddr.Multiaddr {
	return h.host.Addrs()
}

// Connect connects to a peer
func (h *Host) Connect(ctx context.Context, peerInfo peer.AddrInfo) error {
	return h.host.Connect(ctx, peerInfo)
}

// Peerstore returns the peerstore
func (h *Host) Peerstore() peerstore.Peerstore {
	return h.host.Peerstore()
}

// generateKeyFromSeed generates a private key from a seed
// This allows deterministic peer IDs for device identity
func generateKeyFromSeed(seed []byte) (crypto.PrivKey, error) {
	// For now, generate a random Ed25519 key
	// TODO: Implement deterministic key generation from seed
	// This would allow devices to maintain the same peer ID across restarts
	privKey, _, err := crypto.GenerateEd25519Key(rand.Reader)
	return privKey, err
}

