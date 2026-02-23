package handler

import (
	"context"
	"crypto/rand"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/node"

	"github.com/Develonaut/bnto-api/internal/execution"
	"github.com/Develonaut/bnto-api/internal/r2"
)

// runRequest is the JSON body for POST /api/run.
type runRequest struct {
	Definition node.Definition `json:"definition"`
	Timeout    string          `json:"timeout,omitempty"`
	SessionID  string          `json:"sessionId,omitempty"`
}

// envMu serializes executions that set INPUT_DIR/OUTPUT_DIR env vars.
// The Go engine resolves templates from os.Environ() at context creation time,
// so concurrent executions with different paths would clobber each other.
// Phase 1 limitation: one R2-backed execution at a time. Non-R2 executions
// are unaffected. Future improvement: add Env map to engine RunOptions.
var envMu sync.Mutex

// RunWorkflow returns an http.HandlerFunc for POST /api/run.
// It starts an async execution and returns 202 with the execution ID.
func RunWorkflow(svc *api.BntoService, mgr *execution.Manager, store r2.ObjectStore) http.HandlerFunc {
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

		if req.SessionID != "" && store == nil {
			writeError(w, http.StatusServiceUnavailable, "file transit not configured")
			return
		}

		id := newExecutionID()
		mgr.Create(id)

		if req.SessionID != "" {
			go runWithTransit(svc, mgr, store, id, &req.Definition, timeout, req.SessionID)
		} else {
			go runAsync(svc, mgr, id, &req.Definition, timeout)
		}

		writeJSON(w, http.StatusAccepted, map[string]string{"id": id})
	}
}

// runAsync executes a workflow in the background, updating the manager.
func runAsync(svc *api.BntoService, mgr *execution.Manager, id string, def *node.Definition, timeout time.Duration) {
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

// runWithTransit executes a workflow with R2 file transit:
// download input files → execute → upload output files.
func runWithTransit(svc *api.BntoService, mgr *execution.Manager, store r2.ObjectStore, id string, def *node.Definition, timeout time.Duration, sessionID string) {
	mgr.SetRunning(id)

	files, err := transitExec(svc, mgr, store, id, def, timeout, sessionID)
	if err != nil {
		mgr.Fail(id, err.Error())
		return
	}
	mgr.CompleteWithFiles(id, files.result, files.outputFiles)
}

// transitResult holds the output of a transit execution.
type transitResult struct {
	result      *api.RunResult
	outputFiles []execution.OutputFile
}

// transitExec performs the download→execute→upload pipeline, returning an error
// on any step failure. Temp directory is cleaned up on return.
func transitExec(svc *api.BntoService, mgr *execution.Manager, store r2.ObjectStore, id string, def *node.Definition, timeout time.Duration, sessionID string) (*transitResult, error) {
	ctx := context.Background()

	tmpDir, err := os.MkdirTemp("", "bnto-exec-*")
	if err != nil {
		return nil, fmt.Errorf("creating temp dir: %w", err)
	}
	defer os.RemoveAll(tmpDir)

	inputDir := filepath.Join(tmpDir, "input")
	outputDir := filepath.Join(tmpDir, "output")

	if err := r2.DownloadSession(ctx, store, sessionID, inputDir); err != nil {
		return nil, fmt.Errorf("downloading input files: %w", err)
	}

	// Input files are now local — delete from R2 to free transit storage.
	// Best-effort: log errors but don't fail the execution.
	r2.CleanupSessionBestEffort(ctx, store, sessionID)

	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		return nil, fmt.Errorf("creating output dir: %w", err)
	}

	result, err := executeWithEnv(ctx, svc, mgr, id, def, timeout, inputDir, outputDir)
	if err != nil {
		return nil, err
	}

	uploaded, err := r2.UploadOutputs(ctx, store, id, outputDir)
	if err != nil {
		return nil, fmt.Errorf("uploading output files: %w", err)
	}

	return &transitResult{result: result, outputFiles: toOutputFiles(uploaded)}, nil
}

// executeWithEnv runs a workflow with INPUT_DIR and OUTPUT_DIR env vars set.
// Uses a mutex to prevent concurrent env var clobbering.
func executeWithEnv(ctx context.Context, svc *api.BntoService, mgr *execution.Manager, id string, def *node.Definition, timeout time.Duration, inputDir, outputDir string) (*api.RunResult, error) {
	envMu.Lock()
	defer envMu.Unlock()

	os.Setenv("INPUT_DIR", inputDir)
	os.Setenv("OUTPUT_DIR", outputDir)
	defer os.Unsetenv("INPUT_DIR")
	defer os.Unsetenv("OUTPUT_DIR")

	opts := api.RunOptions{
		Timeout: timeout,
		OnProgress: func(nodeID, status string) {
			mgr.AddProgress(id, nodeID, status)
		},
	}

	return svc.RunWorkflow(ctx, def, opts)
}

// toOutputFiles converts r2.FileInfo to execution.OutputFile.
func toOutputFiles(files []r2.FileInfo) []execution.OutputFile {
	out := make([]execution.OutputFile, len(files))
	for i, f := range files {
		out[i] = execution.OutputFile{
			Key:         f.Key,
			Name:        f.Name,
			SizeBytes:   f.SizeBytes,
			ContentType: f.ContentType,
		}
	}
	return out
}

// newExecutionID generates a random execution ID.
func newExecutionID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:])
}
