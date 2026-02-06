package tui

import (
	"github.com/Develonaut/bento/pkg/node"
	editfields "github.com/Develonaut/bento/pkg/node/library/editfields"
	filesystem "github.com/Develonaut/bento/pkg/node/library/filesystem"
	group "github.com/Develonaut/bento/pkg/node/library/group"
	httpneta "github.com/Develonaut/bento/pkg/node/library/http"
	image "github.com/Develonaut/bento/pkg/node/library/image"
	loop "github.com/Develonaut/bento/pkg/node/library/loop"
	parallel "github.com/Develonaut/bento/pkg/node/library/parallel"
	shellcommand "github.com/Develonaut/bento/pkg/node/library/shellcommand"
	spreadsheet "github.com/Develonaut/bento/pkg/node/library/spreadsheet"
	transform "github.com/Develonaut/bento/pkg/node/library/transform"
	"github.com/Develonaut/bento/pkg/registry"
)

// createTUIPantry creates and populates the registry with all node types
func createTUIPantry() *registry.Registry {
	p := registry.New()

	// Register all node types
	p.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })
	p.RegisterFactory("file-system", func() node.Executable { return filesystem.New() })
	p.RegisterFactory("group", func() node.Executable { return group.New() })
	p.RegisterFactory("http-request", func() node.Executable { return httpneta.New() })
	p.RegisterFactory("image", func() node.Executable { return image.New() })
	p.RegisterFactory("loop", func() node.Executable { return loop.New() })
	p.RegisterFactory("parallel", func() node.Executable { return parallel.New() })
	p.RegisterFactory("shell-command", func() node.Executable { return shellcommand.New() })
	p.RegisterFactory("spreadsheet", func() node.Executable { return spreadsheet.New() })
	p.RegisterFactory("transform", func() node.Executable { return transform.New() })

	return p
}
