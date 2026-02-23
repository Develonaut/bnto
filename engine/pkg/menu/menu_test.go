package menu

import (
	"encoding/json"
	"testing"
)

func TestAllReturnsRecipes(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	if len(recipes) == 0 {
		t.Fatal("All() returned zero recipes")
	}
}

func TestAllSortedBySlag(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	for i := 1; i < len(recipes); i++ {
		if recipes[i].Slug < recipes[i-1].Slug {
			t.Errorf("recipes not sorted: %q comes after %q", recipes[i].Slug, recipes[i-1].Slug)
		}
	}
}

func TestNoDuplicateSlugs(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	seen := make(map[string]bool)
	for _, r := range recipes {
		if seen[r.Slug] {
			t.Errorf("duplicate slug: %q", r.Slug)
		}
		seen[r.Slug] = true
	}
}

func TestRecipeFieldsPopulated(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	for _, r := range recipes {
		t.Run(r.Slug, func(t *testing.T) {
			if r.Name == "" {
				t.Error("empty name")
			}
			if r.Description == "" {
				t.Error("empty description")
			}
			if r.Category == "" {
				t.Error("empty category")
			}
			if r.Accept.Label == "" {
				t.Error("empty accept.label")
			}
			if len(r.Accept.MIMETypes) == 0 {
				t.Error("empty accept.mimeTypes")
			}
			if len(r.Features) == 0 {
				t.Error("empty features")
			}
			if r.SEO.Title == "" {
				t.Error("empty seo.title")
			}
			if r.SEO.H1 == "" {
				t.Error("empty seo.h1")
			}
			if len(r.Definition) == 0 {
				t.Error("empty definition")
			}
		})
	}
}

func TestDefinitionsAreValidJSON(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	for _, r := range recipes {
		t.Run(r.Slug, func(t *testing.T) {
			var def map[string]any
			if err := json.Unmarshal(r.Definition, &def); err != nil {
				t.Errorf("definition is not valid JSON: %v", err)
			}
			if def["id"] == nil {
				t.Error("definition missing 'id' field")
			}
			if def["type"] == nil {
				t.Error("definition missing 'type' field")
			}
		})
	}
}

func TestGetKnownSlug(t *testing.T) {
	r, err := Get("compress-images")
	if err != nil {
		t.Fatalf("Get() error: %v", err)
	}
	if r.Slug != "compress-images" {
		t.Errorf("expected slug compress-images, got %s", r.Slug)
	}
}

func TestGetUnknownSlug(t *testing.T) {
	_, err := Get("nonexistent-slug")
	if err == nil {
		t.Fatal("expected error for unknown slug")
	}
}

func TestSlugsMatchAll(t *testing.T) {
	slugs, err := Slugs()
	if err != nil {
		t.Fatalf("Slugs() error: %v", err)
	}
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	if len(slugs) != len(recipes) {
		t.Fatalf("Slugs() count %d != All() count %d", len(slugs), len(recipes))
	}
	for i, s := range slugs {
		if s != recipes[i].Slug {
			t.Errorf("slug mismatch at %d: %q vs %q", i, s, recipes[i].Slug)
		}
	}
}

func TestExpectedRecipeCount(t *testing.T) {
	recipes, err := All()
	if err != nil {
		t.Fatalf("All() error: %v", err)
	}
	if len(recipes) != 6 {
		t.Errorf("expected 6 recipes, got %d", len(recipes))
	}
}
