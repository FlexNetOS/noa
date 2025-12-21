// Package discovery provides Discovery service RPC implementations
//
// T502: Implement Discovery.FindPeers RPC
// US6: P2P Hive-Mind Device Federation
package discovery

import (
	"context"

	"github.com/FlexNetOS/noa/p2p/pkg/protocol"
)

// FindPeers handles the FindPeers RPC
//
// Implements T502: Implement Discovery.FindPeers RPC
func (s *DiscoveryService) FindPeers(ctx context.Context, req *protocol.FindPeersRequest) (*protocol.FindPeersResponse, error) {
	maxResults := int(req.MaxResults)
	if maxResults <= 0 {
		maxResults = 10 // Default limit
	}

	// Query peerstore for peers matching capabilities
	ps := s.host.Peerstore()
	peers := make([]*protocol.PeerInfo, 0)

	// Get all known peers
	allPeers := ps.Peers()
	count := 0

	for _, peerID := range allPeers {
		if peerID == s.host.ID() {
			continue // Skip ourselves
		}

		if count >= maxResults {
			break
		}

		// Get peer addresses
		addrs := ps.Addrs(peerID)
		addrStrings := make([]string, len(addrs))
		for i, addr := range addrs {
			addrStrings[i] = addr.String()
		}

		// Build peer info (simplified - would need to store full peer info)
		peerInfo := &protocol.PeerInfo{
			PeerId:    peerID.String(),
			Addresses: addrStrings,
			// Capabilities and resources would be stored in peer metadata
		}

		peers = append(peers, peerInfo)
		count++
	}

	return &protocol.FindPeersResponse{
		Peers: peers,
	}, nil
}

