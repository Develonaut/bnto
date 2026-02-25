// Package server assembles the HTTP router and middleware.
package server

import (
	"net/http"

	"github.com/Develonaut/bnto/pkg/api"

	"github.com/Develonaut/bnto-api/internal/execution"
	"github.com/Develonaut/bnto-api/internal/handler"
	"github.com/Develonaut/bnto-api/internal/r2"
)

// New creates an http.Handler with all routes registered.
// Pass nil for store to disable R2 file transit (local-only mode).
func New(svc *api.BntoService, mgr *execution.Manager, store r2.ObjectStore) http.Handler {
	mux := http.NewServeMux()

	mux.HandleFunc("GET /health", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
	})
	mux.HandleFunc("POST /api/run", handler.RunWorkflow(svc, mgr, store))
	mux.HandleFunc("GET /api/executions/{id}", handler.GetExecution(mgr))
	mux.HandleFunc("POST /api/validate", handler.Validate(svc))
	mux.HandleFunc("GET /api/workflows", handler.ListWorkflows(svc))
	mux.HandleFunc("GET /api/workflows/{name}", handler.GetWorkflow(svc))
	mux.HandleFunc("POST /api/workflows", handler.SaveWorkflow(svc))
	mux.HandleFunc("DELETE /api/workflows/{name}", handler.DeleteWorkflow(svc))

	return cors(mux)
}

// cors wraps a handler with permissive CORS headers for development.
func cors(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}

		next.ServeHTTP(w, r)
	})
}
