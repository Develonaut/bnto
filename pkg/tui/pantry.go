package tui

import (
	"github.com/Develonaut/bento/pkg/neta"
	editfields "github.com/Develonaut/bento/pkg/neta/library/editfields"
	filesystem "github.com/Develonaut/bento/pkg/neta/library/filesystem"
	group "github.com/Develonaut/bento/pkg/neta/library/group"
	httpneta "github.com/Develonaut/bento/pkg/neta/library/http"
	image "github.com/Develonaut/bento/pkg/neta/library/image"
	loop "github.com/Develonaut/bento/pkg/neta/library/loop"
	parallel "github.com/Develonaut/bento/pkg/neta/library/parallel"
	shellcommand "github.com/Develonaut/bento/pkg/neta/library/shellcommand"
	spreadsheet "github.com/Develonaut/bento/pkg/neta/library/spreadsheet"
	transform "github.com/Develonaut/bento/pkg/neta/library/transform"
	"github.com/Develonaut/bento/pkg/registry"
)

// createTUIPantry creates and populates the registry with all neta types
func createTUIPantry() *registry.Registry {
	p := registry.New()

	// Register all neta types
	p.RegisterFactory("edit-fields", func() neta.Executable { return editfields.New() })
	p.RegisterFactory("file-system", func() neta.Executable { return filesystem.New() })
	p.RegisterFactory("group", func() neta.Executable { return group.New() })
	p.RegisterFactory("http-request", func() neta.Executable { return httpneta.New() })
	p.RegisterFactory("image", func() neta.Executable { return image.New() })
	p.RegisterFactory("loop", func() neta.Executable { return loop.New() })
	p.RegisterFactory("parallel", func() neta.Executable { return parallel.New() })
	p.RegisterFactory("shell-command", func() neta.Executable { return shellcommand.New() })
	p.RegisterFactory("spreadsheet", func() neta.Executable { return spreadsheet.New() })
	p.RegisterFactory("transform", func() neta.Executable { return transform.New() })

	return p
}
