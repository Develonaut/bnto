# 🍱 Approved Status Words for Bento

## Usage Guidelines

Use these sushi-themed status words consistently across:
- CLI output messages
- Log messages
- Progress indicators
- Status updates
- Error messages

**Goal:** Maintain a playful, food-themed vocabulary that makes automation feel fun and approachable.

## Status Word Categories

### Running/In-Progress Status

```go
var runningWords = []string{
    "Tasting",    // Primary - simple and clear
    "Sampling",   // Testing/trying
    "Trying",     // Attempting
    "Enjoying",   // Currently executing
    "Devouring",  // Actively consuming
    "Nibbling",   // Small/incremental progress
    "Savoring",   // Taking time, careful execution
    "Testing",    // Validation
}
```

**Recommended Usage:**
- **Tasting** - Default for workflow execution
- **Sampling** - Quick tests or dry runs
- **Savoring** - Long-running operations
- **Nibbling** - Incremental/iterative operations

### Completed/Success Status

```go
var completedWords = []string{
    "Savored",    // Primary - enjoyed completely
    "Devoured",   // Consumed successfully
    "Enjoyed",    // Completed with satisfaction
    "Relished",   // Completed with delight
    "Finished",   // Simply done
    "Consumed",   // Fully processed
    "Completed",  // Standard completion
    "Perfected",  // Flawless execution
}
```

**Recommended Usage:**
- **Savored** - Default successful completion
- **Devoured** - Fast/efficient completion
- **Perfected** - Zero errors, optimal result
- **Enjoyed** - General success message

### Failed/Error Status

```go
var failedWords = []string{
    "Spoiled",     // Primary - data/state corruption
    "Burnt",       // Over-processing, timeout
    "Dropped",     // Connection/network errors
    "Ruined",      // Catastrophic failure
    "Failed",      // Generic failure
    "Overcooked",  // Excessive retries, timeout
    "Undercooked", // Incomplete, premature termination
}
```

**Recommended Usage:**
- **Spoiled** - Data validation errors, corruption
- **Burnt** - Timeout, over-processing
- **Dropped** - Network/connection failures
- **Undercooked** - Missing requirements, incomplete setup
- **Failed** - Generic error fallback

## Examples in Context

### CLI Output
```
🍱 Tasting bento: data-pipeline
🍙 Savoring neta 'transform-data'... 67% complete
✓ 🍱 Devoured successfully in 1.2s
```

### Error Messages
```
✗ Spoiled: Invalid JSON in workflow.bento.json
✗ Burnt: Timeout after 10m
✗ Dropped: Connection to API failed
```

### Progress Updates
```
⟳ Nibbling through 100 items...
⟳ Savoring long operation (3m elapsed)
✓ Enjoyed! All items processed.
```

## Variation Guidelines

### When to Use Variations

**DO** use variations when:
- Adding personality to verbose/debug output
- Randomizing messages for repeated operations
- Matching word to specific operation type

**DON'T** use variations for:
- Error messages that need to be parsed
- API responses or structured output
- Critical system messages

### Random Selection Example

```go
// For fun, varied output in verbose mode
func getRunningWord() string {
    return runningWords[rand.Intn(len(runningWords))]
}

// For consistent, parseable output
func getStatusWord(status Status) string {
    switch status {
    case Running:
        return "Tasting"  // Always use primary
    case Completed:
        return "Savored"  // Always use primary
    case Failed:
        return "Failed"   // Clear for parsing
    }
}
```

## Word Selection Principles

1. **Food-themed first** - Prioritize sushi/cooking metaphors
2. **Clear meaning** - Status should be obvious from context
3. **Playful but professional** - Fun without being silly
4. **Culturally appropriate** - Respectful of food culture
5. **Parse-friendly** - Can be used in structured logs

## Cross-Reference

See also:
- [EMOJIS.md](./EMOJIS.md) - Approved emoji set
- [PACKAGE_NAMING.md](./PACKAGE_NAMING.md) - Naming conventions

---

**Last Updated:** 2025-10-19
