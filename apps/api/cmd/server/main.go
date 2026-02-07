// Package main is the entry point for the bnto API server.
//
// The API server exposes the bnto engine over HTTP, allowing
// the web frontend to run, validate, and manage workflows.
package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"

	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/storage"

	"github.com/Develonaut/bnto-api/internal/execution"
	"github.com/Develonaut/bnto-api/internal/server"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	reg := api.DefaultRegistry()
	store := storage.NewDefaultStorage()
	svc := api.New(reg, store)
	mgr := execution.NewManager()

	// Background cleanup of completed executions (1 hour TTL).
	go cleanupLoop(mgr, time.Hour, 5*time.Minute)

	handler := server.New(svc, mgr)

	srv := &http.Server{
		Addr:         ":" + port,
		Handler:      handler,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 30 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Graceful shutdown on interrupt.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	go func() {
		fmt.Printf("bnto api listening on :%s\n", port)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("server error: %v", err)
		}
	}()

	<-ctx.Done()
	fmt.Println("\nshutting down...")

	shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := srv.Shutdown(shutdownCtx); err != nil {
		log.Fatalf("shutdown error: %v", err)
	}
}

// cleanupLoop periodically removes completed executions older than ttl.
func cleanupLoop(mgr *execution.Manager, ttl, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()
	for range ticker.C {
		mgr.CleanupBefore(time.Now().Add(-ttl))
	}
}
