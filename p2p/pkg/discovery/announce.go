// Package discovery provides Discovery service RPC implementations
//
// T501: §3.8 Implement Discovery.Announce RPC
// US6: P2P Hive-Mind Device Federation
package discovery

import (
	"context"
	"fmt"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
)

// DiscoveryService implements the Discovery gRPC service
type DiscoveryService struct {
	protocol.UnimplementedDiscoveryServer
	host host.Host
	// Add any additional state needed for discovery
}

// NewDiscoveryService creates a new discovery service
func NewDiscoveryService(h host.Host) *DiscoveryService {
	return &DiscoveryService{
		host: h,
	}
}

// Announce handles the Announce RPC
//
// Implements T501: §3.8 Implement Discovery.Announce RPC
func (s *DiscoveryService) Announce(ctx context.Context, req *protocol.AnnounceRequest) (*protocol.AnnounceResponse, error) {
	// Extract device info from request
	deviceInfo := req.Device
	if deviceInfo == nil {
		return nil, fmt.Errorf("device info is required")
	}

	// Get our peer ID
	peerID := s.host.ID()

	// Get our addresses
	addrs := s.host.Addrs()
	addrStrings := make([]string, len(addrs))
	for i, addr := range addrs {
		addrStrings[i] = addr.String()
	}

	// Build known peers list (simplified - would query peerstore)
	knownPeers := []*protocol.PeerInfo{
		{
			PeerId:    peerID.String(),
			Device:    deviceInfo,
			Addresses: addrStrings,
			Capabilities: req.Capabilities,
			Resources: req.Resources,
		},
	}

	return &protocol.AnnounceResponse{
		Accepted:  true,
		PeerId:    peerID.String(),
		KnownPeers: knownPeers,
	}, nil
}

