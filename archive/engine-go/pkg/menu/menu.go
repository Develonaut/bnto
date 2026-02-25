package menu

import (
	"embed"
	"encoding/json"
	"fmt"
	"path/filepath"
	"sort"
	"strings"
)

//go:embed recipes/*.json
var dataFS embed.FS

// All returns every recipe sorted by slug.
func All() ([]Recipe, error) {
	entries, err := dataFS.ReadDir("recipes")
	if err != nil {
		return nil, fmt.Errorf("reading recipes dir: %w", err)
	}

	recipes := make([]Recipe, 0, len(entries))
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".json") {
			continue
		}
		r, err := parseFile(filepath.Join("recipes", entry.Name()))
		if err != nil {
			return nil, err
		}
		recipes = append(recipes, r)
	}

	sort.Slice(recipes, func(i, j int) bool {
		return recipes[i].Slug < recipes[j].Slug
	})
	return recipes, nil
}

// Get returns a single recipe by slug, or an error if not found.
func Get(slug string) (*Recipe, error) {
	recipes, err := All()
	if err != nil {
		return nil, err
	}
	for i := range recipes {
		if recipes[i].Slug == slug {
			return &recipes[i], nil
		}
	}
	return nil, fmt.Errorf("recipe %q not found", slug)
}

// Slugs returns the slug of every recipe, sorted alphabetically.
func Slugs() ([]string, error) {
	recipes, err := All()
	if err != nil {
		return nil, err
	}
	slugs := make([]string, len(recipes))
	for i, r := range recipes {
		slugs[i] = r.Slug
	}
	return slugs, nil
}

func parseFile(path string) (Recipe, error) {
	data, err := dataFS.ReadFile(path)
	if err != nil {
		return Recipe{}, fmt.Errorf("reading %s: %w", path, err)
	}
	var r Recipe
	if err := json.Unmarshal(data, &r); err != nil {
		return Recipe{}, fmt.Errorf("parsing %s: %w", path, err)
	}
	return r, nil
}
