// bntoignore.go provides .bntoignore file support for image exports
package image

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
)

// bntoIgnore handles .bntoignore pattern matching
type bntoIgnore struct {
	patterns []string
}

// loadBntoIgnore reads .bntoignore file from the given directory
func loadBntoIgnore(dir string) (*bntoIgnore, error) {
	ignorePath := filepath.Join(dir, ".bntoignore")

	// If .bntoignore doesn't exist, return empty ignore list
	if _, err := os.Stat(ignorePath); os.IsNotExist(err) {
		return &bntoIgnore{patterns: []string{}}, nil
	}

	file, err := os.Open(ignorePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	patterns := []string{}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())

		// Skip empty lines and comments
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}

		patterns = append(patterns, line)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return &bntoIgnore{patterns: patterns}, nil
}

// shouldIgnore checks if a file path matches any ignore patterns
func (bi *bntoIgnore) shouldIgnore(filePath string) bool {
	// Normalize the path
	cleanPath := filepath.Clean(filePath)

	for _, pattern := range bi.patterns {
		// Simple glob pattern matching
		if matched, _ := filepath.Match(pattern, filepath.Base(cleanPath)); matched {
			return true
		}

		// Also check full path match for patterns with /
		if strings.Contains(pattern, "/") {
			if matched, _ := filepath.Match(pattern, cleanPath); matched {
				return true
			}
		}

		// Check if any parent directory matches the pattern
		parts := strings.Split(cleanPath, string(filepath.Separator))
		for _, part := range parts {
			if matched, _ := filepath.Match(pattern, part); matched {
				return true
			}
		}
	}

	return false
}
