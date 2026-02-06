// Package pantry provides a thread-safe registry for neta types.
//
// Like a well-organized pantry stores ingredients, this package stores and
// provides access to all available neta implementations.
//
// # Usage
//
//	p := pantry.New()
//
//	// Register a neta factory
//	p.RegisterFactory("http-request", func() neta.Executable {
//	    return httpneta.New()
//	})
//
//	// Retrieve a new instance
//	netaInstance, err := p.GetNew("http-request")
//
//	// List all registered types
//	types := p.List()
//
//	// Check if a type exists
//	exists := p.Has("http-request")
//
// # Thread Safety
//
// The pantry uses sync.RWMutex to ensure safe concurrent access from multiple
// goroutines. Multiple readers can access simultaneously (GetNew, List, Has),
// but writes (RegisterFactory) are exclusive.
//
// # Factory Pattern
//
// The pantry uses the factory pattern to create NEW instances of neta on each
// GetNew() call. This prevents shared state between neta instances, which is
// critical for:
//   - Running multiple workflow iterations without state leakage
//   - Parallel execution of neta in the future
//   - Isolated testing
//
// Learn more:
//   - sync.RWMutex: https://pkg.go.dev/sync#RWMutex
//   - Factory pattern: https://en.wikipedia.org/wiki/Factory_method_pattern
package registry

import (
	"fmt"
	"sort"
	"sync"

	"github.com/Develonaut/bento/pkg/neta"
)

// Factory is a function that creates a new neta instance.
//
// Each call to the factory should return a NEW instance, not a shared one.
// This ensures that neta instances are isolated and don't share state.
//
// Example:
//
//	factory := func() neta.Executable {
//	    return httpneta.New()  // Returns new instance
//	}
type Factory func() neta.Executable

// Pantry is a thread-safe registry for neta types.
//
// It stores factory functions that create new instances of neta on demand.
// The pantry uses sync.RWMutex for thread-safe concurrent access.
type Registry struct {
	mu        sync.RWMutex       // Protects the factories map
	factories map[string]Factory // Type name -> factory function
}

// New creates a new empty Pantry.
//
// The pantry starts empty. Use RegisterFactory to add neta types.
func New() *Registry {
	return &Registry{
		factories: make(map[string]Factory),
	}
}

// RegisterFactory registers a neta type with a factory function.
//
// The factory function should return a NEW instance each time it's called.
// If a type is already registered, this will overwrite it (last wins).
//
// Example:
//
//	p.RegisterFactory("http-request", func() neta.Executable {
//	    return httpneta.New()
//	})
//
// Thread-safe: Uses write lock (exclusive access).
func (p *Registry) RegisterFactory(typeName string, factory Factory) {
	p.mu.Lock()
	defer p.mu.Unlock()

	p.factories[typeName] = factory
}

// GetNew creates and returns a new instance of the specified neta type.
//
// Returns an error if the type is not registered. The error message includes
// the requested type name and a list of all available types.
//
// Example:
//
//	instance, err := p.GetNew("http-request")
//	if err != nil {
//	    log.Fatalf("Failed to get neta: %v", err)
//	}
//
// Thread-safe: Uses read lock (allows concurrent access).
func (p *Registry) GetNew(typeName string) (neta.Executable, error) {
	p.mu.RLock()
	defer p.mu.RUnlock()

	factory, exists := p.factories[typeName]
	if !exists {
		return nil, fmt.Errorf(
			"neta type '%s' is not registered in pantry. Available types: %v",
			typeName,
			p.listUnsafe(),
		)
	}

	return factory(), nil
}

// List returns all registered neta type names in sorted order.
//
// Returns an empty slice if no types are registered.
// The list is sorted alphabetically for consistent output.
//
// Example:
//
//	types := p.List()
//	for _, typeName := range types {
//	    fmt.Println(typeName)
//	}
//
// Thread-safe: Uses read lock (allows concurrent access).
func (p *Registry) List() []string {
	p.mu.RLock()
	defer p.mu.RUnlock()

	return p.listUnsafe()
}

// listUnsafe returns all type names without locking.
//
// IMPORTANT: Caller must hold either RLock or Lock before calling.
// This is an internal helper to avoid duplicate locking.
func (p *Registry) listUnsafe() []string {
	types := make([]string, 0, len(p.factories))

	for typeName := range p.factories {
		types = append(types, typeName)
	}

	// Sort for consistent output
	sort.Strings(types)

	return types
}

// Has checks if a neta type is registered.
//
// Returns true if the type is registered, false otherwise.
//
// Example:
//
//	if p.Has("http-request") {
//	    fmt.Println("HTTP request neta is available")
//	}
//
// Thread-safe: Uses read lock (allows concurrent access).
func (p *Registry) Has(typeName string) bool {
	p.mu.RLock()
	defer p.mu.RUnlock()

	_, exists := p.factories[typeName]
	return exists
}
