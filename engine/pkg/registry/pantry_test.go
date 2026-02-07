package registry_test

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"testing"

	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/registry"
)

// MockNode is a test implementation of node.Executable.
type MockNode struct {
	name string
}

func (m *MockNode) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	return map[string]interface{}{"mock": m.name}, nil
}

// TestPantry_RegisterFactoryAndGetNew tests registering a factory and
// retrieving new instances.
func TestPantry_RegisterFactoryAndGetNew(t *testing.T) {
	p := registry.New()

	// Register a factory function
	p.RegisterFactory("test-node", func() node.Executable {
		return &MockNode{name: "test-node"}
	})

	// Retrieve a new instance
	instance, err := p.GetNew("test-node")
	if err != nil {
		t.Fatalf("GetNew failed: %v", err)
	}

	if instance == nil {
		t.Fatal("GetNew returned nil instance")
	}

	// Execute to verify it works
	result, err := instance.Execute(context.Background(), nil)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	resultMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Result is not a map")
	}

	if resultMap["mock"] != "test-node" {
		t.Errorf("Expected mock='test-node', got %v", resultMap["mock"])
	}
}

// TestPantry_GetUnregistered tests that getting an unregistered type returns
// a clear error message that includes the type name and available types.
func TestPantry_GetUnregistered(t *testing.T) {
	p := registry.New()

	// Register some types so the error can list them
	p.RegisterFactory("type-a", func() node.Executable {
		return &MockNode{name: "a"}
	})
	p.RegisterFactory("type-b", func() node.Executable {
		return &MockNode{name: "b"}
	})

	// Try to get unregistered type
	_, err := p.GetNew("nonexistent-type")
	if err == nil {
		t.Fatal("Expected error for unregistered type")
	}

	errMsg := err.Error()

	// Error should mention it's not registered
	if !strings.Contains(errMsg, "not registered") {
		t.Errorf("Error should mention 'not registered': %v", err)
	}

	// Error should mention the type name
	if !strings.Contains(errMsg, "nonexistent-type") {
		t.Errorf("Error should mention the type name: %v", err)
	}

	// Error should list available types
	if !strings.Contains(errMsg, "type-a") || !strings.Contains(errMsg, "type-b") {
		t.Errorf("Error should list available types: %v", err)
	}
}

// TestPantry_List tests listing all registered types.
func TestPantry_List(t *testing.T) {
	p := registry.New()

	// Register multiple types
	p.RegisterFactory("type-a", func() node.Executable {
		return &MockNode{name: "a"}
	})
	p.RegisterFactory("type-b", func() node.Executable {
		return &MockNode{name: "b"}
	})
	p.RegisterFactory("type-c", func() node.Executable {
		return &MockNode{name: "c"}
	})

	// List all types
	types := p.List()

	if len(types) != 3 {
		t.Errorf("Expected 3 types, got %d", len(types))
	}

	// Verify all types are present
	typeMap := make(map[string]bool)
	for _, typeName := range types {
		typeMap[typeName] = true
	}

	if !typeMap["type-a"] || !typeMap["type-b"] || !typeMap["type-c"] {
		t.Errorf("Not all types were listed. Got: %v", types)
	}

	// Verify list is sorted (consistent output)
	if types[0] != "type-a" || types[1] != "type-b" || types[2] != "type-c" {
		t.Errorf("Types should be sorted alphabetically, got: %v", types)
	}
}

// TestPantry_ConcurrentAccess tests thread-safe concurrent access with 100
// goroutines reading simultaneously.
func TestPantry_ConcurrentAccess(t *testing.T) {
	p := registry.New()

	// Register initial types
	for i := 0; i < 10; i++ {
		typeNum := i
		p.RegisterFactory(fmt.Sprintf("type-%d", i), func() node.Executable {
			return &MockNode{name: fmt.Sprintf("node-%d", typeNum)}
		})
	}

	// Concurrent reads from 100 goroutines
	var wg sync.WaitGroup
	errChan := make(chan error, 100)

	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()

			typeNum := i % 10
			typeName := fmt.Sprintf("type-%d", typeNum)

			// Test GetNew
			_, err := p.GetNew(typeName)
			if err != nil {
				errChan <- fmt.Errorf("concurrent GetNew failed: %v", err)
				return
			}

			// Test List
			types := p.List()
			if len(types) != 10 {
				errChan <- fmt.Errorf("concurrent List returned %d types, expected 10", len(types))
				return
			}

			// Test Has
			if !p.Has(typeName) {
				errChan <- fmt.Errorf("concurrent Has failed for %s", typeName)
				return
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	// Check for errors
	for err := range errChan {
		t.Error(err)
	}
}

// TestPantry_FactoryPattern tests that each GetNew call returns a NEW
// instance, not a shared one.
func TestPantry_FactoryPattern(t *testing.T) {
	p := registry.New()

	// Register a factory
	p.RegisterFactory("test-type", func() node.Executable {
		return &MockNode{name: "test"}
	})

	// Get two instances
	instance1, err := p.GetNew("test-type")
	if err != nil {
		t.Fatalf("GetNew failed: %v", err)
	}

	instance2, err := p.GetNew("test-type")
	if err != nil {
		t.Fatalf("GetNew failed: %v", err)
	}

	// Should be different instances (different memory addresses)
	if instance1 == instance2 {
		t.Error("GetNew should return NEW instances, not the same one")
	}
}

// TestPantry_DuplicateRegistration tests that duplicate registration
// overwrites the previous registration (last registration wins).
func TestPantry_DuplicateRegistration(t *testing.T) {
	p := registry.New()

	// Register first factory
	p.RegisterFactory("test-type", func() node.Executable {
		return &MockNode{name: "first"}
	})

	// Register second factory with same name
	p.RegisterFactory("test-type", func() node.Executable {
		return &MockNode{name: "second"}
	})

	// Get instance
	instance, err := p.GetNew("test-type")
	if err != nil {
		t.Fatalf("GetNew failed: %v", err)
	}

	// Execute and verify it's the second one
	result, err := instance.Execute(context.Background(), nil)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	resultMap := result.(map[string]interface{})
	if resultMap["mock"] != "second" {
		t.Error("Duplicate registration should overwrite previous (last wins)")
	}
}

// TestPantry_Has tests the Has method for checking if a type is registered.
func TestPantry_Has(t *testing.T) {
	p := registry.New()

	// Register a type
	p.RegisterFactory("existing-type", func() node.Executable {
		return &MockNode{name: "exists"}
	})

	// Test Has with existing type
	if !p.Has("existing-type") {
		t.Error("Has should return true for registered type")
	}

	// Test Has with non-existing type
	if p.Has("nonexistent-type") {
		t.Error("Has should return false for unregistered type")
	}
}

// TestPantry_EmptyPantry tests operations on an empty registry.
func TestPantry_EmptyPantry(t *testing.T) {
	p := registry.New()

	// List should return empty slice
	types := p.List()
	if len(types) != 0 {
		t.Errorf("Empty registry should list 0 types, got %d", len(types))
	}

	// GetNew should return error
	_, err := p.GetNew("any-type")
	if err == nil {
		t.Error("GetNew on empty registry should return error")
	}

	// Has should return false
	if p.Has("any-type") {
		t.Error("Has on empty registry should return false")
	}
}
