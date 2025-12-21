// Package discovery provides peer discovery mechanisms
//
// T232: Implement DHT discovery
// US6: P2P Hive-Mind Device Federation
package discovery

import (
	"context"
	"fmt"

	dht "github.com/libp2p/go-libp2p-kad-dht"
	corediscovery "github.com/libp2p/go-libp2p/core/discovery"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/p2p/discovery/routing"
	"github.com/libp2p/go-libp2p/p2p/discovery/util"
)

// DHTDiscovery provides DHT-based peer discovery
type DHTDiscovery struct {
	dht     *dht.IpfsDHT
	host    host.Host
	routing *routing.RoutingDiscovery
	ctx     context.Context
}

// NewDHTDiscovery creates a new DHT discovery service
//
// Implements T232: Implement DHT discovery
func NewDHTDiscovery(ctx context.Context, h host.Host, bootstrapPeers []peer.AddrInfo) (*DHTDiscovery, error) {
	// Create DHT
	kademliaDHT, err := dht.New(ctx, h, dht.Mode(dht.ModeServer))
	if err != nil {
		return nil, fmt.Errorf("failed to create DHT: %w", err)
	}

	// Bootstrap the DHT
	if err := kademliaDHT.Bootstrap(ctx); err != nil {
		return nil, fmt.Errorf("failed to bootstrap DHT: %w", err)
	}

	// Connect to bootstrap peers
	for _, peerInfo := range bootstrapPeers {
		if err := h.Connect(ctx, peerInfo); err != nil {
			fmt.Printf("Warning: Failed to connect to bootstrap peer %s: %v\n", peerInfo.ID, err)
		}
	}

	// Create routing discovery
	routingDiscovery := routing.NewRoutingDiscovery(kademliaDHT)

	// Advertise ourselves
	util.Advertise(ctx, routingDiscovery, "noa-p2p")

	return &DHTDiscovery{
		dht:     kademliaDHT,
		host:    h,
		routing: routingDiscovery,
		ctx:     ctx,
	}, nil
}

// FindPeers searches for peers using DHT
func (d *DHTDiscovery) FindPeers(ctx context.Context, limit int) ([]peer.AddrInfo, error) {
	peerChan, err := d.routing.FindPeers(ctx, "noa-p2p", corediscovery.Limit(limit))
	if err != nil {
		return nil, fmt.Errorf("failed to find peers: %w", err)
	}

	var peers []peer.AddrInfo
	for p := range peerChan {
		if p.ID == d.host.ID() {
			continue // Skip ourselves
		}
		peers = append(peers, p)
		if len(peers) >= limit {
			break
		}
	}

	return peers, nil
}

// Stop stops the DHT discovery service
func (d *DHTDiscovery) Stop() error {
	return d.dht.Close()
}

// DHT returns the underlying DHT instance
func (d *DHTDiscovery) DHT() *dht.IpfsDHT {
	return d.dht
}

