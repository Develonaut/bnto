// transfer.go provides file transfer operations for copying and moving files.
package filesystem

import (
	"fmt"
	"io"
	"os"
)

// validateTransferParams validates source and dest parameters for file operations.
func validateTransferParams(params map[string]interface{}) (source, dest string, err error) {
	source, ok := params["source"].(string)
	if !ok {
		return "", "", fmt.Errorf("source parameter is required and must be a string")
	}

	dest, ok = params["dest"].(string)
	if !ok {
		return "", "", fmt.Errorf("dest parameter is required and must be a string")
	}

	return source, dest, nil
}

// copy copies a file from source to destination.
func (f *FileSystemNeta) copy(params map[string]interface{}) (interface{}, error) {
	source, dest, err := validateTransferParams(params)
	if err != nil {
		return nil, err
	}

	srcFile, err := os.Open(source)
	if err != nil {
		return nil, fmt.Errorf("failed to open source file: %w", err)
	}
	defer srcFile.Close()

	destFile, err := os.Create(dest)
	if err != nil {
		return nil, fmt.Errorf("failed to create destination file: %w", err)
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, srcFile)
	if err != nil {
		return nil, fmt.Errorf("failed to copy file: %w", err)
	}

	return map[string]interface{}{
		"source": source,
		"dest":   dest,
		"copied": true,
	}, nil
}

// move moves/renames a file.
func (f *FileSystemNeta) move(params map[string]interface{}) (interface{}, error) {
	source, dest, err := validateTransferParams(params)
	if err != nil {
		return nil, err
	}

	err = os.Rename(source, dest)
	if err != nil {
		return nil, fmt.Errorf("failed to move file: %w", err)
	}

	return map[string]interface{}{
		"source": source,
		"dest":   dest,
		"moved":  true,
	}, nil
}
