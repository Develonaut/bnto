package integration

import (
	"github.com/Develonaut/bento/pkg/api"
	"github.com/Develonaut/bento/pkg/registry"
)

// createRegistry returns a registry with all built-in node types.
func createRegistry() *registry.Registry {
	return api.DefaultRegistry()
}
