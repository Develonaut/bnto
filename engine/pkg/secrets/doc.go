// Package wasabi provides secure secrets management for Bento workflows.
//
// # Overview
//
// Wasabi handles the "spicy stuff" 🟢 - API tokens, credentials, and other
// sensitive data that shouldn't be hardcoded in bento definitions or committed
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
// # Usage in Bentos
//
// Secrets are accessed using the {{SECRETS.X}} namespace in bento templates:
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
// Secrets are managed via the bento CLI:
//
//	bento wasabi set FIGMA_TOKEN figd_your_token_here
//	bento wasabi get FIGMA_TOKEN
//	bento wasabi list
//	bento wasabi delete FIGMA_TOKEN
//
// # Testing
//
// For testing, wasabi supports file-based storage to avoid keychain prompts:
//
//	mgr, _ := NewManagerWithConfig(ManagerConfig{
//	    ServiceName: "bento-test",
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
// in bento definitions or version control.
package secrets
