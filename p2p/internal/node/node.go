// Package node provides the core P2P node implementation
package node

import (
	"context"
	"fmt"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
)

// Node represents a NOA P2P node
type Node struct {
	host host.Host
}

// New creates a new P2P node
func New(ctx context.Context) (*Node, error) {
	h, err := libp2p.New(
		libp2p.ListenAddrStrings(
			"/ip4/0.0.0.0/tcp/0",
			"/ip6/::/tcp/0",
		),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create host: %w", err)
	}

	return &Node{host: h}, nil
}

// Start starts the P2P node
func (n *Node) Start(ctx context.Context) error {
	// Log listening addresses
	for _, addr := range n.host.Addrs() {
		fmt.Printf("Listening on: %s/p2p/%s\n", addr, n.host.ID())
	}
	return nil
}

// Stop gracefully stops the node
func (n *Node) Stop() error {
	return n.host.Close()
}

// ID returns the peer ID of this node
func (n *Node) ID() peer.ID {
	return n.host.ID()
}

// Host returns the underlying libp2p host
func (n *Node) Host() host.Host {
	return n.host
}

