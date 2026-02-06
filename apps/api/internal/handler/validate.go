package handler

import (
	"net/http"

	"github.com/Develonaut/bento/pkg/api"
	"github.com/Develonaut/bento/pkg/node"
)

// Validate returns an http.HandlerFunc for POST /api/validate.
func Validate(svc *api.BentoService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var def node.Definition
		if err := decodeBody(r, &def); err != nil {
			writeError(w, http.StatusBadRequest, err.Error())
			return
		}

		result, err := svc.ValidateWorkflow(r.Context(), &def)
		if err != nil {
			writeError(w, http.StatusInternalServerError, err.Error())
			return
		}

		writeJSON(w, http.StatusOK, result)
	}
}
