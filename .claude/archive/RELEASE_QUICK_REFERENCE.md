# 🚀 Quick Release Reference Card

**Print this or keep it handy for every release!**

---

## 🎯 When to Release

| Type | Increment | When |
|------|-----------|------|
| **PATCH** (0.1.X) | Bug fixes | Small fixes, no new features |
| **MINOR** (0.X.0) | New features | Phase complete, new capabilities |
| **MAJOR** (X.0.0) | Breaking changes | API stable, production-ready |

---

## ✅ Pre-Flight Check

```bash
# Run before every release:
go fmt ./... && \
golangci-lint run ./... && \
go test -race ./... && \
git status
```

**All must pass!** ✅

---

## 📋 5-Step Release Process

### 1️⃣ Update CHANGELOG.md

```markdown
## [Unreleased]

(empty)

## [0.2.0] - 2025-10-20  👈 Add this section

### Added
- Feature descriptions from [Unreleased]
```

### 2️⃣ Commit Changelog

```bash
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"
```

### 3️⃣ Create Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0 - Brief description"
```

### 4️⃣ Push Everything

```bash
git push origin main
git push origin v0.2.0
```

### 5️⃣ Verify

```bash
git tag                     # See it locally
git ls-remote --tags origin # See it on GitHub
```

**Done!** 🎉

---

## 🎯 Quick Commands

```bash
# View current version
bnto version

# View all releases
git tag

# Create patch release
git tag -a v0.1.1 -m "Release v0.1.1 - Fix bug"

# Create minor release
git tag -a v0.2.0 -m "Release v0.2.0 - New features"

# Push specific tag
git push origin v0.2.0

# Push all tags
git push --tags

# Delete local tag (if mistake)
git tag -d v0.2.0

# View tag details
git show v0.2.0
```

---

## 📝 Changelog Categories

```markdown
### Added       - New features
### Changed     - Changes to existing functionality
### Deprecated  - Soon-to-be removed features
### Removed     - Removed features
### Fixed       - Bug fixes
### Security    - Security fixes
```

---

## 🚨 Common Mistakes

| Mistake | Fix |
|---------|-----|
| ❌ Forgot CHANGELOG.md | Update it, create new patch version |
| ❌ Wrong version number | Don't delete! Create new version |
| ❌ Forgot to push tag | `git push origin v0.2.0` |
| ❌ Uncommitted changes | Commit everything before tagging |
| ❌ Tests failing | Fix tests before releasing |

---

## 💡 Version Number Guide

```
v0.1.0 → v0.1.1  = Bug fix (patch)
v0.1.1 → v0.2.0  = New feature (minor)
v0.9.0 → v1.0.0  = Stable API (major)

Pre-1.0.0: Breaking changes OK in MINOR
Post-1.0.0: Breaking changes require MAJOR
```

---

## 📱 One-Liner Release

```bash
# For experienced users (after updating CHANGELOG.md):
git add CHANGELOG.md && \
git commit -m "chore: Release v0.2.0" && \
git tag -a v0.2.0 -m "Release v0.2.0 - Description" && \
git push origin main && \
git push origin v0.2.0
```

---

## 🎓 Example: Releasing v0.2.0

```bash
# 1. Update CHANGELOG.md (in your editor)
vim CHANGELOG.md

# 2. Commit it
git add CHANGELOG.md
git commit -m "chore: Release v0.2.0"

# 3. Tag it
git tag -a v0.2.0 -m "Release v0.2.0 - Phase 8 complete"

# 4. Push it
git push origin main
git push origin v0.2.0

# 5. Check it
git tag | grep v0.2.0
```

**Done!** Users can now install with:
```bash
go install github.com/Develonaut/bnto/cmd/bnto@v0.2.0
```

---

## 🆘 Emergency: Wrong Tag

**Don't panic!**

**Option 1 (Recommended):** Create new patch version
```bash
git tag -a v0.2.1 -m "Corrected release"
git push origin v0.2.1
```

**Option 2 (If not published):** Delete and recreate
```bash
# Local only (before push)
git tag -d v0.2.0
git tag -a v0.2.0 -m "Correct message"
```

**Option 3 (Already pushed):** Contact team, create patch
```bash
# Never delete published tags!
# Fix issue and release v0.2.1
```

---

## 📚 Full Documentation

For detailed guide, see:
- `.claude/RELEASE_CHECKLIST.md` - Complete checklist
- `.claude/VERSIONING.md` - Versioning guide
- `CHANGELOG.md` - Project changelog

---

**🎯 Pro Tip:** Follow this card every time. Consistency is key!

**🔖 Bookmark this:** `bnto/.claude/RELEASE_QUICK_REFERENCE.md`
