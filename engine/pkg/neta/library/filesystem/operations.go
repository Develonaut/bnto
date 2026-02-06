// operations.go provides basic file system operations for reading, writing,
// creating directories, and checking file existence.
package filesystem

import (
	"fmt"
	"os"
	"path/filepath"
)

// read reads the contents of a file.
func (f *FileSystemNeta) read(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read file: %w", err)
	}

	return map[string]interface{}{
		"content": string(data),
		"path":    path,
	}, nil
}

// write writes content to a file.
func (f *FileSystemNeta) write(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	content, ok := params["content"].(string)
	if !ok {
		return nil, fmt.Errorf("content parameter is required and must be a string")
	}

	// Check .bentoignore in the target directory
	dir := filepath.Dir(path)
	bentoIgnore, err := LoadBentoIgnore(dir)
	if err != nil {
		// If we can't load .bentoignore, log warning but continue
		// (don't want to break existing workflows)
		fmt.Fprintf(os.Stderr, "Warning: failed to load .bentoignore: %v\n", err)
	} else if bentoIgnore.ShouldIgnore(path) {
		return nil, fmt.Errorf("file %s is protected by .bentoignore and cannot be overwritten", path)
	}

	err = os.WriteFile(path, []byte(content), 0644)
	if err != nil {
		return nil, fmt.Errorf("failed to write file: %w", err)
	}

	return map[string]interface{}{
		"path":    path,
		"written": true,
	}, nil
}

// mkdir creates a directory.
func (f *FileSystemNeta) mkdir(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	err := os.MkdirAll(path, 0755)
	if err != nil {
		return nil, fmt.Errorf("failed to create directory: %w", err)
	}

	return map[string]interface{}{
		"path":    path,
		"created": true,
	}, nil
}

// exists checks if a file or directory exists.
func (f *FileSystemNeta) exists(params map[string]interface{}) (interface{}, error) {
	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required and must be a string")
	}

	_, err := os.Stat(path)
	exists := !os.IsNotExist(err)

	return map[string]interface{}{
		"path":   path,
		"exists": exists,
	}, nil
}
