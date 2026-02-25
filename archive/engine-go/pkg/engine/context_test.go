package engine

import (
	"testing"
)

// TestResolveStringWithBasename tests the basename template function.
func TestResolveStringWithBasename(t *testing.T) {
	tests := []struct {
		name     string
		template string
		envVars  map[string]string
		want     string
	}{
		{
			name:     "basename with full path",
			template: "{{basename .DATA_PATH}}",
			envVars: map[string]string{
				"DATA_PATH": "/tmp/test-data/items/alpha-item",
			},
			want: "alpha-item",
		},
		{
			name:     "basename with simple path",
			template: "{{basename .DATA_PATH}}",
			envVars: map[string]string{
				"DATA_PATH": "/path/to/item",
			},
			want: "item",
		},
		{
			name:     "basename in filename",
			template: "{{basename .DATA_PATH}}_001.png",
			envVars: map[string]string{
				"DATA_PATH": "/path/to/Test Item A",
			},
			want: "Test Item A_001.png",
		},
		{
			name:     "basename with trailing slash",
			template: "{{basename .DATA_PATH}}",
			envVars: map[string]string{
				"DATA_PATH": "/path/to/item/",
			},
			want: "item",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ctx := newExecutionContext()

			// Add environment variables to context
			for k, v := range tt.envVars {
				ctx.nodeData[k] = v
			}

			// Resolve the template
			result := ctx.resolveString(tt.template)

			// Check result
			if result != tt.want {
				t.Errorf("resolveString(%q) = %q, want %q", tt.template, result, tt.want)
			}
		})
	}
}

// TestTemplateWithBasenameInArgs tests basename in command args.
func TestTemplateWithBasenameInArgs(t *testing.T) {
	ctx := newExecutionContext()
	ctx.nodeData["DATA_PATH"] = "/tmp/test-data/items/Test Item A"

	template := "--filename-prefix {{basename .DATA_PATH}}"
	result := ctx.resolveString(template)
	want := "--filename-prefix Test Item A"

	if result != want {
		t.Errorf("resolveString(%q) = %q, want %q", template, result, want)
	}
}
