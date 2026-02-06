## 🚨 Critical (Must Pass)

### Go Quality & Build
- [ ] **`go fmt ./...`** - Code formatted
- [ ] **`golangci-lint run`** - Zero warnings
- [ ] **`go test -v ./...`** - All tests pass
- [ ] **`go test -race ./...`** - Race detector clean
- [ ] **`go build ./cmd/bento`** - Builds successfully
- [ ] **`go mod tidy`** - Dependencies clean

### Code Quality
- [ ] **No empty `interface{}`** without justification
- [ ] **All errors checked** - No ignored errors
- [ ] **Context first parameter** - Where applicable
- [ ] **Errors last return value** - Go convention
- [ ] **No redundant comments** - Remove obvious comments
- [ ] **No commented-out code** - Use git history
- [ ] **Files < 250 lines** - Break up if larger
- [ ] **Functions < 20 lines** - Refactor if larger

## 🍱 Bento Box Compliance (CRITICAL!)

- [ ] **Single Responsibility** - Each file/function does ONE thing
- [ ] **No Utility Grab Bags** - No `utils/` or `helpers/` dumping grounds
- [ ] **Clear Boundaries** - Well-defined package interfaces
- [ ] **Composable** - Small functions that work together
- [ ] **YAGNI** - No unused code, no premature features
- [ ] **Files < 250 lines** - Each file is one "compartment"
- [ ] **Functions < 20 lines** - Each function focused
- [ ] **No circular dependencies** - Clean import graph

### Package Organization
- [ ] **Single purpose packages** - One concept per package
- [ ] **No utils packages** - Organize by domain instead
- [ ] **Public API minimal** - Only export what's needed
- [ ] **Internal/ for private** - Use internal/ for package-private code

## 🧪 Testing

### Test Quality
- [ ] **Unit tests present** - All public functions tested
- [ ] **Table-driven tests** - Where appropriate
- [ ] **Edge cases covered** - Boundary conditions tested
- [ ] **Error cases tested** - Unhappy paths covered
- [ ] **Test files: `*_test.go`** - Go convention

### Test Execution
- [ ] **All tests pass** - `go test ./...`
- [ ] **Race detector clean** - `go test -race ./...`
- [ ] **Meaningful tests** - Not just for coverage

## 📝 Documentation

### Go Doc Comments
- [ ] **Package comments** - Every package has doc comment
- [ ] **Public types documented** - godoc comments present
- [ ] **Public functions documented** - godoc comments present
- [ ] **Examples for complex APIs** - Where helpful
- [ ] **No jsdocs style** - Use Go doc format

### README
- [ ] **README updated** - If API changed
- [ ] **Examples work** - Tested if provided

## 🔒 Security

- [ ] **No hardcoded secrets** - Use environment variables
- [ ] **Input validation present** - Validate untrusted input
- [ ] **Error messages safe** - Don't leak sensitive info
- [ ] **Context cancellation** - Respect context.Done()

## ⚡ Performance

- [ ] **No unnecessary allocations** - Profile if needed
- [ ] **Efficient algorithms** - Big-O considered
- [ ] **No memory leaks** - Proper cleanup
- [ ] **Context used properly** - Timeouts/cancellation work

## 📦 Module Hygiene

- [ ] **`go mod tidy` run** - No extra dependencies
- [ ] **Dependencies justified** - Each dependency has purpose
- [ ] **Standard library preferred** - Before external deps
- [ ] **No circular imports** - Clean dependency graph

## ✅ Approval Criteria

**Can approve when:**
- All critical items checked
- All Bento Box principles followed
- Tests pass (including race detector)
- Build succeeds
- No security issues

**Request changes if:**
- Any critical items unchecked
- Bento Box violations detected
- Utils package found
- Files over 250 lines
- Functions over 20 lines
- golangci-lint failures
- Test failures
- Security concerns

## 📊 Review Report Format

```
✅ Format: PASS - All code gofmt'd
✅ Lint: PASS - golangci-lint clean
✅ Tests: PASS - All 47 tests passing
✅ Race: PASS - No race conditions
✅ Build: PASS - Built successfully
✅ File Size: PASS - Largest file: 187 lines
✅ Function Size: PASS - Largest function: 18 lines
✅ Bento Box: COMPLIANT - Single responsibility maintained
✅ Security: PASS - No issues found
```

---

## For AI Agents

When using this checklist:

1. **Run all commands first:**
   ```bash
   go fmt ./...
   golangci-lint run
   go test -v -race ./...
   go build ./cmd/bento
   go mod tidy
   ```

2. **Check file/function sizes:**
   ```bash
   find pkg -name "*.go" -exec wc -l {} + | sort -rn | head -10
   ```

3. **Look for utils packages:**
   ```bash
   find pkg -type d -name "*util*"
   ```

4. **Report status** using format above

5. **Only mark complete when ALL checks pass**

---

**Remember: Bento Box compliance is MANDATORY. Code that violates these principles will be REJECTED by Karen.** 🍱
