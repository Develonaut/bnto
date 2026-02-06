// Package hangiri provides persistent storage for bento data.
//
// "Hangiri" (半切り - wooden tub for sushi rice) stores bento-related data
// on disk as JSON files, allowing bentos, secrets, and other data to be saved,
// loaded, and reused.
//
// Storage structure:
//
//	~/.bento/
//	  bentos/     - User-created workflow definitions
//	  secrets/    - API keys, credentials, etc. (managed by wasabi)
//	  templates/  - Reusable workflow templates
//	  config/     - Configuration files (themes, preferences)
//	  cache/      - Temporary/cached data
//
// File format: <name>.bento.json (for bentos), <name>.json (for others)
//
// # Usage
//
//	// Create a storage instance
//	storage := hangiri.NewDefaultStorage()
//
//	// Save a bento
//	err := storage.SaveBento(ctx, "my-workflow", definition)
//
//	// Load a bento by name
//	def, err := storage.LoadBento(ctx, "my-workflow")
//
//	// List all bentos
//	names, err := storage.ListBentos(ctx)
//
//	// Delete a bento
//	err := storage.DeleteBento(ctx, "my-workflow")
//
// Security: All names are validated to prevent directory traversal attacks.
package storage

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/Develonaut/bento/pkg/tui"
	"github.com/Develonaut/bento/pkg/neta"
)

// StorageType represents different types of storage subdirectories.
type StorageType string

const (
	StorageTypeBentos    StorageType = "bentos"
	StorageTypeSecrets   StorageType = "secrets"
	StorageTypeTemplates StorageType = "templates"
	StorageTypeConfig    StorageType = "config"
	StorageTypeCache     StorageType = "cache"
)

// Storage manages persistent storage of bento-related data.
type Storage struct {
	baseDir string
}

// New creates a new Storage instance with a custom base directory.
//
// baseDir is the root directory (typically ~/.bento/)
func New(baseDir string) *Storage {
	return &Storage{baseDir: expandHome(baseDir)}
}

// NewDefaultStorage creates a Storage instance using the configured bento home directory.
// Falls back to ~/.bento/ if no custom directory is configured.
func NewDefaultStorage() *Storage {
	return New(tui.LoadBentoHome())
}

// expandHome expands ~ to the user's home directory.
func expandHome(path string) string {
	if strings.HasPrefix(path, "~/") {
		home, err := os.UserHomeDir()
		if err == nil {
			return filepath.Join(home, path[2:])
		}
	}
	return path
}

// getStorageDir returns the full path to a storage subdirectory.
func (s *Storage) getStorageDir(storageType StorageType) string {
	return filepath.Join(s.baseDir, string(storageType))
}

// ensureStorageDir creates a storage subdirectory if it doesn't exist.
func (s *Storage) ensureStorageDir(storageType StorageType) error {
	dir := s.getStorageDir(storageType)
	return os.MkdirAll(dir, 0755)
}

// getBentoPath returns the full file path for a bento name.
func (s *Storage) getBentoPath(name string) string {
	// Strip .bento.json extension if present
	name = strings.TrimSuffix(name, ".bento.json")
	return filepath.Join(s.getStorageDir(StorageTypeBentos), name+".bento.json")
}

// SaveBento saves a bento definition to ~/.bento/bentos/
//
// The bento is saved as <name>.bento.json in the bentos directory.
// Returns an error if the name is invalid or if writing fails.
func (s *Storage) SaveBento(ctx context.Context, name string, def *neta.Definition) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}

	if err := validateName(name); err != nil {
		return err
	}

	if err := s.ensureStorageDir(StorageTypeBentos); err != nil {
		return err
	}

	data, err := s.marshal(def)
	if err != nil {
		return fmt.Errorf("failed to serialize bento '%s': %w", name, err)
	}

	path := s.getBentoPath(name)
	if err := os.WriteFile(path, data, 0644); err != nil {
		return fmt.Errorf("failed to write bento '%s': %w", name, err)
	}

	return nil
}

// LoadBento loads a bento definition from ~/.bento/bentos/
//
// Returns an error if the bento doesn't exist or cannot be parsed.
func (s *Storage) LoadBento(ctx context.Context, name string) (*neta.Definition, error) {
	if ctx.Err() != nil {
		return nil, ctx.Err()
	}

	if err := validateName(name); err != nil {
		return nil, err
	}

	path := s.getBentoPath(name)
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("bento '%s' not found", name)
		}
		return nil, fmt.Errorf("failed to read bento '%s': %w", name, err)
	}

	def, err := s.unmarshal(data)
	if err != nil {
		return nil, fmt.Errorf("failed to parse bento '%s': %w", name, err)
	}

	return def, nil
}

// ListBentos returns all saved bento names from ~/.bento/bentos/
func (s *Storage) ListBentos(ctx context.Context) ([]string, error) {
	if ctx.Err() != nil {
		return nil, ctx.Err()
	}

	if err := s.ensureStorageDir(StorageTypeBentos); err != nil {
		return nil, err
	}

	dir := s.getStorageDir(StorageTypeBentos)
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, fmt.Errorf("failed to list bentos: %w", err)
	}

	var names []string
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".bento.json") {
			name := strings.TrimSuffix(entry.Name(), ".bento.json")
			names = append(names, name)
		}
	}
	return names, nil
}

// DeleteBento removes a bento from ~/.bento/bentos/
func (s *Storage) DeleteBento(ctx context.Context, name string) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}

	if err := validateName(name); err != nil {
		return err
	}

	path := s.getBentoPath(name)
	if err := os.Remove(path); err != nil {
		if os.IsNotExist(err) {
			return fmt.Errorf("bento '%s' not found", name)
		}
		return fmt.Errorf("failed to delete bento '%s': %w", name, err)
	}

	return nil
}

// BentoExists checks if a bento exists in storage.
func (s *Storage) BentoExists(ctx context.Context, name string) bool {
	if err := validateName(name); err != nil {
		return false
	}
	path := s.getBentoPath(name)
	_, err := os.Stat(path)
	return err == nil
}
