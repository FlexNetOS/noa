package security

import (
	"crypto/tls"
	"crypto/x509"
	"fmt"
)

// NewMTLSConfig builds a TLS configuration that requires mutual authentication using device keys.
// certPEM/keyPEM should contain the device's certificate and private key in PEM format.
// peerPool should contain trusted peer certificates or CAs.
func NewMTLSConfig(certPEM, keyPEM []byte, peerPool *x509.CertPool) (*tls.Config, error) {
	deviceCert, err := tls.X509KeyPair(certPEM, keyPEM)
	if err != nil {
		return nil, fmt.Errorf("mtls: failed to load device certificate: %w", err)
	}

	cfg := &tls.Config{
		MinVersion:   tls.VersionTLS13,
		ClientCAs:    peerPool,
		ClientAuth:   tls.RequireAndVerifyClientCert,
		Certificates: []tls.Certificate{deviceCert},
	}

	return cfg, nil
}

// AppendPeerCert adds a peer certificate to a cert pool so it can be trusted for mTLS handshakes.
func AppendPeerCert(pool *x509.CertPool, peer tls.Certificate) error {
	for _, raw := range peer.Certificate {
		cert, err := x509.ParseCertificate(raw)
		if err != nil {
			return fmt.Errorf("mtls: failed to parse peer certificate: %w", err)
		}
		pool.AddCert(cert)
	}
	return nil
}

// NewPeerCertPool constructs a pool from a slice of PEM-encoded certificates.
func NewPeerCertPool(pems [][]byte) (*x509.CertPool, error) {
	pool := x509.NewCertPool()
	for _, pemBytes := range pems {
		if ok := pool.AppendCertsFromPEM(pemBytes); !ok {
			return nil, fmt.Errorf("mtls: failed to append peer certificate")
		}
	}
	return pool, nil
}
