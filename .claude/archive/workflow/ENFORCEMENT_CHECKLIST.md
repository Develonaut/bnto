# Workflow Enforcement Checklist

## Quick Reference

All workflow requirements defined in:
- **[MANDATORY_CHECKLIST.md](./MANDATORY_CHECKLIST.md)** - Pre-work requirements
- **[REVIEW_CHECKLIST.md](./REVIEW_CHECKLIST.md)** - Karen's validation checklist
- **[BENTO_BOX_PRINCIPLE.md](../BENTO_BOX_PRINCIPLE.md)** - Core philosophy

## Key Checks

Before starting:
- [ ] Read Bento Box Principle
- [ ] Verbal confirmation given
- [ ] TodoWrite tracking active

Cannot complete without:
- [ ] All Go quality checks pass (gofmt, lint, test, build)
- [ ] Review checklist completed
- [ ] **No utils packages**
- [ ] **Files < 250 lines**
- [ ] **Functions < 20 lines**
- [ ] Karen's final approval

## Red Flags (Work Must Be Rejected)

### Bento Box Violations (Auto-Reject)
- ❌ Utils package found
- ❌ Files over 250 lines
- ❌ Functions over 20 lines
- ❌ Mixed responsibilities in one package
- ❌ Unused code present (YAGNI violation)

### Go Quality Issues
- ❌ golangci-lint failures
- ❌ Test failures
- ❌ Race conditions detected
- ❌ Build failures
- ❌ Empty `interface{}` without justification

### Process Violations
- ❌ No confirmation of reading Bento Box Principle
- ❌ No TodoWrite tracking
- ❌ Declared "complete" without Karen's approval
- ❌ Review checklist not completed

## Enforcement Actions

If workflow not followed:
1. **STOP** all work immediately
2. Direct back to Bento Box Principle
3. Require restart with proper compliance
4. Document violation

## Remember

**NO EXCEPTIONS** - The Bento Box Principle is mandatory for:
- Claude directly
- All agents
- All code changes
- All implementations

Even "quick fixes" must follow the principle.

**Karen enforces these rules with zero tolerance.** 🍱
