# 🚨 MANDATORY PRE-WORK CHECKLIST 🚨

## Before Starting ANY Work

1. **READ** the [Bento Box Principle](../BENTO_BOX_PRINCIPLE.md) - Our core philosophy
2. **CONFIRM**: State "I understand the Bento Box Principle and will follow it"
3. **USE TodoWrite** to track workflow steps

## The Bento Box Principle Defines

Our code quality foundation - 5 key principles:
1. **Single Responsibility** - One thing per file/function
2. **No Utility Grab Bags** - No `utils/` packages
3. **Clear Boundaries** - Clean package interfaces
4. **Composable** - Small functions working together
5. **YAGNI** - No unused code

## Critical Quality Gates

- **No utils packages** - INSTANT REJECTION
- **Files < 250 lines** - Must be refactored if exceeded
- **Functions < 20 lines** - Must be simplified if exceeded
- **All checks pass** - `go fmt && golangci-lint && go test -race && go build`
- **Document Level Documentation** - Every package/file must have clear and concise description at the top describing its purpose.

## Go-Specific Requirements

- `context.Context` first parameter
- `error` as last return value
- No empty `interface{}` without justification
- All errors checked
- Standard library before external deps

**This is NOT optional. ALL work follows the Bento Box Principle.**
