package secrets

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// TestManager_SetAndGet verifies basic secret storage and retrieval.
func TestManager_SetAndGet(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Set a secret
	err := mgr.Set("TEST_TOKEN", "secret-value-123")
	require.NoError(t, err, "Should set secret successfully")

	// Get the secret
	value, err := mgr.Get("TEST_TOKEN")
	require.NoError(t, err, "Should retrieve secret successfully")
	assert.Equal(t, "secret-value-123", value, "Retrieved value should match")
}

// TestManager_GetNotFound verifies error handling for missing secrets.
func TestManager_GetNotFound(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Try to get non-existent secret
	_, err := mgr.Get("NONEXISTENT_SECRET")
	assert.Error(t, err, "Should return error for missing secret")
	assert.Contains(t, err.Error(), "not found", "Error should indicate secret not found")
}

// TestManager_Delete verifies secret deletion.
func TestManager_Delete(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Set a secret
	err := mgr.Set("DELETE_ME", "value")
	require.NoError(t, err)

	// Verify it exists
	value, err := mgr.Get("DELETE_ME")
	require.NoError(t, err)
	assert.Equal(t, "value", value)

	// Delete it
	err = mgr.Delete("DELETE_ME")
	require.NoError(t, err, "Should delete successfully")

	// Verify it's gone
	_, err = mgr.Get("DELETE_ME")
	assert.Error(t, err, "Should not find deleted secret")
}

// TestManager_List verifies listing all secret keys.
func TestManager_List(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Set multiple secrets
	secrets := map[string]string{
		"SECRET_ONE":   "value1",
		"SECRET_TWO":   "value2",
		"SECRET_THREE": "value3",
	}

	for key, value := range secrets {
		err := mgr.Set(key, value)
		require.NoError(t, err)
	}

	// List all keys
	keys, err := mgr.List()
	require.NoError(t, err, "Should list secrets successfully")
	assert.Len(t, keys, 3, "Should have 3 secrets")

	// Verify all keys present (order doesn't matter)
	for expectedKey := range secrets {
		assert.Contains(t, keys, expectedKey, "List should include %s", expectedKey)
	}
}

// TestManager_ResolveTemplate_SecretsNamespace verifies {{SECRETS.X}} resolution.
func TestManager_ResolveTemplate_SecretsNamespace(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Set a secret
	err := mgr.Set("FIGMA_TOKEN", "figd_secret123")
	require.NoError(t, err)

	tests := []struct {
		name     string
		template string
		expected string
	}{
		{
			name:     "single secret",
			template: "Token: {{SECRETS.FIGMA_TOKEN}}",
			expected: "Token: figd_secret123",
		},
		{
			name:     "secret in middle",
			template: "Bearer {{SECRETS.FIGMA_TOKEN}} end",
			expected: "Bearer figd_secret123 end",
		},
		{
			name:     "no spaces around braces",
			template: "{{SECRETS.FIGMA_TOKEN}}",
			expected: "figd_secret123",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := mgr.ResolveTemplate(tt.template)
			require.NoError(t, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}

// TestManager_ResolveTemplate_IgnoresPlainVariables verifies {{X}} is NOT resolved.
func TestManager_ResolveTemplate_IgnoresPlainVariables(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	tests := []struct {
		name     string
		template string
		expected string
	}{
		{
			name:     "plain variable unchanged",
			template: "URL: {{.FIGMA_API_URL}}",
			expected: "URL: {{.FIGMA_API_URL}}",
		},
		{
			name:     "only plain variables",
			template: "{{.URL}}/api?key={{.API_KEY}}",
			expected: "{{.URL}}/api?key={{.API_KEY}}",
		},
		{
			name:     "no SECRETS namespace",
			template: "{{PLAIN_VAR}}",
			expected: "{{PLAIN_VAR}}",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Note: We haven't set TOKEN secret, so this tests the "ignore" behavior
			result, _ := mgr.ResolveTemplate(tt.template)
			assert.Equal(t, tt.expected, result)
		})
	}
}

// TestManager_ResolveTemplate_MultipleSecrets verifies multiple {{SECRETS.X}} resolution.
func TestManager_ResolveTemplate_MultipleSecrets(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	// Set multiple secrets
	err := mgr.Set("TOKEN_A", "value-a")
	require.NoError(t, err)
	err = mgr.Set("TOKEN_B", "value-b")
	require.NoError(t, err)

	template := "First: {{SECRETS.TOKEN_A}}, Second: {{SECRETS.TOKEN_B}}"
	result, err := mgr.ResolveTemplate(template)
	require.NoError(t, err)
	assert.Equal(t, "First: value-a, Second: value-b", result)
}

// TestManager_ResolveTemplate_MissingSecret verifies error for missing secret.
func TestManager_ResolveTemplate_MissingSecret(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	template := "Token: {{SECRETS.MISSING_TOKEN}}"
	_, err := mgr.ResolveTemplate(template)
	assert.Error(t, err, "Should error when secret not found")
	assert.Contains(t, err.Error(), "not found", "Error should indicate secret not found")
}

// TestManager_ResolveParams_NestedObjects verifies resolution in nested maps.
func TestManager_ResolveParams_NestedObjects(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	err := mgr.Set("API_TOKEN", "secret123")
	require.NoError(t, err)

	params := map[string]interface{}{
		"url": "https://api.example.com",
		"headers": map[string]interface{}{
			"Authorization": "Bearer {{SECRETS.API_TOKEN}}",
			"Content-Type":  "application/json",
		},
	}

	resolved, err := mgr.ResolveParams(params)
	require.NoError(t, err)

	headers := resolved["headers"].(map[string]interface{})
	assert.Equal(t, "Bearer secret123", headers["Authorization"])
	assert.Equal(t, "application/json", headers["Content-Type"])
}

// TestManager_ResolveParams_Arrays verifies resolution in array values.
func TestManager_ResolveParams_Arrays(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	err := mgr.Set("ARG_SECRET", "secret-arg")
	require.NoError(t, err)

	params := map[string]interface{}{
		"args": []interface{}{
			"--token",
			"{{SECRETS.ARG_SECRET}}",
			"--verbose",
		},
	}

	resolved, err := mgr.ResolveParams(params)
	require.NoError(t, err)

	args := resolved["args"].([]interface{})
	assert.Equal(t, "--token", args[0])
	assert.Equal(t, "secret-arg", args[1])
	assert.Equal(t, "--verbose", args[2])
}

// TestManager_ResolveParams_PreservesNonStrings verifies non-string values unchanged.
func TestManager_ResolveParams_PreservesNonStrings(t *testing.T) {
	mgr := setupTestManager(t)
	defer cleanupTestManager(t, mgr)

	params := map[string]interface{}{
		"timeout": 300,
		"enabled": true,
		"ratio":   0.75,
	}

	resolved, err := mgr.ResolveParams(params)
	require.NoError(t, err)

	assert.Equal(t, 300, resolved["timeout"])
	assert.Equal(t, true, resolved["enabled"])
	assert.Equal(t, 0.75, resolved["ratio"])
}

// setupTestManager creates a manager with isolated test keyring.
func setupTestManager(t *testing.T) *Manager {
	t.Helper()

	// Create unique temp directory for this test's keyring
	tempDir, err := os.MkdirTemp("", "wasabi-test-*")
	require.NoError(t, err, "Should create temp directory")

	// Create manager with isolated keyring
	mgr, err := NewManagerWithConfig(ManagerConfig{
		ServiceName: "bnto-test",
		KeyringDir:  tempDir,
	})
	require.NoError(t, err, "Should create test manager")

	return mgr
}

// cleanupTestManager removes test keyring and secrets.
func cleanupTestManager(t *testing.T, mgr *Manager) {
	t.Helper()

	if mgr == nil {
		return
	}

	// Get keyring directory
	keyringDir := mgr.keyringDir

	// Remove temp directory
	if keyringDir != "" {
		os.RemoveAll(keyringDir)
	}
}
