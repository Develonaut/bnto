// glob.go provides file deletion operations with support for glob patterns.
package filesystem

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// delete deletes a file or files matching a glob pattern.
// Supports both single file paths and glob patterns (e.g., "*.png", "output-*.png").
func (f *FileSystemNode) delete(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	// Check if path contains glob pattern (* or ?)
	if strings.ContainsAny(path, "*?") {
		return f.deleteGlobPattern(path)
	}

	// Single file deletion
	return f.deleteSingleFile(path)
}

// deleteGlobPattern deletes all files matching a glob pattern.
func (f *FileSystemNode) deleteGlobPattern(pattern string) (interface{}, error) {
	// Expand glob pattern
	matches, err := filepath.Glob(pattern)
	if err != nil {
		return nil, fmt.Errorf("failed to expand glob pattern: %w", err)
	}

	// Delete all matching files
	deletedCount := 0
	for _, match := range matches {
		if err := os.Remove(match); err != nil {
			return nil, fmt.Errorf("failed to delete file %s: %w", match, err)
		}
		deletedCount++
	}

	return map[string]interface{}{
		"path":    pattern,
		"deleted": deletedCount,
		"files":   matches,
	}, nil
}

// deleteSingleFile deletes a single file.
func (f *FileSystemNode) deleteSingleFile(path string) (interface{}, error) {
	err := os.Remove(path)
	if err != nil {
		return nil, fmt.Errorf("failed to delete file: %w", err)
	}

	return map[string]interface{}{
		"path":    path,
		"deleted": true,
	}, nil
}

// list lists all files matching a pattern or in a directory.
// Supports glob patterns (e.g., "*.png", "images/*.jpg") or directory paths.
// Returns an array of file paths that can be used with loop nodes.
func (f *FileSystemNode) list(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	// Check if path contains glob pattern (* or ?)
	if strings.ContainsAny(path, "*?") {
		return f.listGlobPattern(path)
	}

	// If it's a directory, list all files in it
	fileInfo, err := os.Stat(path)
	if err != nil {
		return nil, fmt.Errorf("failed to access path: %w", err)
	}

	if fileInfo.IsDir() {
		return f.listDirectory(path)
	}

	// Single file - return it as an array with one element
	return map[string]interface{}{
		"files": []interface{}{path},
		"count": 1,
	}, nil
}

// listGlobPattern lists all files matching a glob pattern.
// Supports brace expansion like *.{jpg,png} which Go's filepath.Glob doesn't.
func (f *FileSystemNode) listGlobPattern(pattern string) (interface{}, error) {
	// Expand brace patterns into multiple glob patterns
	patterns := expandBracePattern(pattern)

	// Collect all matches from all patterns
	matchSet := make(map[string]bool)
	for _, p := range patterns {
		matches, err := filepath.Glob(p)
		if err != nil {
			continue // Skip invalid patterns
		}
		for _, m := range matches {
			matchSet[m] = true
		}
	}

	// Filter out directories, only include files
	files := make([]interface{}, 0, len(matchSet))
	for match := range matchSet {
		fileInfo, err := os.Stat(match)
		if err != nil {
			continue // Skip files we can't stat
		}
		if !fileInfo.IsDir() {
			files = append(files, match)
		}
	}

	return map[string]interface{}{
		"files": files,
		"count": len(files),
	}, nil
}

// expandBracePattern expands patterns like *.{jpg,png} into [*.jpg, *.png].
// If no braces, returns the original pattern.
func expandBracePattern(pattern string) []string {
	// Find brace section
	start := strings.Index(pattern, "{")
	end := strings.Index(pattern, "}")

	if start == -1 || end == -1 || end < start {
		return []string{pattern}
	}

	prefix := pattern[:start]
	suffix := pattern[end+1:]
	options := strings.Split(pattern[start+1:end], ",")

	patterns := make([]string, len(options))
	for i, opt := range options {
		patterns[i] = prefix + opt + suffix
	}

	return patterns
}

// listDirectory lists all files in a directory (non-recursive).
func (f *FileSystemNode) listDirectory(dirPath string) (interface{}, error) {
	entries, err := os.ReadDir(dirPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read directory: %w", err)
	}

	files := make([]interface{}, 0, len(entries))
	for _, entry := range entries {
		if !entry.IsDir() {
			fullPath := filepath.Join(dirPath, entry.Name())
			files = append(files, fullPath)
		}
	}

	return map[string]interface{}{
		"files": files,
		"count": len(files),
	}, nil
}
