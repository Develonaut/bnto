package tui

import (
	"github.com/charmbracelet/huh"
)

// PromptForVariables displays a Huh form to collect variable values.
// Returns a map of variable name -> user-provided value.
func PromptForVariables(vars []Variable) (map[string]string, error) {
	if len(vars) == 0 {
		return make(map[string]string), nil
	}

	// Create value holders for each variable
	valueHolders := make(map[string]*string)
	for _, v := range vars {
		holder := v.DefaultValue
		valueHolders[v.Name] = &holder
	}

	// Build form fields
	fields := make([]huh.Field, 0, len(vars))
	for _, v := range vars {
		fields = append(fields, buildField(v, valueHolders[v.Name]))
	}

	// Create form
	form := huh.NewForm(
		huh.NewGroup(fields...),
	).WithTheme(huh.ThemeCharm())

	// Run form
	if err := form.Run(); err != nil {
		return nil, err
	}

	// Convert to map
	values := make(map[string]string)
	for name, holder := range valueHolders {
		values[name] = *holder
	}

	return values, nil
}
