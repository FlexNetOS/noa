// Package discovery provides peer discovery mechanisms
//
// T231: §3.8 Implement mDNS discovery
// US6: P2P Hive-Mind Device Federation
package discovery

import (
	"context"
	"fmt"
	"time"

	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/p2p/discovery/mdns"
)

const (
	// ServiceTag is the mDNS service tag for NOA P2P
	ServiceTag = "noa-p2p"
	// DiscoveryInterval is how often to announce presence
	DiscoveryInterval = time.Minute
)

// mDNSNotifee handles mDNS discovery events
type mDNSNotifee struct {
	host   host.Host
	onPeer func(peer.AddrInfo)
}

// HandlePeerFound is called when a peer is discovered
func (n *mDNSNotifee) HandlePeerFound(pi peer.AddrInfo) {
	fmt.Printf("mDNS: Discovered peer %s\n", pi.ID)
	if n.onPeer != nil {
		n.onPeer(pi)
	}
}

// MDNSDiscovery provides mDNS-based peer discovery
type MDNSDiscovery struct {
	service mdns.Service
	notifee *mDNSNotifee
}

// NewMDNSDiscovery creates a new mDNS discovery service
//
// Implements T231: §3.8 Implement mDNS discovery
func NewMDNSDiscovery(ctx context.Context, h host.Host, onPeer func(peer.AddrInfo)) (*MDNSDiscovery, error) {
	notifee := &mDNSNotifee{
		host:   h,
		onPeer: onPeer,
	}

	// In libp2p v0.37+, NewMdnsService takes the notifee directly
	service := mdns.NewMdnsService(h, ServiceTag, notifee)

	return &MDNSDiscovery{
		service: service,
		notifee: notifee,
	}, nil
}

// Start starts the mDNS discovery service
func (d *MDNSDiscovery) Start() error {
	// Service starts automatically when created
	return nil
}

// Stop stops the mDNS discovery service
func (d *MDNSDiscovery) Stop() error {
	return d.service.Close()
}

