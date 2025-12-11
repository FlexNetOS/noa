// Package storage provides distributed storage functionality
//
// T244: §3.8 Implement CAS (Content-Addressable Storage)
// US6: P2P Hive-Mind Device Federation
package storage

import (
	"crypto/sha256"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sync"
)

// CAS provides Content-Addressable Storage
type CAS struct {
	mu       sync.RWMutex
	basePath string
	index    map[string]string // content_hash -> file_path
}

// NewCAS creates a new CAS instance
//
// Implements T244: §3.8 Implement CAS (Content-Addressable Storage)
func NewCAS(basePath string) (*CAS, error) {
	// Ensure base path exists
	if err := os.MkdirAll(basePath, 0755); err != nil {
		return nil, fmt.Errorf("failed to create CAS directory: %w", err)
	}

	cas := &CAS{
		basePath: basePath,
		index:    make(map[string]string),
	}

	// Load existing index
	if err := cas.loadIndex(); err != nil {
		return nil, fmt.Errorf("failed to load CAS index: %w", err)
	}

	return cas, nil
}

// Store stores content and returns its hash
func (c *CAS) Store(content []byte) (string, error) {
	// Calculate content hash
	hash := c.calculateHash(content)

	c.mu.Lock()
	defer c.mu.Unlock()

	// Check if already stored
	if path, exists := c.index[hash]; exists {
		// Verify file still exists
		if _, err := os.Stat(path); err == nil {
			return hash, nil // Already stored
		}
		// File missing, remove from index
		delete(c.index, hash)
	}

	// Store content
	// Use hash-based directory structure: hash[0:2]/hash[2:4]/hash
	dir := filepath.Join(c.basePath, hash[0:2], hash[2:4])
	if err := os.MkdirAll(dir, 0755); err != nil {
		return "", fmt.Errorf("failed to create CAS directory: %w", err)
	}

	filePath := filepath.Join(dir, hash)
	if err := os.WriteFile(filePath, content, 0644); err != nil {
		return "", fmt.Errorf("failed to write CAS file: %w", err)
	}

	// Update index
	c.index[hash] = filePath

	return hash, nil
}

// Retrieve retrieves content by hash
func (c *CAS) Retrieve(hash string) ([]byte, error) {
	c.mu.RLock()
	path, exists := c.index[hash]
	c.mu.RUnlock()

	if !exists {
		// Try to find in expected location
		path = filepath.Join(c.basePath, hash[0:2], hash[2:4], hash)
	}

	content, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read CAS file: %w", err)
	}

	// Verify hash
	calculatedHash := c.calculateHash(content)
	if calculatedHash != hash {
		return nil, fmt.Errorf("hash mismatch: expected %s, got %s", hash, calculatedHash)
	}

	return content, nil
}

// Exists checks if content exists
func (c *CAS) Exists(hash string) bool {
	c.mu.RLock()
	defer c.mu.RUnlock()

	path, exists := c.index[hash]
	if !exists {
		return false
	}

	// Verify file exists
	_, err := os.Stat(path)
	return err == nil
}

// Delete removes content from CAS
func (c *CAS) Delete(hash string) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	path, exists := c.index[hash]
	if !exists {
		return fmt.Errorf("content not found: %s", hash)
	}

	if err := os.Remove(path); err != nil {
		return fmt.Errorf("failed to delete CAS file: %w", err)
	}

	delete(c.index, hash)
	return nil
}

// calculateHash calculates SHA-256 hash of content
func (c *CAS) calculateHash(content []byte) string {
	h := sha256.New()
	h.Write(content)
	return fmt.Sprintf("%x", h.Sum(nil))
}

// loadIndex loads the CAS index from disk
func (c *CAS) loadIndex() error {
	// Walk the CAS directory and build index
	return filepath.Walk(c.basePath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		// Read file and calculate hash
		content, err := os.ReadFile(path)
		if err != nil {
			return err
		}

		hash := c.calculateHash(content)
		c.index[hash] = path

		return nil
	})
}

// StoreFromReader stores content from an io.Reader
func (c *CAS) StoreFromReader(reader io.Reader) (string, int64, error) {
	// Read all content
	content, err := io.ReadAll(reader)
	if err != nil {
		return "", 0, fmt.Errorf("failed to read content: %w", err)
	}

	hash, err := c.Store(content)
	if err != nil {
		return "", 0, err
	}

	return hash, int64(len(content)), nil
}

