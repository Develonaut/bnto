# Go Versioning Guide for Bento

## Understanding Go Versioning

Go uses **Semantic Versioning** (SemVer) with a specific approach to modules and releases.

---

## Semantic Versioning (SemVer)

Format: `MAJOR.MINOR.PATCH` (e.g., `v1.2.3`)

```
v1.2.3
│ │ │
│ │ └─ PATCH: Bug fixes (backward compatible)
│ └─── MINOR: New features (backward compatible)
└───── MAJOR: Breaking changes (incompatible API changes)
```

### When to Increment

| Version | When | Example |
|---------|------|---------|
| **MAJOR** | Breaking API changes | Removing a command, changing function signature |
| **MINOR** | New features (compatible) | Adding new neta type, new CLI flag |
| **PATCH** | Bug fixes (compatible) | Fixing a crash, correcting output |

---

## Pre-1.0.0 Versions (Where We Are Now)

**Current Status: v0.1.0 (Development)**

### Rules for 0.x.x Versions

While under 1.0.0:
- API can change freely
- **MINOR** bumps may include breaking changes
- **PATCH** bumps are for small fixes
- No stability guarantees

**Example progression:**
```
v0.1.0 → v0.2.0 → v0.3.0 → ... → v1.0.0
```

### When to Release 1.0.0

Release 1.0.0 when:
- ✅ API is stable and won't change
- ✅ All core features working
- ✅ Production-ready
- ✅ Breaking changes are rare
- ✅ Ready to maintain backward compatibility

**For bento:** After Phase 8 completes successfully!

---

## How Versioning Works in Go

### 1. Version Lives in Git Tags

Unlike Node.js `package.json`, Go versions are **git tags**:

```bash
# Node.js
{
  "version": "1.2.3"  // In package.json
}

# Go
git tag v1.2.3  // Version is a git tag
```

### 2. The Version Variable

The version shown by `bento version` comes from a build-time variable:

**In code:**
```go
// cmd/bento/main.go
var version = "dev"  // Default for development
```

**At build time:**
```bash
# Set version during build
go build -ldflags "-X main.version=v0.2.0" ./cmd/bento
```

### 3. go.mod File

The `go.mod` file declares the **module path**, not the version:

```go
module github.com/Develonaut/bento

go 1.21
```

**Note:** The version in `go.mod` is the **Go language version**, not your app version!

---

## How to Release a New Version

### Step-by-Step Release Process

#### 1. Update CHANGELOG.md

Move items from `[Unreleased]` to a new version section:

```markdown
## [Unreleased]

(empty or new work in progress)

## [0.2.0] - 2025-10-20

### Added
- Phase 8.3 integration tests
- HTTP request bento example
```

#### 2. Commit the Changelog

```bash
git add CHANGELOG.md
git commit -m "chore: Prepare release v0.2.0"
```

#### 3. Create Git Tag

```bash
# Lightweight tag (simple)
git tag v0.2.0

# Annotated tag (recommended - includes message and metadata)
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8.3 complete"
```

#### 4. Push Tag and Code

```bash
# Push the commit
git push origin main

# Push the tag
git push origin v0.2.0

# Or push all tags at once
git push --tags
```

#### 5. Verify on GitHub

GitHub will automatically:
- Show the tag in the "Releases" section
- Allow you to create a release from the tag
- Make it available for `go get`

---

## Practical Examples

### Scenario 1: Just Completed Phase 8.2

**What we did:**
- Added integration test
- Added CLI tests
- Added documentation

**What to do:**
```bash
# Update CHANGELOG.md (already done)
# We'll release as v0.2.0 when Phase 8 is complete

# For now, keep in [Unreleased] section
```

### Scenario 2: Completed All of Phase 8

**What to do:**
```bash
# 1. Update CHANGELOG.md
vim CHANGELOG.md
# Move [Unreleased] to [0.2.0] - 2025-10-XX

# 2. Commit
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# 3. Tag
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8 complete"

# 4. Push
git push origin main
git push origin v0.2.0
```

### Scenario 3: Fixed a Bug

**What to do:**
```bash
# Fix the bug, then:

# 1. Update CHANGELOG.md
## [0.2.1] - 2025-10-21
### Fixed
- Fixed crash when CSV file is empty

# 2. Create patch tag
git tag -a v0.2.1 -m "Fix: CSV empty file crash"
git push origin v0.2.1
```

---

## Build with Version

### During Development

```bash
# Normal dev build (version = "dev")
go build ./cmd/bento
./bento version
# Output: bento version dev
```

### For Release

```bash
# Build with specific version
go build -ldflags "-X main.version=v0.2.0" -o bento ./cmd/bento
./bento version
# Output: bento version v0.2.0
```

### Using Makefile (Recommended)

Create a `Makefile`:

```makefile
VERSION ?= $(shell git describe --tags --always --dirty)

build:
	go build -ldflags "-X main.version=$(VERSION)" -o bento ./cmd/bento

install:
	go install -ldflags "-X main.version=$(VERSION)" ./cmd/bento

release:
	@echo "Building release for version $(VERSION)"
	go build -ldflags "-X main.version=$(VERSION)" -o bento ./cmd/bento
```

Then:
```bash
make build    # Builds with git tag version
make install  # Installs with version
```

---

## Versioning Best Practices

### 1. Always Update CHANGELOG.md First

```bash
# Good workflow:
1. Update CHANGELOG.md
2. Commit: "chore: Release vX.Y.Z"
3. Tag: git tag vX.Y.Z
4. Push tag
```

### 2. Use Annotated Tags

```bash
# Better (includes metadata)
git tag -a v0.2.0 -m "Release v0.2.0"

# vs. lightweight tag
git tag v0.2.0
```

### 3. Prefix with 'v'

Go convention uses `v` prefix:
```bash
v0.1.0  # Good
0.1.0   # Works, but not Go convention
```

### 4. Never Delete Tags

If you mess up:
```bash
# Don't delete and recreate
git tag -d v0.2.0  # Bad

# Instead, create a new patch version
git tag v0.2.1     # Good
```

### 5. Document Breaking Changes

In CHANGELOG.md:
```markdown
### Changed
- **BREAKING**: Removed `--old-flag` (use `--new-flag` instead)
```

---

## Checking Versions

### View All Tags

```bash
git tag
# Lists: v0.1.0, v0.2.0, etc.
```

### View Tag Details

```bash
git show v0.2.0
# Shows commit, message, diff
```

### Current Version in Code

```bash
go run ./cmd/bento version
# or
bento version
```

### Latest Tag

```bash
git describe --tags --abbrev=0
# Shows: v0.2.0
```

---

## Comparison: Node.js vs Go

| Aspect | Node.js | Go |
|--------|---------|-----|
| **Version Location** | `package.json` | Git tag |
| **Bump Version** | `npm version patch` | `git tag v0.2.1` |
| **View Version** | `npm version` | `git tag` |
| **Build Version** | Read from JSON | Pass via ldflags |
| **Publish** | `npm publish` | `git push --tags` |

---

## Recommended Workflow for Bento

### After Completing Each Phase

```bash
# 1. Update CHANGELOG.md
# Add completed features to [Unreleased]

# 2. Commit changes
git add .
git commit -m "feat(phase-X): Complete Phase X"
git push

# 3. DON'T tag yet - wait for major milestone
```

### After Completing Major Milestone (e.g., Phase 8)

```bash
# 1. Update CHANGELOG.md
# Move [Unreleased] to [0.2.0]

# 2. Commit
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# 3. Tag
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8 complete"

# 4. Push
git push origin main
git push origin v0.2.0
```

### Quick Reference

```bash
# View current tags
git tag

# Create new version
git tag -a v0.X.0 -m "Release v0.X.0"

# Push tag
git push origin v0.X.0

# Delete local tag (if mistake)
git tag -d v0.X.0

# Build with version
go build -ldflags "-X main.version=$(git describe --tags)" ./cmd/bento
```

---

## Summary

1. **Version is in git tags**, not a file
2. **Use SemVer**: MAJOR.MINOR.PATCH
3. **Pre-1.0.0**: Breaking changes allowed in MINOR
4. **Update CHANGELOG.md** with each release
5. **Tag format**: `v0.2.0` (with 'v' prefix)
6. **Push tags** to make releases available

**Current Status:** v0.1.0 (Development)
**Next Release:** v0.2.0 (after Phase 8 complete)

---

For questions, see:
- https://semver.org/
- https://go.dev/doc/modules/version-numbers
- https://keepachangelog.com/
