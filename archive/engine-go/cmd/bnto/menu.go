package main

import (
	"encoding/json"
	"fmt"
	"os"
	"text/tabwriter"

	"github.com/Develonaut/bnto/pkg/menu"
	"github.com/spf13/cobra"
)

var (
	menuJSON bool
	menuSlug string
)

var menuCmd = &cobra.Command{
	Use:   "menu",
	Short: "List predefined bnto recipes",
	Long: `List the catalog of predefined bnto recipes.

Each recipe is a pre-assembled workflow built from generic node types.

Examples:
  bnto menu                            Human-readable table
  bnto menu --json                     Full JSON array
  bnto menu --json --slug compress-images   Single entry JSON`,
	RunE: runMenu,
}

func init() {
	menuCmd.Flags().BoolVar(&menuJSON, "json", false, "Output as JSON")
	menuCmd.Flags().StringVar(&menuSlug, "slug", "", "Filter to a single recipe by slug")
}

func runMenu(_ *cobra.Command, _ []string) error {
	if menuSlug != "" {
		return runMenuSingle(menuSlug)
	}
	return runMenuAll()
}

func runMenuAll() error {
	recipes, err := menu.All()
	if err != nil {
		return fmt.Errorf("loading menu: %w", err)
	}

	if menuJSON {
		return writeJSON(recipes)
	}

	w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
	fmt.Fprintln(w, "SLUG\tNAME\tCATEGORY\tDESCRIPTION")
	for _, r := range recipes {
		desc := r.Description
		if len(desc) > 60 {
			desc = desc[:57] + "..."
		}
		fmt.Fprintf(w, "%s\t%s\t%s\t%s\n", r.Slug, r.Name, r.Category, desc)
	}
	return w.Flush()
}

func runMenuSingle(slug string) error {
	r, err := menu.Get(slug)
	if err != nil {
		return err
	}

	if menuJSON {
		return writeJSON(r)
	}

	fmt.Printf("Slug:        %s\n", r.Slug)
	fmt.Printf("Name:        %s\n", r.Name)
	fmt.Printf("Category:    %s\n", r.Category)
	fmt.Printf("Description: %s\n", r.Description)
	fmt.Printf("Accepts:     %s\n", r.Accept.Label)
	fmt.Printf("Features:    %v\n", r.Features)
	fmt.Printf("SEO Title:   %s\n", r.SEO.Title)
	fmt.Printf("SEO H1:      %s\n", r.SEO.H1)
	return nil
}

func writeJSON(v any) error {
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	return enc.Encode(v)
}
