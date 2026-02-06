package integration

import (
	"github.com/Develonaut/bento/pkg/node"
	"github.com/Develonaut/bento/pkg/registry"

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

// createRegistry creates a registry with all 10 node types.
func createRegistry() *registry.Registry {
	p := registry.New()

	p.RegisterFactory("edit-fields", func() node.Executable { return editfields.New() })
	p.RegisterFactory("file-system", func() node.Executable { return filesystem.New() })
	p.RegisterFactory("group", func() node.Executable { return group.New() })
	p.RegisterFactory("http-request", func() node.Executable { return httpnode.New() })
	p.RegisterFactory("image", func() node.Executable { return image.New() })
	p.RegisterFactory("loop", func() node.Executable { return loop.New() })
	p.RegisterFactory("parallel", func() node.Executable { return parallel.New() })
	p.RegisterFactory("shell-command", func() node.Executable { return shellcommand.New() })
	p.RegisterFactory("spreadsheet", func() node.Executable { return spreadsheet.New() })
	p.RegisterFactory("transform", func() node.Executable { return transform.New() })

	return p
}
