package tui

import (
	"github.com/Develonaut/bnto/pkg/paths"
)

// ResolvePath expands special markers and environment variables in a path.
// This is a convenience wrapper around paths.ResolvePath.
func ResolvePath(path string) (string, error) {
	return paths.ResolvePath(path)
}

// CompressPath converts absolute paths to use special markers for portability.
// This is a convenience wrapper around paths.CompressPath.
func CompressPath(path string) string {
	return paths.CompressPath(path)
}

// ResolvePathsInMap resolves all paths in a string map (useful for variables).
// This is a convenience wrapper around paths.ResolvePathsInMap.
func ResolvePathsInMap(m map[string]string) (map[string]string, error) {
	return paths.ResolvePathsInMap(m)
}
