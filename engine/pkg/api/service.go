package api

import (
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/registry"
	"github.com/Develonaut/bento/pkg/storage"
	"github.com/Develonaut/bento/pkg/validator"

	editfields "github.com/Develonaut/bento/pkg/node/library/editfields"
	filesystem "github.com/Develonaut/bento/pkg/node/library/filesystem"
	group "github.com/Develonaut/bento/pkg/node/library/group"
	httpnode "github.com/Develonaut/bento/pkg/node/library/http"
	image "github.com/Develonaut/bento/pkg/node/library/image"
	loop "github.com/Develonaut/bento/pkg/node/library/loop"
	parallel "github.com/Develonaut/bento/pkg/node/library/parallel"
	shellcommand "github.com/Develonaut/bento/pkg/node/library/shellcommand"
	spreadsheet "github.com/Develonaut/bento/pkg/node/library/spreadsheet"
	transform "github.com/Develonaut/bento/pkg/node/library/transform"
)

// BentoService provides the shared service layer for bento operations.
// Used by CLI, HTTP server (apps/api/), and Wails desktop app.
type BentoService struct {
	registry  *registry.Registry
	storage   *storage.Storage
	validator *validator.Validator
}

// New creates a BentoService with the given registry and storage.
// A validator is created automatically.
func New(reg *registry.Registry, store *storage.Storage) *BentoService {
	return &BentoService{
		registry:  reg,
		storage:   store,
		validator: validator.New(),
	}
}

// DefaultRegistry creates a registry populated with all built-in node types.
func DefaultRegistry() *registry.Registry {
	r := registry.New()

	r.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })
	r.RegisterFactory("file-system", func() node.Executable { return filesystem.New() })
	r.RegisterFactory("group", func() node.Executable { return group.New() })
	r.RegisterFactory("http-request", func() node.Executable { return httpnode.New() })
	r.RegisterFactory("image", func() node.Executable { return image.New() })
	r.RegisterFactory("loop", func() node.Executable { return loop.New() })
	r.RegisterFactory("parallel", func() node.Executable { return parallel.New() })
	r.RegisterFactory("shell-command", func() node.Executable { return shellcommand.New() })
	r.RegisterFactory("spreadsheet", func() node.Executable { return spreadsheet.New() })
	r.RegisterFactory("transform", func() node.Executable { return transform.New() })

	return r
}
