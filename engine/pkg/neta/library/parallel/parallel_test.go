package parallel_test

import (
	"context"
	"sync/atomic"
	"testing"
	"time"

	"github.com/Develonaut/bento/pkg/neta/library/parallel"
)

// TestParallel_ConcurrentExecution tests running multiple tasks concurrently
func TestParallel_ConcurrentExecution(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	// Create tasks that would take 3 seconds if run sequentially
	tasks := []interface{}{
		map[string]interface{}{"duration": 100}, // 100ms
		map[string]interface{}{"duration": 100},
		map[string]interface{}{"duration": 100},
	}

	params := map[string]interface{}{
		"tasks": tasks,
	}

	start := time.Now()
	result, err := p.Execute(ctx, params)
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Should complete in ~100ms (concurrent), not 300ms (sequential)
	if duration > 200*time.Millisecond {
		t.Errorf("Took too long: %v (expected ~100ms with concurrency)", duration)
	}

	output, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map[string]interface{} result")
	}

	results, ok := output["results"].([]interface{})
	if !ok {
		t.Fatal("Expected results to be []interface{}")
	}

	if len(results) != 3 {
		t.Errorf("len(results) = %d, want 3", len(results))
	}
}

// TestParallel_WorkerPool tests limiting concurrency with worker pool
func TestParallel_WorkerPool(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	// Track max concurrent workers
	var currentWorkers int32
	var maxWorkers int32

	tasks := make([]interface{}, 10)
	for i := 0; i < 10; i++ {
		tasks[i] = map[string]interface{}{
			"id": i,
		}
	}

	params := map[string]interface{}{
		"tasks":      tasks,
		"maxWorkers": 3, // Limit to 3 concurrent workers
		"_onStart": func() {
			current := atomic.AddInt32(&currentWorkers, 1)
			for {
				max := atomic.LoadInt32(&maxWorkers)
				if current <= max || atomic.CompareAndSwapInt32(&maxWorkers, max, current) {
					break
				}
			}
		},
		"_onComplete": func() {
			atomic.AddInt32(&currentWorkers, -1)
		},
	}

	result, err := p.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	results := output["results"].([]interface{})

	if len(results) != 10 {
		t.Errorf("len(results) = %d, want 10", len(results))
	}

	max := atomic.LoadInt32(&maxWorkers)
	if max > 3 {
		t.Errorf("maxWorkers = %d, want <= 3", max)
	}
}

// TestParallel_ErrorHandling_FailFast tests fail fast error handling
func TestParallel_ErrorHandling_FailFast(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	tasks := []interface{}{
		map[string]interface{}{"shouldError": false},
		map[string]interface{}{"shouldError": true}, // This will cause error
		map[string]interface{}{"shouldError": false},
	}

	params := map[string]interface{}{
		"tasks":         tasks,
		"errorStrategy": "failFast",
		"_shouldError": func(task map[string]interface{}) bool {
			val, _ := task["shouldError"].(bool)
			return val
		},
	}

	_, err := p.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error with failFast strategy, got nil")
	}
}

// TestParallel_ErrorHandling_CollectAll tests collecting all errors
func TestParallel_ErrorHandling_CollectAll(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	tasks := []interface{}{
		map[string]interface{}{"shouldError": false},
		map[string]interface{}{"shouldError": true},
		map[string]interface{}{"shouldError": true},
		map[string]interface{}{"shouldError": false},
	}

	params := map[string]interface{}{
		"tasks":         tasks,
		"errorStrategy": "collectAll",
		"_shouldError": func(task map[string]interface{}) bool {
			val, _ := task["shouldError"].(bool)
			return val
		},
	}

	result, err := p.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute should not fail with collectAll, got: %v", err)
	}

	output := result.(map[string]interface{})
	errors, ok := output["errors"].([]interface{})
	if !ok {
		t.Fatal("Expected errors array")
	}

	// Should have 2 errors collected
	if len(errors) != 2 {
		t.Errorf("len(errors) = %d, want 2", len(errors))
	}
}

// TestParallel_ContextCancellation tests cancellation support
func TestParallel_ContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	p := parallel.New()

	tasks := make([]interface{}, 100)
	for i := 0; i < 100; i++ {
		tasks[i] = map[string]interface{}{"id": i}
	}

	var started atomic.Int32

	params := map[string]interface{}{
		"tasks": tasks,
		"_onStart": func() {
			started.Add(1)
			// Simulate work to give time for cancellation
			time.Sleep(50 * time.Millisecond)
		},
	}

	// Cancel after a few tasks start
	go func() {
		for started.Load() < 5 {
			time.Sleep(1 * time.Millisecond)
		}
		cancel()
	}()

	_, err := p.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected context cancellation error, got nil")
	}

	if err != context.Canceled {
		t.Errorf("Expected context.Canceled error, got: %v", err)
	}
}

// TestParallel_EmptyTasks tests empty tasks array
func TestParallel_EmptyTasks(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	params := map[string]interface{}{
		"tasks": []interface{}{},
	}

	result, err := p.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	results := output["results"].([]interface{})

	if len(results) != 0 {
		t.Errorf("len(results) = %d, want 0", len(results))
	}
}

// TestParallel_DefaultWorkers tests default worker pool size
func TestParallel_DefaultWorkers(t *testing.T) {
	ctx := context.Background()

	p := parallel.New()

	tasks := make([]interface{}, 5)
	for i := 0; i < 5; i++ {
		tasks[i] = map[string]interface{}{"id": i}
	}

	params := map[string]interface{}{
		"tasks": tasks,
		// No maxWorkers specified - should use default
	}

	result, err := p.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	results := output["results"].([]interface{})

	if len(results) != 5 {
		t.Errorf("len(results) = %d, want 5", len(results))
	}
}
