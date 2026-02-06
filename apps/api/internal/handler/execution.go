package handler

import (
	"net/http"

	"github.com/Develonaut/bento-api/internal/execution"
)

// GetExecution returns an http.HandlerFunc for GET /api/executions/{id}.
func GetExecution(mgr *execution.Manager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id := r.PathValue("id")

		exec := mgr.Get(id)
		if exec == nil {
			writeError(w, http.StatusNotFound, "execution not found")
			return
		}

		writeJSON(w, http.StatusOK, exec)
	}
}
