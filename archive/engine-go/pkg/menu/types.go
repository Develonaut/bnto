// Package menu provides the canonical catalog of predefined bnto recipes.
//
// Each recipe is a pre-assembled workflow definition (a "bento box") built
// from generic node types. The engine owns these definitions; consumers
// (web, desktop, CLI) read them via this package or the `bnto menu` command.
package menu

import "encoding/json"

// Recipe is a predefined bnto — metadata, accepted file types, SEO data,
// and the full workflow definition ready for execution.
type Recipe struct {
	Slug        string          `json:"slug"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	Category    string          `json:"category"`
	Accept      AcceptSpec      `json:"accept"`
	Features    []string        `json:"features"`
	SEO         SEOSpec         `json:"seo"`
	Definition  json.RawMessage `json:"definition"`
}

// AcceptSpec describes the file types a recipe accepts as input.
type AcceptSpec struct {
	MIMETypes  []string `json:"mimeTypes"`
	Extensions []string `json:"extensions"`
	Label      string   `json:"label"`
	MIMEPrefix string   `json:"mimePrefix,omitempty"`
}

// SEOSpec holds search-engine metadata for the recipe's public page.
type SEOSpec struct {
	Title string `json:"title"`
	H1    string `json:"h1"`
}
