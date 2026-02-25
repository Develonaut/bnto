// Package storage provides persistent storage for bnto data.
//
// Storage structure:
//
//	~/.bnto/
//	  bntos/     - User-created workflow definitions
//	  secrets/    - API keys, credentials, etc.
//	  templates/  - Reusable workflow templates
//	  config/     - Configuration files (themes, preferences)
//	  cache/      - Temporary/cached data
//
// File format: <name>.bnto.json (for bntos), <name>.json (for others)
//
// # Usage
//
//	// Create a storage instance
//	storage := storage.NewDefaultStorage()
//
//	// Save a bnto
//	err := storage.SaveBnto(ctx, "my-workflow", definition)
//
//	// Load a bnto by name
//	def, err := storage.LoadBnto(ctx, "my-workflow")
//
//	// List all bntos
//	names, err := storage.ListBntos(ctx)
//
//	// Delete a bnto
//	err := storage.DeleteBnto(ctx, "my-workflow")
//
// Security: All names are validated to prevent directory traversal attacks.
package storage

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/paths"
)

// StorageType represents different types of storage subdirectories.
type StorageType string

const (
	StorageTypeBntos    StorageType = "bntos"
	StorageTypeSecrets   StorageType = "secrets"
	StorageTypeTemplates StorageType = "templates"
	StorageTypeConfig    StorageType = "config"
	StorageTypeCache     StorageType = "cache"
)

// Storage manages persistent storage of bnto-related data.
type Storage struct {
	baseDir string
}

// New creates a new Storage instance with a custom base directory.
//
// baseDir is the root directory (typically ~/.bnto/)
func New(baseDir string) *Storage {
	return &Storage{baseDir: expandHome(baseDir)}
}

// NewDefaultStorage creates a Storage instance using the configured bnto home directory.
// Falls back to ~/.bnto/ if no custom directory is configured.
func NewDefaultStorage() *Storage {
	return New(paths.LoadBntoHome())
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

// getBntoPath returns the full file path for a bnto name.
func (s *Storage) getBntoPath(name string) string {
	// Strip .bnto.json extension if present
	name = strings.TrimSuffix(name, ".bnto.json")
	return filepath.Join(s.getStorageDir(StorageTypeBntos), name+".bnto.json")
}

// SaveBnto saves a bnto definition to ~/.bnto/bntos/
//
// The bnto is saved as <name>.bnto.json in the bntos directory.
// Returns an error if the name is invalid or if writing fails.
func (s *Storage) SaveBnto(ctx context.Context, name string, def *node.Definition) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}

	if err := validateName(name); err != nil {
		return err
	}

	if err := s.ensureStorageDir(StorageTypeBntos); err != nil {
		return err
	}

	data, err := s.marshal(def)
	if err != nil {
		return fmt.Errorf("failed to serialize bnto '%s': %w", name, err)
	}

	path := s.getBntoPath(name)
	if err := os.WriteFile(path, data, 0644); err != nil {
		return fmt.Errorf("failed to write bnto '%s': %w", name, err)
	}

	return nil
}

// LoadBnto loads a bnto definition from ~/.bnto/bntos/
//
// Returns an error if the bnto doesn't exist or cannot be parsed.
func (s *Storage) LoadBnto(ctx context.Context, name string) (*node.Definition, error) {
	if ctx.Err() != nil {
		return nil, ctx.Err()
	}

	if err := validateName(name); err != nil {
		return nil, err
	}

	path := s.getBntoPath(name)
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("bnto '%s' not found", name)
		}
		return nil, fmt.Errorf("failed to read bnto '%s': %w", name, err)
	}

	def, err := s.unmarshal(data)
	if err != nil {
		return nil, fmt.Errorf("failed to parse bnto '%s': %w", name, err)
	}

	return def, nil
}

// ListBntos returns all saved bnto names from ~/.bnto/bntos/
func (s *Storage) ListBntos(ctx context.Context) ([]string, error) {
	if ctx.Err() != nil {
		return nil, ctx.Err()
	}

	if err := s.ensureStorageDir(StorageTypeBntos); err != nil {
		return nil, err
	}

	dir := s.getStorageDir(StorageTypeBntos)
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, fmt.Errorf("failed to list bntos: %w", err)
	}

	var names []string
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".bnto.json") {
			name := strings.TrimSuffix(entry.Name(), ".bnto.json")
			names = append(names, name)
		}
	}
	return names, nil
}

// DeleteBnto removes a bnto from ~/.bnto/bntos/
func (s *Storage) DeleteBnto(ctx context.Context, name string) error {
	if ctx.Err() != nil {
		return ctx.Err()
	}

	if err := validateName(name); err != nil {
		return err
	}

	path := s.getBntoPath(name)
	if err := os.Remove(path); err != nil {
		if os.IsNotExist(err) {
			return fmt.Errorf("bnto '%s' not found", name)
		}
		return fmt.Errorf("failed to delete bnto '%s': %w", name, err)
	}

	return nil
}

// BntoExists checks if a bnto exists in storage.
func (s *Storage) BntoExists(ctx context.Context, name string) bool {
	if err := validateName(name); err != nil {
		return false
	}
	path := s.getBntoPath(name)
	_, err := os.Stat(path)
	return err == nil
}
