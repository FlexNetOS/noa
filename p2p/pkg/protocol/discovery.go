// Package protocol - Discovery RPC types
package protocol

import (
	"context"
)

// DeviceInfo represents device information
type DeviceInfo struct {
	DeviceId   string `json:"device_id"`
	DeviceName string `json:"device_name"`
	DeviceType string `json:"device_type"`
}

// ResourceInfo represents resource information
type ResourceInfo struct {
	CpuCores   int32  `json:"cpu_cores"`
	MemoryMb   int64  `json:"memory_mb"`
	StorageGb  int64  `json:"storage_gb"`
	GpuAvailable bool `json:"gpu_available"`
}

// FindPeersRequest is the request for finding peers
type FindPeersRequest struct {
	Capabilities []string `json:"capabilities"`
	MaxResults   int32    `json:"max_results"`
}

// FindPeersResponse is the response for finding peers
type FindPeersResponse struct {
	Peers []*PeerInfo `json:"peers"`
}

// PeerInfo represents information about a peer
type PeerInfo struct {
	PeerId       string            `json:"peer_id"`
	Addresses    []string          `json:"addresses"`
	Capabilities []string          `json:"capabilities"`
	Metadata     map[string]string `json:"metadata"`
	Status       string            `json:"status"`
	Device       *DeviceInfo       `json:"device"`
	Resources    *ResourceInfo     `json:"resources"`
}

// AnnounceRequest is the request for announcing presence
type AnnounceRequest struct {
	Capabilities []string          `json:"capabilities"`
	Metadata     map[string]string `json:"metadata"`
	Ttl          int64             `json:"ttl"`
	Device       *DeviceInfo       `json:"device"`
	Resources    *ResourceInfo     `json:"resources"`
}

// AnnounceResponse is the response for announcing presence
type AnnounceResponse struct {
	Success    bool        `json:"success"`
	Message    string      `json:"message"`
	Accepted   bool        `json:"accepted"`
	PeerId     string      `json:"peer_id"`
	KnownPeers []*PeerInfo `json:"known_peers"`
}

// PingRequest is the request for pinging a peer
type PingRequest struct {
	PeerId    string `json:"peer_id"`
	Payload   []byte `json:"payload"`
	Timestamp int64  `json:"timestamp"`
}

// PingResponse is the response for ping
type PingResponse struct {
	Success   bool   `json:"success"`
	LatencyMs int64  `json:"latency_ms"`
	Payload   []byte `json:"payload"`
	Timestamp int64  `json:"timestamp"`
}

// DiscoveryServer is the server interface for Discovery service
type DiscoveryServer interface {
	FindPeers(ctx context.Context, req *FindPeersRequest) (*FindPeersResponse, error)
	Announce(ctx context.Context, req *AnnounceRequest) (*AnnounceResponse, error)
	Ping(ctx context.Context, req *PingRequest) (*PingResponse, error)
}

// UnimplementedDiscoveryServer can be embedded to have forward compatible implementations
type UnimplementedDiscoveryServer struct{}

func (UnimplementedDiscoveryServer) FindPeers(ctx context.Context, req *FindPeersRequest) (*FindPeersResponse, error) {
	return nil, nil
}

func (UnimplementedDiscoveryServer) Announce(ctx context.Context, req *AnnounceRequest) (*AnnounceResponse, error) {
	return nil, nil
}

func (UnimplementedDiscoveryServer) Ping(ctx context.Context, req *PingRequest) (*PingResponse, error) {
	return nil, nil
}
