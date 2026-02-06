// Package wasabi provides secure secrets management for Bento workflows.
//
// Wasabi handles the "spicy stuff" - API tokens, credentials, and other sensitive
// data that shouldn't be hardcoded in bento definitions. It uses OS-native keychain
// storage (macOS Keychain, Windows Credential Manager, Linux Secret Service) to
// keep secrets secure.
//
// Secrets are accessed in bento templates using the {{SECRETS.X}} namespace,
// which is strictly separated from regular {{.X}} environment variables.
//
// Example:
//
//	// In bento definition:
//	{
//	  "type": "http-request",
//	  "parameters": {
//	    "url": "{{.FIGMA_API_URL}}/files/abc123",
//	    "headers": {
//	      "Authorization": "Bearer {{SECRETS.FIGMA_TOKEN}}"
//	    }
//	  }
//	}
//
// This ensures clear separation between configuration (URLs, paths) and
// sensitive data (tokens, passwords).
package secrets

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/99designs/keyring"
)

// secretsPattern matches {{SECRETS.KEY_NAME}} in templates.
// Compiled once at package initialization for performance.
var secretsPattern = regexp.MustCompile(`\{\{SECRETS\.([A-Z_][A-Z0-9_]*)\}\}`)

// Manager handles secrets storage and retrieval using OS-native keychain.
type Manager struct {
	keyring    keyring.Keyring
	keyringDir string // For cleanup in tests
}

// ManagerConfig configures the secrets manager.
type ManagerConfig struct {
	ServiceName string // Keyring service name (e.g., "bento", "bento-test")
	KeyringDir  string // Directory for file backend (empty = use OS keychain)
}

// NewManager creates a new secrets manager using OS-native keychain.
func NewManager() (*Manager, error) {
	return NewManagerWithConfig(ManagerConfig{
		ServiceName: "bento",
	})
}

// NewManagerWithConfig creates a secrets manager with custom configuration.
// This is primarily used for testing with isolated file-based storage.
func NewManagerWithConfig(cfg ManagerConfig) (*Manager, error) {
	var kr keyring.Keyring
	var err error

	if cfg.KeyringDir != "" {
		// File backend for testing (isolated storage)
		kr, err = keyring.Open(keyring.Config{
			ServiceName:              cfg.ServiceName,
			FileDir:                  cfg.KeyringDir,
			FilePasswordFunc:         filePassword,
			AllowedBackends:          []keyring.BackendType{keyring.FileBackend},
			KeychainTrustApplication: true,
		})
	} else {
		// OS keychain for production use
		kr, err = keyring.Open(keyring.Config{
			ServiceName:              cfg.ServiceName,
			KeychainTrustApplication: true,
		})
	}

	if err != nil {
		return nil, fmt.Errorf("failed to open keyring: %w", err)
	}

	return &Manager{
		keyring:    kr,
		keyringDir: cfg.KeyringDir,
	}, nil
}

// filePassword returns a consistent password for file backend (testing only).
func filePassword(prompt string) (string, error) {
	return "test-password", nil
}

// Set stores a secret in the keychain.
func (m *Manager) Set(key, value string) error {
	if key == "" {
		return fmt.Errorf("secret key cannot be empty")
	}

	item := keyring.Item{
		Key:  key,
		Data: []byte(value),
	}

	if err := m.keyring.Set(item); err != nil {
		return fmt.Errorf("failed to set secret %s: %w", key, err)
	}

	return nil
}

// Get retrieves a secret from the keychain.
func (m *Manager) Get(key string) (string, error) {
	if key == "" {
		return "", fmt.Errorf("secret key cannot be empty")
	}

	item, err := m.keyring.Get(key)
	if err != nil {
		if err == keyring.ErrKeyNotFound {
			return "", fmt.Errorf("secret %s not found", key)
		}
		return "", fmt.Errorf("failed to get secret %s: %w", key, err)
	}

	return string(item.Data), nil
}

// Delete removes a secret from the keychain.
func (m *Manager) Delete(key string) error {
	if key == "" {
		return fmt.Errorf("secret key cannot be empty")
	}

	if err := m.keyring.Remove(key); err != nil {
		if err == keyring.ErrKeyNotFound {
			return fmt.Errorf("secret %s not found", key)
		}
		return fmt.Errorf("failed to delete secret %s: %w", key, err)
	}

	return nil
}

// List returns all secret keys stored in the keychain.
func (m *Manager) List() ([]string, error) {
	keys, err := m.keyring.Keys()
	if err != nil {
		return nil, fmt.Errorf("failed to list secrets: %w", err)
	}

	return keys, nil
}

// ResolveTemplate resolves {{SECRETS.X}} placeholders in a template string.
// Plain {{X}} or {{.X}} variables are left untouched.
func (m *Manager) ResolveTemplate(template string) (string, error) {
	result := template
	matches := secretsPattern.FindAllStringSubmatch(template, -1)

	for _, match := range matches {
		placeholder := match[0] // {{SECRETS.KEY_NAME}}
		secretKey := match[1]   // KEY_NAME

		// Get secret value
		value, err := m.Get(secretKey)
		if err != nil {
			return "", fmt.Errorf("failed to resolve %s: %w", placeholder, err)
		}

		// Replace placeholder with value
		result = strings.ReplaceAll(result, placeholder, value)
	}

	return result, nil
}

// ResolveParams recursively resolves {{SECRETS.X}} placeholders in a params map.
// Handles nested maps, arrays, and preserves non-string values.
func (m *Manager) ResolveParams(params map[string]interface{}) (map[string]interface{}, error) {
	resolved := make(map[string]interface{})

	for key, value := range params {
		resolvedValue, err := m.resolveValue(value)
		if err != nil {
			return nil, err
		}
		resolved[key] = resolvedValue
	}

	return resolved, nil
}

// resolveValue recursively resolves a single value (handles strings, maps, arrays).
func (m *Manager) resolveValue(value interface{}) (interface{}, error) {
	switch v := value.(type) {
	case string:
		// Resolve template in string
		return m.ResolveTemplate(v)

	case map[string]interface{}:
		// Recursively resolve nested map
		resolved := make(map[string]interface{})
		for k, val := range v {
			resolvedVal, err := m.resolveValue(val)
			if err != nil {
				return nil, err
			}
			resolved[k] = resolvedVal
		}
		return resolved, nil

	case []interface{}:
		// Recursively resolve array
		resolved := make([]interface{}, len(v))
		for i, val := range v {
			resolvedVal, err := m.resolveValue(val)
			if err != nil {
				return nil, err
			}
			resolved[i] = resolvedVal
		}
		return resolved, nil

	default:
		// Non-string values (int, bool, float) pass through unchanged
		return value, nil
	}
}
