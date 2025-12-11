// Package node provides peer connection management
//
// T234: §3.8 Implement peer connection management
// US6: P2P Hive-Mind Device Federation
package node

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/network"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/peerstore"
)

const (
	// DefaultConnectionTimeout is the default timeout for connections
	DefaultConnectionTimeout = 30 * time.Second
	// DefaultKeepAliveInterval is how often to ping peers
	DefaultKeepAliveInterval = 60 * time.Second
)

// PeerManager manages peer connections and state
type PeerManager struct {
	host      host.Host
	peers     map[peer.ID]*PeerInfo
	mu        sync.RWMutex
	ctx       context.Context
	onConnect func(peer.ID)
	onDisconnect func(peer.ID)
}

// PeerInfo holds information about a connected peer
type PeerInfo struct {
	ID          peer.ID
	Addrs       []string
	ConnectedAt time.Time
	LastSeen    time.Time
	State       ConnectionState
	mu          sync.RWMutex
}

// ConnectionState represents the connection state
type ConnectionState int

const (
	StateDisconnected ConnectionState = iota
	StateConnecting
	StateConnected
	StateDisconnecting
)

func (s ConnectionState) String() string {
	switch s {
	case StateDisconnected:
		return "disconnected"
	case StateConnecting:
		return "connecting"
	case StateConnected:
		return "connected"
	case StateDisconnecting:
		return "disconnecting"
	default:
		return "unknown"
	}
}

// NewPeerManager creates a new peer manager
//
// Implements T234: §3.8 Implement peer connection management
func NewPeerManager(ctx context.Context, h host.Host) *PeerManager {
	pm := &PeerManager{
		host:  h,
		peers: make(map[peer.ID]*PeerInfo),
		ctx:   ctx,
	}

	// Set up network notifiers
	h.Network().Notify(&network.NotifyBundle{
		ConnectedF: func(n network.Network, c network.Conn) {
			pm.handleConnected(c.RemotePeer())
		},
		DisconnectedF: func(n network.Network, c network.Conn) {
			pm.handleDisconnected(c.RemotePeer())
		},
	})

	return pm
}

// SetOnConnect sets the callback for when a peer connects
func (pm *PeerManager) SetOnConnect(fn func(peer.ID)) {
	pm.mu.Lock()
	defer pm.mu.Unlock()
	pm.onConnect = fn
}

// SetOnDisconnect sets the callback for when a peer disconnects
func (pm *PeerManager) SetOnDisconnect(fn func(peer.ID)) {
	pm.mu.Lock()
	defer pm.mu.Unlock()
	pm.onDisconnect = fn
}

// Connect connects to a peer
func (pm *PeerManager) Connect(ctx context.Context, peerInfo peer.AddrInfo) error {
	pm.mu.Lock()
	info, exists := pm.peers[peerInfo.ID]
	if !exists {
		info = &PeerInfo{
			ID:    peerInfo.ID,
			State: StateDisconnected,
		}
		pm.peers[peerInfo.ID] = info
	}
	info.State = StateConnecting
	pm.mu.Unlock()

	// Add addresses to peerstore
	pm.host.Peerstore().AddAddrs(peerInfo.ID, peerInfo.Addrs, peerstore.PermanentAddrTTL)

	// Connect with timeout
	ctx, cancel := context.WithTimeout(ctx, DefaultConnectionTimeout)
	defer cancel()

	if err := pm.host.Connect(ctx, peerInfo); err != nil {
		pm.mu.Lock()
		info.State = StateDisconnected
		pm.mu.Unlock()
		return fmt.Errorf("failed to connect to peer %s: %w", peerInfo.ID, err)
	}

	pm.mu.Lock()
	info.State = StateConnected
	info.ConnectedAt = time.Now()
	info.LastSeen = time.Now()
	pm.mu.Unlock()

	return nil
}

// Disconnect disconnects from a peer
func (pm *PeerManager) Disconnect(peerID peer.ID) error {
	pm.mu.Lock()
	info, exists := pm.peers[peerID]
	if exists {
		info.State = StateDisconnecting
	}
	pm.mu.Unlock()

	if err := pm.host.Network().ClosePeer(peerID); err != nil {
		return fmt.Errorf("failed to disconnect from peer %s: %w", peerID, err)
	}

	pm.mu.Lock()
	if info != nil {
		info.State = StateDisconnected
	}
	pm.mu.Unlock()

	return nil
}

// GetPeer returns information about a peer
func (pm *PeerManager) GetPeer(peerID peer.ID) (*PeerInfo, bool) {
	pm.mu.RLock()
	defer pm.mu.RUnlock()
	info, exists := pm.peers[peerID]
	if !exists {
		return nil, false
	}
	return info, true
}

// ListPeers returns all known peers
func (pm *PeerManager) ListPeers() []*PeerInfo {
	pm.mu.RLock()
	defer pm.mu.RUnlock()
	peers := make([]*PeerInfo, 0, len(pm.peers))
	for _, info := range pm.peers {
		peers = append(peers, info)
	}
	return peers
}

// ConnectedPeers returns all currently connected peers
func (pm *PeerManager) ConnectedPeers() []peer.ID {
	pm.mu.RLock()
	defer pm.mu.RUnlock()
	var connected []peer.ID
	for id, info := range pm.peers {
		if info.State == StateConnected {
			connected = append(connected, id)
		}
	}
	return connected
}

// UpdateLastSeen updates the last seen timestamp for a peer
func (pm *PeerManager) UpdateLastSeen(peerID peer.ID) {
	pm.mu.Lock()
	defer pm.mu.Unlock()
	if info, exists := pm.peers[peerID]; exists {
		info.LastSeen = time.Now()
	}
}

// handleConnected is called when a peer connects
func (pm *PeerManager) handleConnected(peerID peer.ID) {
	pm.mu.Lock()
	info, exists := pm.peers[peerID]
	if !exists {
		info = &PeerInfo{
			ID:    peerID,
			State: StateConnected,
		}
		pm.peers[peerID] = info
	}
	info.State = StateConnected
	info.ConnectedAt = time.Now()
	info.LastSeen = time.Now()
	onConnect := pm.onConnect
	pm.mu.Unlock()

	if onConnect != nil {
		onConnect(peerID)
	}
}

// handleDisconnected is called when a peer disconnects
func (pm *PeerManager) handleDisconnected(peerID peer.ID) {
	pm.mu.Lock()
	info, exists := pm.peers[peerID]
	if exists {
		info.State = StateDisconnected
	}
	onDisconnect := pm.onDisconnect
	pm.mu.Unlock()

	if onDisconnect != nil {
		onDisconnect(peerID)
	}
}

