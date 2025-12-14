module github.com/FlexNetOS/noa/p2p

go 1.23

require (
	github.com/libp2p/go-libp2p v0.37.0
	github.com/libp2p/go-libp2p-kad-dht v0.28.0
	github.com/libp2p/go-libp2p/p2p/discovery/mdns v0.13.0
	github.com/libp2p/go-libp2p/p2p/discovery/routing v0.2.0
	github.com/libp2p/go-libp2p/p2p/discovery/util v0.1.0
	github.com/libp2p/go-libp2p/p2p/security/noise v0.9.0
	libp2p.org/go-libp2p/p2p/security/tls v0.0.0-20241012235623-2c5e3c0e3c0e
	github.com/multiformats/go-multiaddr v0.14.0
	github.com/ipfs/go-datastore v0.6.0
	github.com/shirou/gopsutil/v3 v3.24.1
	google.golang.org/grpc v1.68.0
	google.golang.org/protobuf v1.35.2
)

