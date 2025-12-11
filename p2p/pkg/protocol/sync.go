// Package protocol - Sync RPC types
package protocol

import (
	"context"
)

// PushChangesRequest is the request for pushing changes
type PushChangesRequest struct {
	Changes []*Change `json:"changes"`
	SourceDevice string `json:"source_device"`
}

// PushChangesResponse is the response for pushing changes
type PushChangesResponse struct {
	Applied    int32       `json:"applied"`
	Accepted   bool        `json:"accepted"`
	NewVersion int64       `json:"new_version"`
	Conflicts  []*Conflict `json:"conflicts"`
	Success    bool        `json:"success"`
}

// PullChangesRequest is the request for pulling changes
type PullChangesRequest struct {
	SinceVersion int64    `json:"since_version"`
	EntityTypes  []string `json:"entity_types"`
	MaxChanges   int32    `json:"max_changes"`
}

// PullChangesResponse is the response for pulling changes
type PullChangesResponse struct {
	Changes        []*Change `json:"changes"`
	CurrentVersion int64     `json:"current_version"`
	LatestVersion  int64     `json:"latest_version"`
	HasMore        bool      `json:"has_more"`
}

// GetStateRequest is the request for getting state
type GetStateRequest struct {
	EntityId     string `json:"entity_id"`
	EntityType   string `json:"entity_type"`
	SinceVersion int64  `json:"since_version"`
}

// GetStateResponse is the response for getting state
type GetStateResponse struct {
	Data           []byte `json:"data"`
	Version        int64  `json:"version"`
	CurrentVersion int64  `json:"current_version"`
	StateHash      []byte `json:"state_hash"`
	PendingChanges int32  `json:"pending_changes"`
	Found          bool   `json:"found"`
}

// ResolveConflictRequest is the request for resolving conflicts
type ResolveConflictRequest struct {
	ConflictId string `json:"conflict_id"`
	Resolution string `json:"resolution"`
	Winner     string `json:"winner"`
}

// ResolveConflictResponse is the response for resolving conflicts
type ResolveConflictResponse struct {
	Success    bool   `json:"success"`
	NewVersion int64  `json:"new_version"`
	Message    string `json:"message"`
}

// Change represents a change/delta
type Change struct {
	Id           string `json:"id"`
	EntityId     string `json:"entity_id"`
	EntityType   string `json:"entity_type"`
	Type         string `json:"type"`
	Data         []byte `json:"data"`
	Timestamp    int64  `json:"timestamp"`
	SourceDevice string `json:"source_device"`
	Version      int64  `json:"version"`
}

// Conflict represents a sync conflict
type Conflict struct {
	Id            string  `json:"id"`
	EntityId      string  `json:"entity_id"`
	EntityType    string  `json:"entity_type"`
	LocalVersion  *Change `json:"local_version"`
	RemoteVersion *Change `json:"remote_version"`
	LocalChange   *Change `json:"local_change"`
	RemoteChange  *Change `json:"remote_change"`
	CreatedAt     int64   `json:"created_at"`
}

// ChangeType is a string alias for change types
type ChangeType = string

const (
	ChangeTypeCreate ChangeType = "create"
	ChangeTypeUpdate ChangeType = "update"
	ChangeTypeDelete ChangeType = "delete"
)

// Resolution constants for conflict resolution
const (
	Resolution_RESOLUTION_LOCAL_WINS  = "local_wins"
	Resolution_RESOLUTION_REMOTE_WINS = "remote_wins"
	Resolution_RESOLUTION_MERGE       = "merge"
)

// SyncServer is the server interface for Sync service
type SyncServer interface {
	PushChanges(ctx context.Context, req *PushChangesRequest) (*PushChangesResponse, error)
	PullChanges(ctx context.Context, req *PullChangesRequest) (*PullChangesResponse, error)
	GetState(ctx context.Context, req *GetStateRequest) (*GetStateResponse, error)
	ResolveConflict(ctx context.Context, req *ResolveConflictRequest) (*ResolveConflictResponse, error)
}

// UnimplementedSyncServer can be embedded to have forward compatible implementations
type UnimplementedSyncServer struct{}

func (UnimplementedSyncServer) PushChanges(ctx context.Context, req *PushChangesRequest) (*PushChangesResponse, error) {
	return nil, nil
}

func (UnimplementedSyncServer) PullChanges(ctx context.Context, req *PullChangesRequest) (*PullChangesResponse, error) {
	return nil, nil
}

func (UnimplementedSyncServer) GetState(ctx context.Context, req *GetStateRequest) (*GetStateResponse, error) {
	return nil, nil
}

func (UnimplementedSyncServer) ResolveConflict(ctx context.Context, req *ResolveConflictRequest) (*ResolveConflictResponse, error) {
	return nil, nil
}
