package api

import (
	"testing"

	"github.com/Develonaut/bento/pkg/storage"
)

func TestNew(t *testing.T) {
	reg := DefaultRegistry()
	store := storage.New(t.TempDir())

	svc := New(reg, store)
	if svc == nil {
		t.Fatal("expected non-nil service")
	}
	if svc.registry == nil {
		t.Error("expected registry to be set")
	}
	if svc.storage == nil {
		t.Error("expected storage to be set")
	}
	if svc.validator == nil {
		t.Error("expected validator to be set")
	}
}

func TestDefaultRegistry(t *testing.T) {
	reg := DefaultRegistry()

	expected := []string{
		"edit-fields", "file-system", "group", "http-request",
		"image", "loop", "parallel", "shell-command",
		"spreadsheet", "transform",
	}

	for _, name := range expected {
		if !reg.Has(name) {
			t.Errorf("expected registry to have %q", name)
		}
	}

	if got := len(reg.List()); got != len(expected) {
		t.Errorf("expected %d node types, got %d", len(expected), got)
	}
}
