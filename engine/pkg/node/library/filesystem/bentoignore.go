// bentoignore.go provides .bentoignore file support to prevent bentos from
// overwriting files that should be manually managed.
package filesystem

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
)

// BentoIgnore handles .bentoignore pattern matching
type BentoIgnore struct {
	patterns []string
}

// LoadBentoIgnore reads .bentoignore file from the given directory
func LoadBentoIgnore(dir string) (*BentoIgnore, error) {
	ignorePath := filepath.Join(dir, ".bentoignore")

	// If .bentoignore doesn't exist, return empty ignore list
	if _, err := os.Stat(ignorePath); os.IsNotExist(err) {
		return &BentoIgnore{patterns: []string{}}, nil
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

	return &BentoIgnore{patterns: patterns}, nil
}

// ShouldIgnore checks if a file path matches any ignore patterns
func (bi *BentoIgnore) ShouldIgnore(filePath string) bool {
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
