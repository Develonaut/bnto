package handler

import (
	"context"
	"crypto/rand"
	"fmt"
	"net/http"
	"time"

	"github.com/Develonaut/bento/pkg/api"
	"github.com/Develonaut/bento/pkg/node"

	"github.com/Develonaut/bento-api/internal/execution"
)

// runRequest is the JSON body for POST /api/run.
type runRequest struct {
	Definition node.Definition `json:"definition"`
	Timeout    string          `json:"timeout,omitempty"`
}

// RunWorkflow returns an http.HandlerFunc for POST /api/run.
// It starts an async execution and returns 202 with the execution ID.
func RunWorkflow(svc *api.BentoService, mgr *execution.Manager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var req runRequest
		if err := decodeBody(r, &req); err != nil {
			writeError(w, http.StatusBadRequest, err.Error())
			return
		}

		var timeout time.Duration
		if req.Timeout != "" {
			var err error
			timeout, err = time.ParseDuration(req.Timeout)
			if err != nil {
				writeError(w, http.StatusBadRequest, fmt.Sprintf("invalid timeout: %s", err))
				return
			}
		}

		id := newExecutionID()
		mgr.Create(id)

		go runAsync(svc, mgr, id, &req.Definition, timeout)

		writeJSON(w, http.StatusAccepted, map[string]string{"id": id})
	}
}

// runAsync executes a workflow in the background, updating the manager.
func runAsync(svc *api.BentoService, mgr *execution.Manager, id string, def *node.Definition, timeout time.Duration) {
	mgr.SetRunning(id)

	opts := api.RunOptions{
		Timeout: timeout,
		OnProgress: func(nodeID, status string) {
			mgr.AddProgress(id, nodeID, status)
		},
	}

	result, err := svc.RunWorkflow(context.Background(), def, opts)
	if err != nil {
		mgr.Fail(id, err.Error())
		return
	}
	mgr.Complete(id, result)
}

// newExecutionID generates a random execution ID.
func newExecutionID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:])
}
