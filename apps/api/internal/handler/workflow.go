package handler

import (
	"net/http"
	"strings"

	"github.com/Develonaut/bento/pkg/api"
	"github.com/Develonaut/bento/pkg/node"
)

// ListWorkflows returns an http.HandlerFunc for GET /api/workflows.
func ListWorkflows(svc *api.BentoService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		summaries, err := svc.ListWorkflows(r.Context())
		if err != nil {
			writeError(w, http.StatusInternalServerError, err.Error())
			return
		}
		writeJSON(w, http.StatusOK, summaries)
	}
}

// GetWorkflow returns an http.HandlerFunc for GET /api/workflows/{name}.
func GetWorkflow(svc *api.BentoService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		name := r.PathValue("name")
		def, err := svc.GetWorkflow(r.Context(), name)
		if err != nil {
			if isNotFound(err) {
				writeError(w, http.StatusNotFound, err.Error())
				return
			}
			writeError(w, http.StatusInternalServerError, err.Error())
			return
		}
		writeJSON(w, http.StatusOK, def)
	}
}

// saveWorkflowRequest is the JSON body for POST /api/workflows.
type saveWorkflowRequest struct {
	Name       string          `json:"name"`
	Definition node.Definition `json:"definition"`
}

// SaveWorkflow returns an http.HandlerFunc for POST /api/workflows.
func SaveWorkflow(svc *api.BentoService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var req saveWorkflowRequest
		if err := decodeBody(r, &req); err != nil {
			writeError(w, http.StatusBadRequest, err.Error())
			return
		}
		if req.Name == "" {
			writeError(w, http.StatusBadRequest, "name is required")
			return
		}

		if err := svc.SaveWorkflow(r.Context(), req.Name, &req.Definition); err != nil {
			writeError(w, http.StatusInternalServerError, err.Error())
			return
		}
		writeJSON(w, http.StatusCreated, map[string]string{"name": req.Name})
	}
}

// DeleteWorkflow returns an http.HandlerFunc for DELETE /api/workflows/{name}.
func DeleteWorkflow(svc *api.BentoService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		name := r.PathValue("name")
		if err := svc.DeleteWorkflow(r.Context(), name); err != nil {
			if isNotFound(err) {
				writeError(w, http.StatusNotFound, err.Error())
				return
			}
			writeError(w, http.StatusInternalServerError, err.Error())
			return
		}
		w.WriteHeader(http.StatusNoContent)
	}
}

// isNotFound checks if an error indicates a resource was not found.
func isNotFound(err error) bool {
	return err != nil && strings.Contains(err.Error(), "not found")
}
