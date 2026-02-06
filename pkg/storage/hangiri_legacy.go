package storage

import (
	"context"
	"encoding/json"

	"github.com/Develonaut/bento/pkg/neta"
)

// Legacy methods for backward compatibility
// These maintain the old API but delegate to the new bento-specific methods.

// Save saves a bento definition using the legacy API.
// Deprecated: Use SaveBento instead.
func (s *Storage) Save(ctx context.Context, name string, def *neta.Definition) error {
	return s.SaveBento(ctx, name, def)
}

// Load loads a bento definition using the legacy API.
// Deprecated: Use LoadBento instead.
func (s *Storage) Load(ctx context.Context, name string) (*neta.Definition, error) {
	return s.LoadBento(ctx, name)
}

// List returns all saved bento names using the legacy API.
// Deprecated: Use ListBentos instead.
func (s *Storage) List(ctx context.Context) ([]string, error) {
	return s.ListBentos(ctx)
}

// Delete removes a bento using the legacy API.
// Deprecated: Use DeleteBento instead.
func (s *Storage) Delete(ctx context.Context, name string) error {
	return s.DeleteBento(ctx, name)
}

// marshal serializes a definition to JSON with indentation.
func (s *Storage) marshal(def *neta.Definition) ([]byte, error) {
	return json.MarshalIndent(def, "", "  ")
}

// unmarshal deserializes JSON data to a definition.
func (s *Storage) unmarshal(data []byte) (*neta.Definition, error) {
	var def neta.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, err
	}
	return &def, nil
}
