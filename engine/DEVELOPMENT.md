# 🛠️ Development Guide

Welcome to bento development! This guide will help you get started, whether you're new to Go or coming from other languages like Node.js.

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Running Commands](#running-commands)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Common Tasks](#common-tasks)
- [Node.js to Go Translation](#nodejs-to-go-translation)
- [Troubleshooting](#troubleshooting)

---

## 🚀 Quick Start

### Prerequisites

- **Go 1.21+** - [Install Go](https://go.dev/doc/install)
- **golangci-lint** - `brew install golangci-lint` (macOS)
- **Optional: glow** - `brew install glow` (for viewing docs with `bento recipe`)

### First Time Setup

```bash
# Clone the repository
git clone https://github.com/Develonaut/bento.git
cd bento

# Download dependencies
go mod download

# Run tests to verify setup
go test ./...

# Try running bento in dev mode
go run ./cmd/bento --help
```

That's it! You're ready to develop. 🎉

---

## 💻 Development Workflow

### The Golden Rule: Stay in Project Root

**Always run commands from `/Users/Ryan/Code/bento` (the project root)**

```bash
# Check you're in the right place
pwd
# Should show: /Users/Ryan/Code/bento

# You should see these directories
ls
# cmd/  pkg/  tests/  examples/  go.mod  README.md
```

### Dev Mode (Recommended)

Use `go run` to compile and execute in one step - **this is your "dev mode"**:

```bash
# Run bento commands during development
go run ./cmd/bento --help
go run ./cmd/bento savor examples/csv-reader.bento.json
go run ./cmd/bento sample examples/csv-reader.bento.json
go run ./cmd/bento recipe principles
```

**Why `go run`?**
- ✅ Always uses latest code (no need to rebuild)
- ✅ Fast iteration cycle
- ✅ No binary clutter in your working directory
- ✅ Like running `node src/index.js` in Node.js

### When to Install Globally

Only install when you want to test the "production" binary:

```bash
# Install to $GOPATH/bin (usually ~/go/bin)
go install ./cmd/bento

# Now 'bento' works from anywhere
bento savor examples/csv-reader.bento.json
cd ~/Documents
bento --help  # Still works!
```

**Remember:** After `go install`, you must reinstall to see code changes!

---

## 📂 Project Structure

```
bento/
├── cmd/
│   └── bento/              # CLI application (main package)
│       ├── main.go         # Entry point, command registration
│       ├── savor.go        # Execute bento command
│       ├── sample.go       # Validate bento command
│       ├── menu.go         # List bentos command
│       ├── box.go          # Create template command
│       ├── recipe.go       # View docs command
│       ├── version.go      # Version command
│       └── main_test.go    # CLI integration tests
│
├── pkg/                    # Reusable packages (the "library" code)
│   ├── neta/              # Neta (node) types
│   │   ├── spreadsheet/   # CSV/Excel reading
│   │   ├── http/          # HTTP requests
│   │   ├── transform/     # Data transformation
│   │   └── ...
│   ├── itamae/            # Orchestration engine
│   ├── miso/              # UI components (progress bars, etc.)
│   └── shoyu/             # Logging utilities
│
├── tests/
│   ├── integration/       # End-to-end integration tests
│   │   ├── csv_reader_test.go
│   │   ├── test_utilities.go
│   │   └── helpers_test.go
│   └── fixtures/          # Test data files
│       └── products-test.csv
│
├── examples/              # Example bento files
│   └── csv-reader.bento.json
│
├── .claude/               # Development documentation
│   ├── BENTO_BOX_PRINCIPLE.md
│   ├── GO_STANDARDS_REVIEW.md
│   └── strategy/          # Phase implementation docs
│
├── go.mod                 # Go module definition (like package.json)
├── go.sum                 # Dependency checksums (like package-lock.json)
├── README.md              # User documentation
└── DEVELOPMENT.md         # This file!
```

### Key Concepts

- **`cmd/`** = Executables (CLIs, servers)
- **`pkg/`** = Reusable libraries
- **`tests/`** = Test files (can also be alongside code as `*_test.go`)

---

## 🏃 Running Commands

### Development Commands

```bash
# View help
go run ./cmd/bento --help

# Execute a bento
go run ./cmd/bento savor examples/csv-reader.bento.json

# Validate a bento
go run ./cmd/bento sample examples/csv-reader.bento.json

# List bentos in a directory
go run ./cmd/bento menu examples

# Create new bento template
go run ./cmd/bento box my-workflow

# View documentation (requires glow)
go run ./cmd/bento recipe principles

# Check version
go run ./cmd/bento version
```

### Build & Install

```bash
# Build binary in current directory
go build -o bento ./cmd/bento
./bento --help

# Install globally to $GOPATH/bin
go install ./cmd/bento
bento --help  # Works from anywhere now

# Where is the installed binary?
which bento
# Shows: /Users/Ryan/go/bin/bento
```

---

## 🧪 Testing

### Run All Tests

```bash
# Run all tests
go test ./...

# Run with verbose output
go test -v ./...

# Run with race detector (detects concurrency bugs)
go test -race ./...

# Run tests in specific package
go test ./pkg/neta/spreadsheet/...
go test ./tests/integration/...
go test ./cmd/bento/...
```

### Run Specific Tests

```bash
# Run tests matching pattern
go test -v -run TestCSVReader ./tests/integration/...

# Run specific test
go test -v -run TestSavorCommand_ValidBento ./cmd/bento/...

# Run with coverage
go test -cover ./...
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out  # View in browser
```

### Watch Tests (Auto-Rerun on Changes)

**Option 1: Using entr (simple)**
```bash
# Install entr
brew install entr

# Watch and rerun tests
find . -name "*.go" | entr -c go test ./...
```

**Option 2: Using air (popular)**
```bash
# Install air
go install github.com/cosmtrek/air@latest

# Run with auto-reload
air
```

---

## ✨ Code Quality

### Formatting

```bash
# Format all Go code (required before commit)
go fmt ./...

# Check formatting without changing files
gofmt -l .
```

### Linting

```bash
# Run all linters
golangci-lint run ./...

# Run with auto-fix
golangci-lint run --fix ./...

# Run specific linters
golangci-lint run --disable-all --enable=errcheck ./...
```

### Pre-Commit Checklist

Before committing, always run:

```bash
# 1. Format code
go fmt ./...

# 2. Run linters
golangci-lint run ./...

# 3. Run all tests
go test ./...

# 4. Run tests with race detector
go test -race ./...

# Or run all at once:
go fmt ./... && golangci-lint run ./... && go test -race ./...
```

---

## 🔧 Common Tasks

### Adding a New CLI Command

1. Create new file in `cmd/bento/`:
```bash
touch cmd/bento/mycommand.go
```

2. Define the command:
```go
package main

import "github.com/spf13/cobra"

var myCmd = &cobra.Command{
    Use:   "mycommand [args]",
    Short: "🎯 Short description",
    RunE:  runMyCommand,
}

func runMyCommand(cmd *cobra.Command, args []string) error {
    // Implementation here
    return nil
}
```

3. Register in `cmd/bento/main.go`:
```go
func init() {
    rootCmd.AddCommand(savorCmd)
    rootCmd.AddCommand(myCmd)  // Add this line
}
```

4. Add tests in `cmd/bento/main_test.go`:
```go
func TestMyCommand_Works(t *testing.T) {
    output := verifyCommandSuccess(t,
        exec.Command("bento", "mycommand"),
        "expected output")
}
```

5. Test in dev mode:
```bash
go run ./cmd/bento mycommand
```

### Adding a New Neta Type

1. Create package in `pkg/neta/`:
```bash
mkdir -p pkg/neta/myneta
touch pkg/neta/myneta/myneta.go
```

2. Implement the `Executable` interface:
```go
package myneta

import "context"

type MyNeta struct{}

func (n *MyNeta) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
    // Implementation
    return nil, nil
}
```

3. Register in the pantry (neta registry)

4. Add tests:
```bash
touch pkg/neta/myneta/myneta_test.go
```

### Creating Integration Tests

1. Create test file in `tests/integration/`:
```go
package integration

import (
    "testing"
    "path/filepath"
)

func TestMyFeature_Integration(t *testing.T) {
    bentoPath := filepath.Join("examples", "my-feature.bento.json")

    output, err := RunBento(t, bentoPath, nil)
    if err != nil {
        t.Fatalf("RunBento failed: %v", err)
    }

    // Assertions here
}
```

2. Run the test:
```bash
go test -v ./tests/integration/... -run TestMyFeature
```

---

## 🔄 Node.js to Go Translation

### Package Management

| Node.js | Go |
|---------|-----|
| `npm install package` | `go get package` |
| `npm install` | `go mod download` |
| `npm update` | `go get -u` |
| `package.json` | `go.mod` |
| `package-lock.json` | `go.sum` |
| `node_modules/` | `$GOPATH/pkg/mod/` |

### Running & Building

| Node.js | Go | Notes |
|---------|-----|-------|
| `node src/index.js` | `go run ./cmd/bento` | Dev mode |
| `npm run dev` | `go run ./cmd/bento` | Quick iteration |
| `npm run build` | `go build -o bento ./cmd/bento` | Create binary |
| `npm install -g .` | `go install ./cmd/bento` | Install globally |
| `npm test` | `go test ./...` | Run tests |
| `npm run lint` | `golangci-lint run ./...` | Linting |

### File Structure

| Node.js | Go |
|---------|-----|
| `src/` | `cmd/` or `pkg/` |
| `index.js` | `main.go` |
| `lib/` | `pkg/` |
| `test/` | `tests/` or `*_test.go` |
| `require()` / `import` | `import` |
| `module.exports` | `package` + capital letter exports |

### Common Patterns

**Exporting Functions:**

```javascript
// Node.js
module.exports = {
    myFunction: function() { }
}
```

```go
// Go - Capital letter = exported
package mypackage

func MyFunction() { }  // Exported
func myHelper() { }    // Private to package
```

**Error Handling:**

```javascript
// Node.js
try {
    doSomething();
} catch (err) {
    console.error(err);
}
```

```go
// Go - explicit error returns
result, err := doSomething()
if err != nil {
    return fmt.Errorf("failed: %w", err)
}
```

---

## 🔌 External Dependencies (glow)

### Why Doesn't Bento Auto-Install glow?

**Short Answer:** This is standard practice for Go CLIs! External tools remain separate.

### The Philosophy

Bento follows the **industry standard approach** used by popular CLIs:

| CLI | External Dependency | Why Separate? |
|-----|---------------------|---------------|
| **git** | Doesn't bundle text editor | Users choose vim/nano/emacs |
| **docker** | Doesn't include docker-compose | Separate versioning |
| **kubectl** | Doesn't bundle helm | Different release cycles |
| **gh** | Doesn't include glow | Optional enhancement |
| **bento** | Doesn't include glow | Same reasons! |

### Benefits of External Dependencies

1. **Smaller Binary Size**
   - bento: ~10MB without glow bundled
   - Would be ~15MB+ with glow embedded

2. **User Choice**
   - Users can update glow independently
   - Can use different versions if needed
   - Some users may already have it installed

3. **Clear Separation**
   - glow is maintained by Charm
   - bento is maintained separately
   - Each tool has its own release cycle

4. **Better Error Messages**
   - Helpful installation instructions
   - OS-specific guidance (brew, apt, etc.)

### How It Works

When you run `bento recipe` without glow:

```bash
$ bento recipe principles

Error: glow is not installed

Glow is required to view documentation in a beautiful format.

Install with:
  brew install glow        # macOS
  sudo apt install glow    # Linux (Debian/Ubuntu)
  go install github.com/charmbracelet/glow@latest  # Any OS

Or visit: https://github.com/charmbracelet/glow
```

### One-Time Setup

Just install glow once:

```bash
# macOS
brew install glow

# Linux (Debian/Ubuntu)
sudo apt install glow

# Any OS (if you have Go)
go install github.com/charmbracelet/glow@latest

# Windows
scoop install glow
# or
choco install glow
```

Then `bento recipe` works forever!

### Alternative: Read Raw Markdown

You can always read docs without glow:

```bash
# View raw markdown
cat .claude/BENTO_BOX_PRINCIPLE.md
less .claude/BENTO_BOX_PRINCIPLE.md

# Or use your editor
code .claude/BENTO_BOX_PRINCIPLE.md
vim .claude/BENTO_BOX_PRINCIPLE.md
```

### For Other Developers

When sharing bento with your team, document in README:

```markdown
## Prerequisites

- Go 1.21+ (required)
- glow (optional, for `bento recipe` command)
  ```bash
  brew install glow
  ```
```

This way expectations are clear!

---

## 🐛 Troubleshooting

### Command not found after `go install`

**Problem:** `bento: command not found`

**Solution:** Add `$GOPATH/bin` to your PATH:

```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="$HOME/go/bin:$PATH"

# Reload shell
source ~/.zshrc
```

### Changes not reflected in `bento` command

**Problem:** Code changes don't appear when running `bento`

**Cause:** You're using the installed binary, which is outdated

**Solution:**
```bash
# Option 1: Use go run (recommended for development)
go run ./cmd/bento savor examples/csv-reader.bento.json

# Option 2: Reinstall
go install ./cmd/bento
bento savor examples/csv-reader.bento.json
```

### Import cycle detected

**Problem:** `import cycle not allowed`

**Cause:** Package A imports Package B, which imports Package A

**Solution:** Refactor to break the cycle. Common approaches:
- Extract common code to a third package
- Use interfaces to break dependencies
- Rethink package boundaries

### Tests fail with "file not found"

**Problem:** Tests can't find fixture files

**Cause:** Running from wrong directory or using wrong path

**Solution:**
```bash
# Always run from project root
cd /Users/Ryan/Code/bento
go test ./...

# Use correct relative paths in tests
bentoPath := filepath.Join("examples", "csv-reader.bento.json")
```

### Module cache issues

**Problem:** Strange errors about dependencies

**Solution:**
```bash
# Clean module cache
go clean -modcache

# Re-download dependencies
go mod download

# Verify and clean up
go mod tidy
```

---

## 📚 Learning Resources

### Go Basics
- [A Tour of Go](https://go.dev/tour/) - Interactive tutorial
- [Go by Example](https://gobyexample.com/) - Code examples
- [Effective Go](https://go.dev/doc/effective_go) - Best practices

### Testing
- [Go Testing Package](https://pkg.go.dev/testing)
- [Table Driven Tests](https://dave.cheney.net/2019/05/07/prefer-table-driven-tests)

### Project-Specific
- `.claude/BENTO_BOX_PRINCIPLE.md` - Our coding philosophy
- `.claude/GO_STANDARDS_REVIEW.md` - Go standards for this project
- `bento recipe principles` - View principles in terminal

---

## 🎯 Quick Reference

### Most Common Commands

```bash
# Development
go run ./cmd/bento --help                    # Run in dev mode
go run ./cmd/bento savor examples/*.json     # Execute bento

# Testing
go test ./...                                 # Run all tests
go test -v -run TestName ./pkg/...           # Run specific test
go test -race ./...                          # Check for race conditions

# Code Quality
go fmt ./...                                  # Format code
golangci-lint run ./...                      # Lint code

# Building
go build -o bento ./cmd/bento                # Build binary
go install ./cmd/bento                       # Install globally

# Dependencies
go mod tidy                                   # Clean up dependencies
go mod download                               # Download dependencies
```

### File Naming Conventions

- `*.go` - Go source files
- `*_test.go` - Test files (in same package)
- `main.go` - Entry point for executables
- `doc.go` - Package documentation

### Import Paths

```go
// Standard library (no domain)
import "fmt"
import "context"

// External packages (with domain)
import "github.com/spf13/cobra"

// This project's packages
import "github.com/Develonaut/bento/pkg/neta"
```

---

## 🤝 Contributing

1. Read `.claude/BENTO_BOX_PRINCIPLE.md`
2. Make your changes
3. Run quality checks:
   ```bash
   go fmt ./... && golangci-lint run ./... && go test -race ./...
   ```
4. Add tests for new features
5. Update documentation

---

## 💡 Tips

- **Use `go run` during development** - It's faster than rebuilding
- **Run tests frequently** - `go test ./...` is very fast
- **Format before committing** - `go fmt ./...` is required
- **Read error messages carefully** - Go's errors are usually very descriptive
- **Use the race detector** - `go test -race` catches concurrency bugs
- **Keep functions small** - Target < 20 lines
- **Keep files focused** - Target < 250 lines

---

**Happy coding!** 🍱

Questions? Check the `.claude/` directory for more documentation, or run:
```bash
go run ./cmd/bento recipe --help
```
