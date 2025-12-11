// Package protocol - Storage RPC types
package protocol

import (
	"context"
)

// StoreRequest is the request for storing content
type StoreRequest struct {
	Content     []byte            `json:"content"`
	ContentHash string            `json:"content_hash"`
	ContentType string            `json:"content_type"`
	Metadata    map[string]string `json:"metadata"`
	Replicas    int32             `json:"replicas"`
}

// StoreResponse is the response for storing content
type StoreResponse struct {
	Hash        string `json:"hash"`
	ContentHash string `json:"content_hash"`
	Size        int64  `json:"size"`
	Success     bool   `json:"success"`
}

// RetrieveRequest is the request for retrieving content
type RetrieveRequest struct {
	Hash        string `json:"hash"`
	ContentHash string `json:"content_hash"`
}

// RetrieveResponse is the response for retrieving content
type RetrieveResponse struct {
	Content  []byte             `json:"content"`
	Found    bool               `json:"found"`
	Metadata *ArtifactMetadata  `json:"metadata"`
}

// ExistsRequest is the request for checking if content exists
type ExistsRequest struct {
	ContentHash string `json:"content_hash"`
}

// ExistsResponse is the response for checking if content exists
type ExistsResponse struct {
	Exists    bool     `json:"exists"`
	Locations []string `json:"locations"`
	Replicas  []string `json:"replicas"`
}

// ListRequest is the request for listing stored content
type ListRequest struct {
	Prefix   string `json:"prefix"`
	Limit    int32  `json:"limit"`
	MaxItems int32  `json:"max_items"`
	Token    string `json:"token"`
}

// ListResponse is the response for listing stored content
type ListResponse struct {
	Items      []*ContentItem  `json:"items"`
	Artifacts  []*ArtifactInfo `json:"artifacts"`
	NextToken  string          `json:"next_token"`
	NextCursor string          `json:"next_cursor"`
}

// ContentItem represents a stored content item
type ContentItem struct {
	Hash        string            `json:"hash"`
	Size        int64             `json:"size"`
	ContentType string            `json:"content_type"`
	Metadata    map[string]string `json:"metadata"`
	CreatedAt   int64             `json:"created_at"`
}

// ArtifactInfo represents artifact information
type ArtifactInfo struct {
	ContentHash  string            `json:"content_hash"`
	Metadata     *ArtifactMetadata `json:"metadata"`
	ReplicaCount int32             `json:"replica_count"`
}

// ArtifactMetadata contains artifact metadata
type ArtifactMetadata struct {
	SizeBytes int64             `json:"size_bytes"`
	CreatedAt int64             `json:"created_at"`
	Labels    map[string]string `json:"labels"`
}

// ReplicateRequest is the request for replicating content
type ReplicateRequest struct {
	Hash        string   `json:"hash"`
	ContentHash string   `json:"content_hash"`
	TargetPeer  string   `json:"target_peer"`
	TargetPeers []string `json:"target_peers"`
}

// ReplicateResponse is the response for replicating content
type ReplicateResponse struct {
	Success         bool     `json:"success"`
	ReplicatedPeers []string `json:"replicated_peers"`
	FailedPeers     []string `json:"failed_peers"`
}

// StorageServer is the server interface for Storage service
type StorageServer interface {
	Store(ctx context.Context, req *StoreRequest) (*StoreResponse, error)
	Retrieve(ctx context.Context, req *RetrieveRequest) (*RetrieveResponse, error)
	Exists(ctx context.Context, req *ExistsRequest) (*ExistsResponse, error)
	List(ctx context.Context, req *ListRequest) (*ListResponse, error)
	Replicate(ctx context.Context, req *ReplicateRequest) (*ReplicateResponse, error)
}

// UnimplementedStorageServer can be embedded to have forward compatible implementations
type UnimplementedStorageServer struct{}

func (UnimplementedStorageServer) Store(ctx context.Context, req *StoreRequest) (*StoreResponse, error) {
	return nil, nil
}

func (UnimplementedStorageServer) Retrieve(ctx context.Context, req *RetrieveRequest) (*RetrieveResponse, error) {
	return nil, nil
}

func (UnimplementedStorageServer) Exists(ctx context.Context, req *ExistsRequest) (*ExistsResponse, error) {
	return nil, nil
}

func (UnimplementedStorageServer) List(ctx context.Context, req *ListRequest) (*ListResponse, error) {
	return nil, nil
}

func (UnimplementedStorageServer) Replicate(ctx context.Context, req *ReplicateRequest) (*ReplicateResponse, error) {
	return nil, nil
}
