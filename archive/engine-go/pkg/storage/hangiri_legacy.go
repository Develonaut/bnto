package storage

import (
	"context"
	"encoding/json"

	"github.com/Develonaut/bnto/pkg/node"
)

// Legacy methods for backward compatibility
// These maintain the old API but delegate to the new bnto-specific methods.

// Save saves a bnto definition using the legacy API.
// Deprecated: Use SaveBnto instead.
func (s *Storage) Save(ctx context.Context, name string, def *node.Definition) error {
	return s.SaveBnto(ctx, name, def)
}

// Load loads a bnto definition using the legacy API.
// Deprecated: Use LoadBnto instead.
func (s *Storage) Load(ctx context.Context, name string) (*node.Definition, error) {
	return s.LoadBnto(ctx, name)
}

// List returns all saved bnto names using the legacy API.
// Deprecated: Use ListBntos instead.
func (s *Storage) List(ctx context.Context) ([]string, error) {
	return s.ListBntos(ctx)
}

// Delete removes a bnto using the legacy API.
// Deprecated: Use DeleteBnto instead.
func (s *Storage) Delete(ctx context.Context, name string) error {
	return s.DeleteBnto(ctx, name)
}

// marshal serializes a definition to JSON with indentation.
func (s *Storage) marshal(def *node.Definition) ([]byte, error) {
	return json.MarshalIndent(def, "", "  ")
}

// unmarshal deserializes JSON data to a definition.
func (s *Storage) unmarshal(data []byte) (*node.Definition, error) {
	var def node.Definition
	if err := json.Unmarshal(data, &def); err != nil {
		return nil, err
	}
	return &def, nil
}
