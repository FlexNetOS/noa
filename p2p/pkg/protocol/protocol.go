// Package protocol contains the Go types and gRPC service definitions for NOA's
// P2P protocol.
//
// NOTE: This package is intentionally checked into the repository (rather than
// generated during builds) so `go test`/`go mod tidy` work out of the box.
//
// The authoritative schema lives in:
//   specs/001-noa-seed-foundation/contracts/p2p-protocol.proto
package protocol

import (
	context "context"

	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// ====================
// Common Types
// ====================

type DeviceInfo struct {
	DeviceId   string
	Name       string
	Type       string
	Platform   string
	NoaVersion string
}

type PeerInfo struct {
	PeerId       string
	Device       *DeviceInfo
	Addresses    []string
	Capabilities []Capability
	Resources    *ResourceInfo
	LastSeen     int64
}

type Capability int32

const (
	Capability_CAPABILITY_UNSPECIFIED Capability = 0
	Capability_CAPABILITY_INFERENCE   Capability = 1
	Capability_CAPABILITY_DIGEST      Capability = 2
	Capability_CAPABILITY_STORAGE     Capability = 3
	Capability_CAPABILITY_COMPUTE     Capability = 4
	Capability_CAPABILITY_GPU         Capability = 5
)

type ResourceInfo struct {
	CpuCores         int64
	MemoryMb         int64
	StorageMb        int64
	GpuMemoryMb      int64
	CpuAvailable     float32
	MemoryAvailable  float32
	StorageAvailable float32
}

type ResourceRequirements struct {
	MinMemoryMb         int64
	MinCpuCores         int64
	RequiresGpu         bool
	EstimatedDurationMs int64
}

type Change struct {
	Id           string
	EntityId     string
	EntityType   string
	Type         ChangeType
	Data         []byte
	Timestamp    int64
	SourceDevice string
	Version      int64
}

type ChangeType int32

const (
	ChangeType_CHANGE_TYPE_UNSPECIFIED ChangeType = 0
	ChangeType_CHANGE_TYPE_CREATE      ChangeType = 1
	ChangeType_CHANGE_TYPE_UPDATE      ChangeType = 2
	ChangeType_CHANGE_TYPE_DELETE      ChangeType = 3
)

type Conflict struct {
	Id           string
	EntityId     string
	LocalChange  *Change
	RemoteChange *Change
}

type Resolution int32

const (
	Resolution_RESOLUTION_UNSPECIFIED  Resolution = 0
	Resolution_RESOLUTION_LOCAL_WINS   Resolution = 1
	Resolution_RESOLUTION_REMOTE_WINS  Resolution = 2
	Resolution_RESOLUTION_MERGE        Resolution = 3
)

type TaskType int32

const (
	TaskType_TASK_TYPE_UNSPECIFIED TaskType = 0
	TaskType_TASK_TYPE_INFERENCE   TaskType = 1
	TaskType_TASK_TYPE_EMBEDDING   TaskType = 2
	TaskType_TASK_TYPE_PARSE       TaskType = 3
	TaskType_TASK_TYPE_ANALYZE     TaskType = 4
	TaskType_TASK_TYPE_CUSTOM      TaskType = 5
)

type TaskStatus int32

const (
	TaskStatus_TASK_STATUS_UNSPECIFIED TaskStatus = 0
	TaskStatus_TASK_STATUS_QUEUED      TaskStatus = 1
	TaskStatus_TASK_STATUS_RUNNING     TaskStatus = 2
	TaskStatus_TASK_STATUS_COMPLETED   TaskStatus = 3
	TaskStatus_TASK_STATUS_FAILED      TaskStatus = 4
	TaskStatus_TASK_STATUS_CANCELLED   TaskStatus = 5
)

type OutputType int32

const (
	OutputType_OUTPUT_TYPE_UNSPECIFIED OutputType = 0
	OutputType_OUTPUT_TYPE_STDOUT      OutputType = 1
	OutputType_OUTPUT_TYPE_STDERR      OutputType = 2
	OutputType_OUTPUT_TYPE_PROGRESS    OutputType = 3
	OutputType_OUTPUT_TYPE_RESULT      OutputType = 4
)

type ArtifactMetadata struct {
	ContentType string
	SizeBytes   int64
	CreatedAt   int64
	Labels      map[string]string
}

type ArtifactInfo struct {
	ContentHash   string
	Metadata      *ArtifactMetadata
	ReplicaCount  int32
}

// ====================
// Discovery Service
// ====================

type AnnounceRequest struct {
	Device       *DeviceInfo
	Capabilities []Capability
	Resources    *ResourceInfo
}

type AnnounceResponse struct {
	Accepted   bool
	PeerId     string
	KnownPeers []*PeerInfo
}

type FindPeersRequest struct {
	RequiredCapabilities []Capability
	MaxResults           int32
}

type FindPeersResponse struct {
	Peers []*PeerInfo
}

type PingRequest struct {
	Timestamp int64
}

type PingResponse struct {
	Timestamp int64
	LatencyMs int64
}

// DiscoveryServer is the server API for Discovery service.
//
// Implementations should embed UnimplementedDiscoveryServer for forward
// compatibility.
type DiscoveryServer interface {
	Announce(context.Context, *AnnounceRequest) (*AnnounceResponse, error)
	FindPeers(context.Context, *FindPeersRequest) (*FindPeersResponse, error)
	Ping(context.Context, *PingRequest) (*PingResponse, error)
	mustEmbedUnimplementedDiscoveryServer()
}

type UnimplementedDiscoveryServer struct{}

func (UnimplementedDiscoveryServer) Announce(context.Context, *AnnounceRequest) (*AnnounceResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Announce not implemented")
}
func (UnimplementedDiscoveryServer) FindPeers(context.Context, *FindPeersRequest) (*FindPeersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method FindPeers not implemented")
}
func (UnimplementedDiscoveryServer) Ping(context.Context, *PingRequest) (*PingResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Ping not implemented")
}
func (UnimplementedDiscoveryServer) mustEmbedUnimplementedDiscoveryServer() {}

// UnsafeDiscoveryServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended.
type UnsafeDiscoveryServer interface {
	mustEmbedUnimplementedDiscoveryServer()
}

func RegisterDiscoveryServer(s grpc.ServiceRegistrar, srv DiscoveryServer) {
	s.RegisterService(&Discovery_ServiceDesc, srv)
}

func _Discovery_Announce_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(AnnounceRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(DiscoveryServer).Announce(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Discovery/Announce"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(DiscoveryServer).Announce(ctx, req.(*AnnounceRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Discovery_FindPeers_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(FindPeersRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(DiscoveryServer).FindPeers(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Discovery/FindPeers"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(DiscoveryServer).FindPeers(ctx, req.(*FindPeersRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Discovery_Ping_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(PingRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(DiscoveryServer).Ping(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Discovery/Ping"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(DiscoveryServer).Ping(ctx, req.(*PingRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var Discovery_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "noa.p2p.Discovery",
	HandlerType: (*DiscoveryServer)(nil),
	Methods: []grpc.MethodDesc{
		{MethodName: "Announce", Handler: _Discovery_Announce_Handler},
		{MethodName: "FindPeers", Handler: _Discovery_FindPeers_Handler},
		{MethodName: "Ping", Handler: _Discovery_Ping_Handler},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "specs/001-noa-seed-foundation/contracts/p2p-protocol.proto",
}

// ====================
// Sync Service
// ====================

type GetStateRequest struct {
	EntityType   string
	SinceVersion int64
}

type GetStateResponse struct {
	CurrentVersion int64
	StateHash      []byte
	PendingChanges int32
}

type PushChangesRequest struct {
	EntityType  string
	Changes     []*Change
	BaseVersion int64
}

type PushChangesResponse struct {
	Accepted   bool
	NewVersion int64
	Conflicts  []*Conflict
}

type PullChangesRequest struct {
	EntityType    string
	SinceVersion  int64
	MaxChanges    int32
}

type PullChangesResponse struct {
	Changes       []*Change
	LatestVersion int64
	HasMore       bool
}

type ResolveConflictRequest struct {
	ConflictId string
	Resolution Resolution
}

type ResolveConflictResponse struct {
	Success    bool
	NewVersion int64
}

// SyncServer is the server API for Sync service.
//
// Implementations should embed UnimplementedSyncServer for forward
// compatibility.
type SyncServer interface {
	GetState(context.Context, *GetStateRequest) (*GetStateResponse, error)
	PushChanges(context.Context, *PushChangesRequest) (*PushChangesResponse, error)
	PullChanges(context.Context, *PullChangesRequest) (*PullChangesResponse, error)
	ResolveConflict(context.Context, *ResolveConflictRequest) (*ResolveConflictResponse, error)
	mustEmbedUnimplementedSyncServer()
}

type UnimplementedSyncServer struct{}

func (UnimplementedSyncServer) GetState(context.Context, *GetStateRequest) (*GetStateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetState not implemented")
}
func (UnimplementedSyncServer) PushChanges(context.Context, *PushChangesRequest) (*PushChangesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PushChanges not implemented")
}
func (UnimplementedSyncServer) PullChanges(context.Context, *PullChangesRequest) (*PullChangesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PullChanges not implemented")
}
func (UnimplementedSyncServer) ResolveConflict(context.Context, *ResolveConflictRequest) (*ResolveConflictResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ResolveConflict not implemented")
}
func (UnimplementedSyncServer) mustEmbedUnimplementedSyncServer() {}

type UnsafeSyncServer interface {
	mustEmbedUnimplementedSyncServer()
}

func RegisterSyncServer(s grpc.ServiceRegistrar, srv SyncServer) {
	s.RegisterService(&Sync_ServiceDesc, srv)
}

func _Sync_GetState_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(GetStateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SyncServer).GetState(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Sync/GetState"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(SyncServer).GetState(ctx, req.(*GetStateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Sync_PushChanges_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(PushChangesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SyncServer).PushChanges(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Sync/PushChanges"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(SyncServer).PushChanges(ctx, req.(*PushChangesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Sync_PullChanges_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(PullChangesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SyncServer).PullChanges(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Sync/PullChanges"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(SyncServer).PullChanges(ctx, req.(*PullChangesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Sync_ResolveConflict_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(ResolveConflictRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SyncServer).ResolveConflict(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Sync/ResolveConflict"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(SyncServer).ResolveConflict(ctx, req.(*ResolveConflictRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var Sync_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "noa.p2p.Sync",
	HandlerType: (*SyncServer)(nil),
	Methods: []grpc.MethodDesc{
		{MethodName: "GetState", Handler: _Sync_GetState_Handler},
		{MethodName: "PushChanges", Handler: _Sync_PushChanges_Handler},
		{MethodName: "PullChanges", Handler: _Sync_PullChanges_Handler},
		{MethodName: "ResolveConflict", Handler: _Sync_ResolveConflict_Handler},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "specs/001-noa-seed-foundation/contracts/p2p-protocol.proto",
}

// ====================
// Compute Service
// ====================

type SubmitTaskRequest struct {
	TaskId       string
	TaskType     TaskType
	Payload      []byte
	Requirements *ResourceRequirements
	TimeoutMs    int64
}

type SubmitTaskResponse struct {
	Accepted    bool
	ExecutionId string
	EstimatedMs int64
}

type GetTaskStatusRequest struct {
	ExecutionId string
}

type GetTaskStatusResponse struct {
	Status   TaskStatus
	Progress float32
	Result   []byte
	Error    string
}

type CancelTaskRequest struct {
	ExecutionId string
}

type CancelTaskResponse struct {
	Cancelled bool
}

type StreamOutputRequest struct {
	ExecutionId string
}

type TaskOutput struct {
	Type      OutputType
	Data      []byte
	Timestamp int64
}

type ComputeServer interface {
	SubmitTask(context.Context, *SubmitTaskRequest) (*SubmitTaskResponse, error)
	GetTaskStatus(context.Context, *GetTaskStatusRequest) (*GetTaskStatusResponse, error)
	CancelTask(context.Context, *CancelTaskRequest) (*CancelTaskResponse, error)
	StreamOutput(*StreamOutputRequest, Compute_StreamOutputServer) error
	mustEmbedUnimplementedComputeServer()
}

type UnimplementedComputeServer struct{}

func (UnimplementedComputeServer) SubmitTask(context.Context, *SubmitTaskRequest) (*SubmitTaskResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SubmitTask not implemented")
}
func (UnimplementedComputeServer) GetTaskStatus(context.Context, *GetTaskStatusRequest) (*GetTaskStatusResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetTaskStatus not implemented")
}
func (UnimplementedComputeServer) CancelTask(context.Context, *CancelTaskRequest) (*CancelTaskResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CancelTask not implemented")
}
func (UnimplementedComputeServer) StreamOutput(*StreamOutputRequest, Compute_StreamOutputServer) error {
	return status.Errorf(codes.Unimplemented, "method StreamOutput not implemented")
}
func (UnimplementedComputeServer) mustEmbedUnimplementedComputeServer() {}

type UnsafeComputeServer interface {
	mustEmbedUnimplementedComputeServer()
}

type Compute_StreamOutputServer interface {
	Send(*TaskOutput) error
	grpc.ServerStream
}

func RegisterComputeServer(s grpc.ServiceRegistrar, srv ComputeServer) {
	s.RegisterService(&Compute_ServiceDesc, srv)
}

func _Compute_SubmitTask_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(SubmitTaskRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ComputeServer).SubmitTask(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Compute/SubmitTask"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(ComputeServer).SubmitTask(ctx, req.(*SubmitTaskRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Compute_GetTaskStatus_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(GetTaskStatusRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ComputeServer).GetTaskStatus(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Compute/GetTaskStatus"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(ComputeServer).GetTaskStatus(ctx, req.(*GetTaskStatusRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Compute_CancelTask_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(CancelTaskRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ComputeServer).CancelTask(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Compute/CancelTask"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(ComputeServer).CancelTask(ctx, req.(*CancelTaskRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Compute_StreamOutput_Handler(srv any, stream grpc.ServerStream) error {
	m := new(StreamOutputRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(ComputeServer).StreamOutput(m, &computeStreamOutputServer{ServerStream: stream})
}

type computeStreamOutputServer struct{ grpc.ServerStream }

func (x *computeStreamOutputServer) Send(m *TaskOutput) error { return x.ServerStream.SendMsg(m) }

var Compute_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "noa.p2p.Compute",
	HandlerType: (*ComputeServer)(nil),
	Methods: []grpc.MethodDesc{
		{MethodName: "SubmitTask", Handler: _Compute_SubmitTask_Handler},
		{MethodName: "GetTaskStatus", Handler: _Compute_GetTaskStatus_Handler},
		{MethodName: "CancelTask", Handler: _Compute_CancelTask_Handler},
	},
	Streams: []grpc.StreamDesc{
		{StreamName: "StreamOutput", Handler: _Compute_StreamOutput_Handler, ServerStreams: true},
	},
	Metadata: "specs/001-noa-seed-foundation/contracts/p2p-protocol.proto",
}

// ====================
// Storage Service
// ====================

type StoreRequest struct {
	ContentHash string
	Content     []byte
	Metadata    *ArtifactMetadata
}

type StoreResponse struct {
	Success     bool
	ContentHash string
}

type RetrieveRequest struct {
	ContentHash string
}

type RetrieveResponse struct {
	Content   []byte
	Metadata  *ArtifactMetadata
}

type ExistsRequest struct {
	ContentHash string
}

type ExistsResponse struct {
	Exists    bool
	Locations []string
}

type ListRequest struct {
	Prefix string
	Limit  int32
	Cursor string
}

type ListResponse struct {
	Artifacts   []*ArtifactInfo
	NextCursor  string
}

type ReplicateRequest struct {
	ContentHash string
	TargetPeer  string
}

type ReplicateResponse struct {
	Success bool
}

type StorageServer interface {
	Store(context.Context, *StoreRequest) (*StoreResponse, error)
	Retrieve(context.Context, *RetrieveRequest) (*RetrieveResponse, error)
	Exists(context.Context, *ExistsRequest) (*ExistsResponse, error)
	List(context.Context, *ListRequest) (*ListResponse, error)
	Replicate(context.Context, *ReplicateRequest) (*ReplicateResponse, error)
	mustEmbedUnimplementedStorageServer()
}

type UnimplementedStorageServer struct{}

func (UnimplementedStorageServer) Store(context.Context, *StoreRequest) (*StoreResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Store not implemented")
}
func (UnimplementedStorageServer) Retrieve(context.Context, *RetrieveRequest) (*RetrieveResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Retrieve not implemented")
}
func (UnimplementedStorageServer) Exists(context.Context, *ExistsRequest) (*ExistsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Exists not implemented")
}
func (UnimplementedStorageServer) List(context.Context, *ListRequest) (*ListResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method List not implemented")
}
func (UnimplementedStorageServer) Replicate(context.Context, *ReplicateRequest) (*ReplicateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Replicate not implemented")
}
func (UnimplementedStorageServer) mustEmbedUnimplementedStorageServer() {}

type UnsafeStorageServer interface {
	mustEmbedUnimplementedStorageServer()
}

func RegisterStorageServer(s grpc.ServiceRegistrar, srv StorageServer) {
	s.RegisterService(&Storage_ServiceDesc, srv)
}

func _Storage_Store_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(StoreRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StorageServer).Store(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Storage/Store"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(StorageServer).Store(ctx, req.(*StoreRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Storage_Retrieve_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(RetrieveRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StorageServer).Retrieve(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Storage/Retrieve"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(StorageServer).Retrieve(ctx, req.(*RetrieveRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Storage_Exists_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(ExistsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StorageServer).Exists(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Storage/Exists"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(StorageServer).Exists(ctx, req.(*ExistsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Storage_List_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(ListRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StorageServer).List(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Storage/List"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(StorageServer).List(ctx, req.(*ListRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Storage_Replicate_Handler(srv any, ctx context.Context, dec func(any) error, interceptor grpc.UnaryServerInterceptor) (any, error) {
	in := new(ReplicateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StorageServer).Replicate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{Server: srv, FullMethod: "/noa.p2p.Storage/Replicate"}
	handler := func(ctx context.Context, req any) (any, error) {
		return srv.(StorageServer).Replicate(ctx, req.(*ReplicateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var Storage_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "noa.p2p.Storage",
	HandlerType: (*StorageServer)(nil),
	Methods: []grpc.MethodDesc{
		{MethodName: "Store", Handler: _Storage_Store_Handler},
		{MethodName: "Retrieve", Handler: _Storage_Retrieve_Handler},
		{MethodName: "Exists", Handler: _Storage_Exists_Handler},
		{MethodName: "List", Handler: _Storage_List_Handler},
		{MethodName: "Replicate", Handler: _Storage_Replicate_Handler},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "specs/001-noa-seed-foundation/contracts/p2p-protocol.proto",
}
