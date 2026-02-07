package integration

import (
	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/registry"
)

// createRegistry returns a registry with all built-in node types.
func createRegistry() *registry.Registry {
	return api.DefaultRegistry()
}
