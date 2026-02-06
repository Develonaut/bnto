# 📋 Release Checklist

A step-by-step guide for creating releases in the bento project.

**Use this checklist every time you're ready to release a new version.**

---

## 🎯 When to Release

### Minor Version (0.X.0)
- ✅ Completed a major phase (e.g., Phase 8)
- ✅ Added significant new features
- ✅ Multiple sub-features completed
- ✅ Major milestone reached

**Example:** Phase 8 complete → v0.2.0

### Patch Version (0.1.X)
- ✅ Fixed critical bugs
- ✅ Small improvements
- ✅ Documentation updates
- ✅ No new features

**Example:** Fixed CSV crash → v0.1.1

### Major Version (1.0.0)
- ✅ API is stable
- ✅ Production-ready
- ✅ All planned features complete
- ✅ Ready for public use

**Example:** After all Phase 8 sub-phases, polish, and testing → v1.0.0

---

## ✅ Pre-Release Checklist

Before starting the release process, ensure:

- [ ] All tests pass: `go test ./...`
- [ ] All tests pass with race detector: `go test -race ./...`
- [ ] Code is formatted: `go fmt ./...`
- [ ] Linting passes: `golangci-lint run ./...`
- [ ] All changes committed: `git status` shows clean
- [ ] On main branch: `git branch` shows `* main`
- [ ] Latest code pulled: `git pull origin main`
- [ ] CI/CD passing (if you have it set up)

**Quick check command:**
```bash
go fmt ./... && \
golangci-lint run ./... && \
go test -race ./... && \
git status
```

If any of these fail, fix issues before proceeding!

---

## 📝 Release Process

### Step 1: Determine Version Number

**Current version:** Check `git tag` or CHANGELOG.md

**What kind of release?**

| Type | Increment | Example |
|------|-----------|---------|
| **Bug fixes only** | PATCH | 0.1.0 → 0.1.1 |
| **New features** | MINOR | 0.1.0 → 0.2.0 |
| **Breaking changes (pre-1.0)** | MINOR | 0.2.0 → 0.3.0 |
| **Stable release** | MAJOR | 0.9.0 → 1.0.0 |

**Decision:**
```bash
# For this release, I'm creating version: v_____
```

---

### Step 2: Update CHANGELOG.md

**2.1 Open CHANGELOG.md**
```bash
vim CHANGELOG.md
# or
code CHANGELOG.md
```

**2.2 Find the [Unreleased] section**

It should look like:
```markdown
## [Unreleased]

### Added
- Feature 1
- Feature 2

### Fixed
- Bug fix
```

**2.3 Create new version section**

Move content from [Unreleased] to a new version section:

```markdown
## [Unreleased]

(This section is now empty - ready for next features)

## [0.2.0] - 2025-10-20

### Added
- Feature 1
- Feature 2

### Fixed
- Bug fix
```

**2.4 Update the date**

Use today's date in ISO format: `YYYY-MM-DD`

**2.5 Check categories**

Use appropriate categories:
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security fixes

**2.6 Save the file**

---

### Step 3: Review Changes

**3.1 Review what you're releasing**
```bash
git diff HEAD CHANGELOG.md
```

**3.2 Double-check the version number**
- Is it correct?
- Does it follow SemVer?
- Did you increment the right part (MAJOR/MINOR/PATCH)?

**3.3 Verify completeness**
- Did you list all major changes?
- Are breaking changes clearly marked?
- Is the date correct?

---

### Step 4: Commit the Changelog

**4.1 Stage the changelog**
```bash
git add CHANGELOG.md
```

**4.2 Verify what's staged**
```bash
git status
# Should show only CHANGELOG.md
```

**4.3 Commit with conventional commit message**
```bash
git commit -m "chore: Release v0.2.0"
```

**Format:** `chore: Release vX.Y.Z`

**4.4 Verify commit**
```bash
git log -1
# Check that commit message is correct
```

---

### Step 5: Create Git Tag

**5.1 Create annotated tag**
```bash
git tag -a v0.2.0 -m "Release v0.2.0 - Brief description"
```

**Format:**
- Tag name: `vX.Y.Z` (with 'v' prefix)
- Message: Brief description of the release

**Examples:**
```bash
# Good tag messages
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8 integration tests"
git tag -a v0.3.0 -m "Release v0.3.0 - Performance improvements"
git tag -a v1.0.0 -m "Release v1.0.0 - Stable API"

# Patch release
git tag -a v0.2.1 -m "Release v0.2.1 - Fix CSV empty file crash"
```

**5.2 Verify tag was created**
```bash
git tag
# Should show v0.2.0 in the list

git show v0.2.0
# Should show tag details and commit
```

---

### Step 6: Push to Remote

**6.1 Push the commit**
```bash
git push origin main
```

**6.2 Push the tag**
```bash
git push origin v0.2.0
```

**Or push all tags at once:**
```bash
git push --tags
```

**6.3 Verify on GitHub**

1. Go to: https://github.com/Develonaut/bento/tags
2. You should see `v0.2.0` in the list
3. Click on it to view details

---

### Step 7: Create GitHub Release (Optional but Recommended)

**7.1 Go to GitHub Releases**
- Navigate to: https://github.com/Develonaut/bento/releases
- Click "Draft a new release"

**7.2 Fill in release details**
- **Tag:** Select `v0.2.0` from dropdown
- **Release title:** `v0.2.0 - Brief Description`
- **Description:** Copy from CHANGELOG.md

**Example description:**
```markdown
## What's New in v0.2.0

### Added
- Phase 8.2: CSV reader integration test
- Complete CLI test coverage (15 tests)
- Comprehensive development guide (DEVELOPMENT.md)

### Changed
- Improved error messages for missing dependencies

### Fixed
- Linting errors in progress tests

**Full Changelog**: https://github.com/Develonaut/bento/blob/main/CHANGELOG.md
```

**7.3 Publish release**
- Check "Set as the latest release" if appropriate
- Click "Publish release"

---

### Step 8: Verify Everything

**8.1 Check git status**
```bash
git status
# Should be clean

git log -1
# Should show your "chore: Release vX.Y.Z" commit
```

**8.2 Verify tag exists locally**
```bash
git tag
# Should show v0.2.0
```

**8.3 Verify tag exists on GitHub**
```bash
git ls-remote --tags origin
# Should show refs/tags/v0.2.0
```

**8.4 Test installation (optional)**
```bash
# Others can now install with:
go install github.com/Develonaut/bento/cmd/bento@v0.2.0
```

---

### Step 9: Announce (Optional)

If you have users or a team:

**Update README.md with new version:**
```markdown
## Installation

```bash
go install github.com/Develonaut/bento/cmd/bento@v0.2.0
```
```

**Announce in:**
- Project Slack/Discord
- README.md
- Social media
- Team meeting

---

## 🔄 Post-Release

**9.1 Start new [Unreleased] section**

CHANGELOG.md should now have an empty [Unreleased] section ready for the next release:

```markdown
## [Unreleased]

(Add new changes here as you work)

## [0.2.0] - 2025-10-20
...
```

**9.2 Continue development**

You're ready to start work on the next version!

---

## 🚨 Common Mistakes to Avoid

### ❌ Mistake 1: Forgot to update CHANGELOG.md
**What happens:** Tag exists but no documented changes
**Fix:** Update CHANGELOG.md and create a new patch release

### ❌ Mistake 2: Wrong version number
**What happens:** Confusion about what's in the release
**Fix:** Don't delete tags! Create a new patch version instead

### ❌ Mistake 3: Forgot to push tags
**What happens:** Tag exists locally but not on GitHub
**Fix:** `git push origin v0.2.0` or `git push --tags`

### ❌ Mistake 4: Breaking changes in patch version
**What happens:** Users get broken on minor update
**Fix:** Use MINOR version for breaking changes (pre-1.0)

### ❌ Mistake 5: Uncommitted changes
**What happens:** Release doesn't include latest work
**Fix:** Always commit everything before tagging

---

## 📱 Quick Reference Card

**Copy this to keep handy:**

```bash
# 1. Ensure everything is ready
go test ./... && golangci-lint run ./... && git status

# 2. Update CHANGELOG.md (manually in editor)
vim CHANGELOG.md

# 3. Commit changelog
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# 4. Create tag
git tag -a v0.2.0 -m "Release v0.2.0 - Brief description"

# 5. Push everything
git push origin main
git push origin v0.2.0

# 6. Verify
git tag
git ls-remote --tags origin
```

---

## 🎓 Learning Examples

### Example 1: Patch Release (Bug Fix)

**Scenario:** Fixed a crash in CSV reader

```bash
# 1. Fix the bug and commit it
git add .
git commit -m "fix: Prevent crash when CSV file is empty"
git push

# 2. Update CHANGELOG.md
# Move fix to [0.1.1] section with today's date

# 3. Commit changelog
git add CHANGELOG.md
git commit -m "chore: Release v0.1.1"

# 4. Create tag
git tag -a v0.1.1 -m "Release v0.1.1 - Fix CSV empty file crash"

# 5. Push
git push origin main
git push origin v0.1.1
```

### Example 2: Minor Release (New Feature)

**Scenario:** Completed Phase 8.3

```bash
# 1. All Phase 8.3 work already committed
# Verify clean state
git status

# 2. Update CHANGELOG.md
# Move all Phase 8.3 features to [0.2.0] section

# 3. Commit changelog
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# 4. Create tag
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8.3 complete"

# 5. Push
git push origin main
git push origin v0.2.0

# 6. Create GitHub release (optional)
# Go to GitHub and create release from tag
```

### Example 3: Major Release (1.0.0)

**Scenario:** API is stable, ready for production

```bash
# 1. Review everything one more time
go test ./...
golangci-lint run ./...
git status

# 2. Update CHANGELOG.md
# Add [1.0.0] section with all final features
# Mark as stable release

# 3. Commit changelog
git add CHANGELOG.md
git commit -m "chore: Release v1.0.0 - Stable release"

# 4. Create tag
git tag -a v1.0.0 -m "Release v1.0.0 - Stable API, production ready"

# 5. Push
git push origin main
git push origin v1.0.0

# 6. Create GitHub release and announce!
# This is a major milestone - celebrate! 🎉
```

---

## 🆘 Troubleshooting

### Problem: Accidentally created wrong tag

**Solution:**
```bash
# Delete local tag
git tag -d v0.2.0

# If already pushed, delete remote tag (use with caution!)
git push origin :refs/tags/v0.2.0

# Create correct tag
git tag -a v0.2.1 -m "Correct release"
git push origin v0.2.1
```

**Better solution:** Don't delete, create new patch version instead!

### Problem: Forgot to update CHANGELOG.md before tagging

**Solution:**
```bash
# Update CHANGELOG.md now
vim CHANGELOG.md

# Commit it
git add CHANGELOG.md
git commit -m "docs: Update CHANGELOG for v0.2.0"
git push

# Tag is still valid, just update GitHub release description
```

### Problem: Tag pushed but tests are now failing

**Solution:**
```bash
# Fix the issue
git add .
git commit -m "fix: Critical bug in v0.2.0"
git push

# Create patch release
git tag -a v0.2.1 -m "Release v0.2.1 - Fix critical bug"
git push origin v0.2.1
```

---

## ✅ Final Checklist

Before you finish:

- [ ] CHANGELOG.md updated with correct version and date
- [ ] "chore: Release vX.Y.Z" commit created
- [ ] Git tag created with `git tag -a vX.Y.Z -m "message"`
- [ ] Both commit and tag pushed to GitHub
- [ ] Tag visible on GitHub: github.com/Develonaut/bento/tags
- [ ] GitHub release created (optional)
- [ ] Tests still passing
- [ ] Version number follows SemVer
- [ ] [Unreleased] section ready for next changes

**All checked?**

🎉 **Congratulations! You've successfully released version X.Y.Z!** 🎉

---

## 📚 Additional Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Go Module Version Numbers](https://go.dev/doc/modules/version-numbers)
- [Git Tagging](https://git-scm.com/book/en/v2/Git-Basics-Tagging)
- `.claude/VERSIONING.md` - Full versioning guide for this project

---

**Keep this checklist bookmarked and follow it every release!**

Last updated: 2025-10-19
