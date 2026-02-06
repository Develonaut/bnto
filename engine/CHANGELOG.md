# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Phase 8.7: Product Automation Master Bento** - Complete end-to-end workflow integration
  - Master bento combining all Phase 8 components (CSV → Folders → Figma API → Blender → WebP)
  - Integration test validating complete product automation workflow
  - Template resolution inside forEach loop contexts (fixed context passing)
  - Performance: 17.7s for 3 products (5.9s per product) with mocks
  - Demonstrates: CSV reading, loops, HTTP requests, shell commands, parallel optimization, file operations
- Phase 8.6: Image Optimization Bento with parallel WebP conversion
  - Integration tests for parallel image conversion (8 PNGs to WebP)
  - Glob pattern support for filesystem delete operation (`render-*.png`)
  - File-level documentation for filesystem package modules
  - Performance validation tests (~100ms for 8 images with 4-way concurrency)
  - `validateTransferParams()` helper for DRY parameter validation
  - Complete test coverage (9 filesystem tests + 3 integration tests)
- **Wasabi Package**: Secure secrets management with OS-native keychain storage
  - `bento wasabi set/get/list/delete` CLI commands
  - `{{SECRETS.X}}` template namespace for secrets (separate from `{{.X}}` config)
  - OS-native keychain integration (macOS Keychain, Windows Credential Manager, Linux Secret Service)
  - Automatic secret resolution in itamae execution context
  - File-based backend for isolated test storage
  - Comprehensive test suite with 11 passing tests
- Phase 8.5: Mock Render Bento with streaming output validation
  - Real-time streaming output from long-running shell commands
  - `_onOutput` callback injection in itamae executor
  - `OnStream` logger callback for incremental output display
  - Mock Blender script for fast integration testing
  - Streaming progress validation tests (line-by-line output ordering)
- Phase 8.4: API Fetch Bento with HTTP request chaining and file downloads
- `saveToFile` parameter for http-request neta (download responses to files)
- `queryParams` parameter for http-request neta (URL query string support)
- Environment variable loading in execution context (templates can access env vars)
- Mock Figma server with actual PNG image serving
- Integration tests for API workflows with race detector validation
- Phase 8.3: Folder Setup Bento integration test with forEach loop
- Loop neta nested node execution support
- Enhanced template resolution for arrays and maps (not just strings)
- Type conversion for CSV output arrays ([]map[string]interface{} → []interface{})
- Integration test for idempotent folder creation
- Phase 8.2: First integration test with CSV reader bento example
- Integration test framework with helper utilities
- 100% CLI command test coverage (15 tests)
- Comprehensive DEVELOPMENT.md guide for Go development
- Version command shorthand (`bento v`)
- OS-specific installation instructions for glow
- Test fixtures and mock servers (Figma API, Blender)
- Node.js to Go translation guide
- Dependency management philosophy documentation

### Changed
- Refactored filesystem package into focused modules (operations.go, transfer.go, glob.go)
  - Split 277-line filesystem.go into 4 files (all <100 lines)
  - Added file-level documentation to all filesystem modules
  - Extracted `validateTransferParams()` helper for code reuse
  - Reduced `copy()` function from 37 to 29 lines (Bento Box compliance)
  - Removed redundant comments from transfer operations
- Enhanced itamae context resolution to support secrets-first template resolution
- Improved error visibility for missing secrets (stderr warnings instead of silent failures)
- Optimized regex compilation in wasabi (compiled once at package level)
- Removed Close() method from wasabi.Manager (YAGNI principle)
- Updated examples to use `{{SECRETS.X}}` syntax for API tokens
- Fixed race condition in mock Figma server using NewUnstartedServer pattern
- Improved error handling in addQueryParams (now returns errors instead of silent failures)
- Enhanced URL parameter handling with type validation
- Renamed `url` variable to `urlStr` to avoid package shadowing
- Fixed path concatenation in tests to use filepath.Join (cross-platform compatibility)
- Refactored loop.go functions to comply with Bento Box Principle (<30 lines)
- Enhanced executionContext to support exact template resolution ({{index . "key"}} returns actual value)
- Added parameter template resolution in executor for nested nodes
- Implemented times loop body execution (was stub)
- Improved error messages for missing glow installation
- Renamed `helpers.go` to `test_utilities.go` for Bento Box compliance
- Enhanced package documentation for integration tests

### Fixed
- Race condition in parallel execution (removed concurrent map write to shared execCtx)
  - Removed unsafe `execCtx.set()` call from parallel goroutines (pkg/itamae/parallel.go:84)
  - Outputs now only written to mutex-protected `result.NodeOutputs`
  - Validated with race detector (`go test -race`)
- Enhanced filesystem delete operation to support glob patterns
  - Added wildcard detection using `strings.ContainsAny(path, "*?")`
  - Implemented `deleteGlobPattern()` using `filepath.Glob()`
  - Supports patterns like `"render-*.png"` and `"products/*/output.txt"`
- Template resolution in loop iteration context ({{.item.field}} now works in nested nodes)
- Loop body execution for forEach mode (nested nodes now execute properly)
- Shallow copy documentation added to executionContext
- Linting errors in progress tests
- Error handling in test utilities

## [0.1.0] - 2025-10-19

### Added

#### Phase 7: CLI Implementation
- Complete bento CLI with playful sushi-themed commands
- `savor` command - Execute bento workflows
- `sample` command - Validate bentos without execution
- `menu` command - List available bentos
- `box` command - Create new bento templates
- `recipe` command - View documentation with glow
- `version` command - Display version information
- Charm CLI integration with beautiful output
- Progress display with Bubbletea TUI

#### Phase 7: Visual Feedback (Miso Package)
- Bubbletea-based progress display
- Daemon-combo pattern for running Bubbletea in background
- Multi-step progress tracking
- Spinner and progress bar components
- Status update system

#### Phase 6: Orchestration Engine (Itamae Package)
- Bento orchestration engine
- Group execution with dependency resolution
- Context passing between neta
- Edge-based execution flow
- Error handling and validation
- Template variable substitution

#### Phase 5: Neta Registry (Pantry Package)
- Neta type registry system
- Factory pattern for neta instantiation
- Built-in neta types registration
- Extensible architecture for custom neta

#### Phases 2-4: Core Infrastructure
- **Shoyu** (pkg/shoyu): Logging with charm/log integration
- **Omakase** (pkg/omakase): Configuration management
- **Hangiri** (pkg/hangiri): Core models and definitions

#### Neta Implementations
- **spreadsheet**: Read/write CSV and Excel files
- **http-request**: HTTP client with full request support
- **transform**: Data transformation with templates
- **edit-fields**: Set/modify field values
- **file-system**: File and directory operations
- **shell-command**: Execute shell commands
- **image**: Image processing and optimization
- **loop**: Iterate over collections
- **parallel**: Concurrent execution
- **group**: Orchestrate multiple neta

### Documentation
- Bento Box Principle (coding philosophy)
- Package naming conventions
- Go standards review
- Complete node inventory
- Status word guidelines
- Emoji usage standards
- Charm stack integration guide
- Phase strategy documents (Phases 1-8)

### Infrastructure
- Go module setup
- golangci-lint configuration
- Git workflow and hooks
- Project structure following Go best practices

## Version Numbering

Bento follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version (1.0.0): Incompatible API changes
- **MINOR** version (0.1.0): New functionality in a backward compatible manner
- **PATCH** version (0.0.1): Backward compatible bug fixes

### Pre-1.0.0 Versioning

While in 0.x.x versions (pre-1.0):
- **MINOR** version increments may include breaking changes
- **PATCH** version increments are for backward compatible changes
- Version 1.0.0 will be released when the API is stable

### Current Status

**Current Version:** 0.1.0 (Development)

We're currently in **Phase 8** of development, building integration tests and validating the complete system with real-world workflows.

### Upcoming Versions

- **0.2.0**: Phase 8 completion (real-world integration tests)
- **0.3.0**: Additional neta types and features
- **0.4.0**: Performance optimizations
- **0.5.0**: Enhanced error handling and validation
- **1.0.0**: Stable API, production-ready release

## How to Update This Changelog

When completing a phase or adding features:

1. **Add to [Unreleased] section** during development
2. **Create a new version section** when releasing
3. **Use these categories:**
   - `Added` - New features
   - `Changed` - Changes to existing functionality
   - `Deprecated` - Soon-to-be removed features
   - `Removed` - Removed features
   - `Fixed` - Bug fixes
   - `Security` - Security fixes

4. **Follow this format:**
   ```markdown
   ### Added
   - Feature description with context
   - Another feature with details
   ```

## Git Tags and Releases

To create a new release:

```bash
# Update CHANGELOG.md with version and date
# Commit the changelog
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# Create and push tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
git push origin main

# GitHub will automatically create a release from the tag
```

## Links

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [Go Modules](https://go.dev/blog/using-go-modules)

---

**Note:** This changelog started with version 0.1.0. Earlier development history is preserved in git commits but not detailed here for brevity.
