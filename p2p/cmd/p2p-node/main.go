// NOA P2P Node
//
// Entry point for the NOA P2P networking service.
package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/FlexNetOS/noa/p2p/internal/node"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Create and start the P2P node
	n, err := node.New(ctx)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to create node: %v\n", err)
		os.Exit(1)
	}

	// Start the node
	if err := n.Start(ctx); err != nil {
		fmt.Fprintf(os.Stderr, "Failed to start node: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("NOA P2P node started. Peer ID: %s\n", n.ID())

	// Wait for shutdown signal
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	<-sigCh

	fmt.Println("\nShutting down...")
	if err := n.Stop(); err != nil {
		fmt.Fprintf(os.Stderr, "Error during shutdown: %v\n", err)
	}
}

