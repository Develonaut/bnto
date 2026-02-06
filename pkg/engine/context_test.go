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
			template: "{{basename .PRODUCT_PATH}}",
			envVars: map[string]string{
				"PRODUCT_PATH": "/Users/Ryan/Library/CloudStorage/GoogleDrive-ryanmmchenry@gmail.com/My Drive/Heavy Handed/Products/Miniatures/Bite The Bullet/Combat Dog (Supplies)",
			},
			want: "Combat Dog (Supplies)",
		},
		{
			name:     "basename with simple path",
			template: "{{basename .PRODUCT_PATH}}",
			envVars: map[string]string{
				"PRODUCT_PATH": "/path/to/product",
			},
			want: "product",
		},
		{
			name:     "basename in filename",
			template: "{{basename .PRODUCT_PATH}}_001.png",
			envVars: map[string]string{
				"PRODUCT_PATH": "/path/to/Combat Engineer",
			},
			want: "Combat Engineer_001.png",
		},
		{
			name:     "basename with trailing slash",
			template: "{{basename .PRODUCT_PATH}}",
			envVars: map[string]string{
				"PRODUCT_PATH": "/path/to/product/",
			},
			want: "product",
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
	ctx.nodeData["PRODUCT_PATH"] = "/Users/Ryan/Products/Combat Dog (Supplies)"

	template := "--filename-prefix {{basename .PRODUCT_PATH}}"
	result := ctx.resolveString(template)
	want := "--filename-prefix Combat Dog (Supplies)"

	if result != want {
		t.Errorf("resolveString(%q) = %q, want %q", template, result, want)
	}
}
