// Package wasabi provides secure secrets management for Bnto workflows.
//
// # Overview
//
// Wasabi handles the "spicy stuff" 🟢 - API tokens, credentials, and other
// sensitive data that shouldn't be hardcoded in bnto definitions or committed
// to version control.
//
// # Storage
//
// Secrets are stored using OS-native keychain systems:
//   - macOS: Keychain Access
//   - Windows: Credential Manager
//   - Linux: Secret Service API (GNOME Keyring, KWallet)
//
// This ensures secrets are encrypted at rest and only accessible to the user.
//
// # Usage in Bntos
//
// Secrets are accessed using the {{SECRETS.X}} namespace in bnto templates:
//
//	{
//	  "id": "fetch-designs",
//	  "type": "http-request",
//	  "parameters": {
//	    "url": "{{.FIGMA_API_URL}}/files/abc123",
//	    "headers": {
//	      "Authorization": "Bearer {{SECRETS.FIGMA_TOKEN}}"
//	    }
//	  }
//	}
//
// # Namespace Separation
//
// Wasabi enforces strict separation between configuration and secrets:
//
//   - {{SECRETS.X}} - Resolved by wasabi from keychain (sensitive data)
//   - {{.X}} - Resolved by engine from environment/context (configuration)
//
// This separation prevents accidental exposure of secrets and makes security
// auditing straightforward.
//
// # CLI Commands
//
// Secrets are managed via the bnto CLI:
//
//	bnto wasabi set FIGMA_TOKEN figd_your_token_here
//	bnto wasabi get FIGMA_TOKEN
//	bnto wasabi list
//	bnto wasabi delete FIGMA_TOKEN
//
// # Testing
//
// For testing, wasabi supports file-based storage to avoid keychain prompts:
//
//	mgr, _ := NewManagerWithConfig(ManagerConfig{
//	    ServiceName: "bnto-test",
//	    KeyringDir:  "/tmp/test-keyring",
//	})
//
// This creates isolated test storage that doesn't interfere with production secrets.
//
// # Security Considerations
//
//   - Secrets are never logged or printed to stdout
//   - Secret resolution errors fail loudly (no silent fallbacks)
//   - Each secret must be explicitly set before use
//   - No default or placeholder values
//
// # Integration with Engine
//
// The engine executor automatically resolves {{SECRETS.X}} placeholders before
// executing nodes, ensuring secrets are available when needed but never exposed
// in bnto definitions or version control.
package secrets
