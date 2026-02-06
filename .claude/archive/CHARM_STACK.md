# 🍱 Charm CLI Stack for Bento

**Purpose:** Define how Charm libraries integrate with Bento's architecture
**Status:** Active - Phase 7 (CLI) complete, Phase 8+ (TUI) planned
**Last Updated:** 2025-10-19

---

## Overview

Bento uses the [Charm CLI toolkit](https://charm.sh/) for beautiful terminal interfaces. This document defines which Charm libraries we use, how they map to our architecture, and the implementation timeline.

**Philosophy:** Start with excellent CLI experience NOW, evolve to full TUI later.

---

## Charm Libraries

### Currently Used (Phase 7)

| Library | Version | Purpose | Bento Package |
|---------|---------|---------|---------------|
| [lipgloss](https://github.com/charmbracelet/lipgloss) | latest | Styling, colors, layouts | `pkg/miso/` |
| [log](https://github.com/charmbracelet/log) | latest | Beautiful structured logging | `pkg/shoyu/` |
| [bubbletea](https://github.com/charmbracelet/bubbletea) | latest | TUI framework (daemon-combo) | `pkg/miso/` |

### Future Use (Phase 9+)

| Library | Purpose | Planned For |
|---------|---------|-------------|
| [bubbles](https://github.com/charmbracelet/bubbles) | TUI components (lists, tables, spinners, etc.) | Full TUI mode |
| [huh](https://github.com/charmbracelet/huh) | Forms and interactive inputs | Workflow creation wizard |
| [wish](https://github.com/charmbracelet/wish) | SSH server framework | Remote workflow execution (maybe) |

---

## Architecture Mapping

### Package: `pkg/miso/` (味噌 - Miso - "Seasoning")

**Purpose:** Terminal output "seasoning" - makes everything look delicious!

**Responsibilities:**
- Theme system (7 sushi-themed color variants)
- Progress display (daemon-combo pattern with Bubbletea)
- Step sequence rendering (with status words + emojis)
- Lipgloss styles (colors, formatting, boxes)

**Charm Libraries Used:**
- `lipgloss` - All styling and theming
- `bubbletea` - Progress display model

**Files:**
```
pkg/miso/
├── config.go       # Theme persistence (~/.bento/theme)
├── manager.go      # Theme switching (SetTheme, GetCurrentTheme)
├── variants.go     # 7 color palettes (Nasu, Wasabi, Toro, etc.)
├── theme.go        # Lipgloss styles + semantic colors
├── sequence.go     # Step sequence renderer
├── spinner.go      # Animated spinner
├── progress.go     # Bubbletea progress model (daemon-combo)
└── *_test.go
```

---

### Package: `pkg/shoyu/` (醤油 - Shoyu - "Soy Sauce")

**Purpose:** Structured logging - the essential "sauce" for debugging

**Responsibilities:**
- Wrap Go's `log/slog` for structured logging
- Optionally use `charm/log` for beautiful file output
- Stream output for long-running processes
- Context-aware logging

**Charm Libraries Used:**
- `charm/log` - Beautiful log formatting (optional, via `UseCharm: true`)

**Integration:**
```go
// Traditional slog output (default)
logger := shoyu.New(shoyu.Config{
    Level: shoyu.LevelInfo,
})

// Beautiful charm/log output (opt-in)
logger := shoyu.New(shoyu.Config{
    Level:    shoyu.LevelInfo,
    UseCharm: true,  // Enables charm/log formatting
})
```

---

### Package: `cmd/bento/` (CLI Commands)

**Purpose:** User-facing CLI commands

**Charm Integration:**
- All commands use `miso.Progress` for visual feedback
- `bento pour` uses `charm/log` for formatted log tailing
- Future: `bento theme` for switching color variants

**Commands Using Progress Display:**
1. **`bento run`** - Shows node execution steps with status words
2. **`bento validate`** - Shows validation steps
3. **`bento new`** - Shows template creation progress
4. **`bento list`** - Shows directory scanning
5. **`bento pour`** - Tails logs with charm/log formatting

---

## Theme System

### Sushi-Themed Color Variants

Users can choose from 7 beautiful themes (persisted to `~/.bento/theme`):

| Variant | Primary Color | Inspiration | Default |
|---------|---------------|-------------|---------|
| **Nasu** | Purple (#BD93F9) | 🍆 Eggplant sushi | TUI default |
| **Wasabi** | Green (#50FA7B) | Spicy green wasabi | |
| **Toro** | Pink (#FF79C6) | 🐟 Fatty tuna | |
| **Tamago** | Yellow (#F1FA8C) | 🥚 Egg sushi | |
| **Tonkotsu** | Red (#f87359) | 🐟 Tuna | **CLI default** |
| **Saba** | Cyan (#8BE9FD) | 🐟 Mackerel | |
| **Ika** | White (#F8F8F2) | 🦑 Squid | |

### Semantic Colors

All themes define these semantic colors:

```go
type Palette struct {
    Primary   lipgloss.Color  // Main theme color (brand)
    Secondary lipgloss.Color  // Accent color
    Success   lipgloss.Color  // Success states (green)
    Error     lipgloss.Color  // Error states (red)
    Warning   lipgloss.Color  // Warning states (yellow)
    Text      lipgloss.Color  // Primary text color
    Muted     lipgloss.Color  // Subtle/secondary text
}
```

### Theme Persistence

Themes are saved to: `~/.bento/theme`

```bash
# Switch theme (future command)
bento theme set wasabi

# View current theme
bento theme

# List all themes
bento theme list
```

---

## Progress Display Pattern (Daemon-Combo)

### Architecture

Based on Bubbletea's [daemon-combo example](https://github.com/charmbracelet/bubbletea/blob/main/examples/tui-daemon-combo/main.go):

```
┌─────────────────┐         ┌──────────────────┐
│ Foreground      │         │ Background       │
│ (Bubbletea)     │◄────────│ (Actual Work)    │
│                 │ Channel │                  │
│ • Renders UI    │         │ • Loads bento    │
│ • Shows progress│         │ • Executes neta  │
│ • Handles input │         │ • Sends updates  │
└─────────────────┘         └──────────────────┘
```

### Code Pattern

**Every command follows this pattern:**

```go
func runCommand(cmd *cobra.Command, args []string) error {
    // 1. Create progress display
    prog := miso.NewProgress()

    // 2. Run actual work in background
    resultChan := make(chan error)
    go func() {
        resultChan <- doActualWork(prog)
    }()

    // 3. Display progress in foreground (blocks until done)
    if err := prog.Run(); err != nil {
        return err
    }

    // 4. Get result from background
    return <-resultChan
}

func doActualWork(prog *miso.Progress) error {
    // Update progress as work happens
    prog.UpdateStep("Loading", miso.StepRunning)
    // ... do work ...
    prog.UpdateStep("Loading", miso.StepCompleted)

    prog.Done() // Signal completion
    return nil
}
```

---

## Status Words Integration

### Source of Truth

Status words are defined in [STATUS_WORDS.md](.claude/STATUS_WORDS.md):

- **Running:** Tasting, Sampling, Savoring, Nibbling, etc.
- **Completed:** Savored, Devoured, Enjoyed, Perfected, etc.
- **Failed:** Spoiled, Burnt, Dropped, Ruined, etc.

### Deterministic Selection

Uses FNV hash to ensure same step always gets same word:

```go
func getStatusLabel(status StepStatus, stepName string) string {
    h := fnv.New32a()
    h.Write([]byte(stepName))
    hash := h.Sum32()

    switch status {
    case StepRunning:
        return runningWords[hash % len(runningWords)]
    case StepCompleted:
        return completedWords[hash % len(completedWords)]
    // ...
    }
}
```

**Why deterministic?**
- Same step always shows same word
- Consistent across runs
- Builds muscle memory for users
- Looks intentional, not random

---

## Implementation Phases

### Phase 7: CLI with Progress (Current)

**Status:** ✅ Complete

- [x] Install Charm dependencies
- [x] Port theme system from archive
- [x] Create `pkg/miso/` package
- [x] Implement progress display (daemon-combo)
- [x] Update all commands with progress
- [x] Create `bento pour` for log tailing
- [x] Wrap shoyu with charm/log

**Result:** Beautiful CLI with themed progress display

---

### Phase 8: Real-World Bentos + Streaming (Next)

**Status:** 📋 Planned

Focus: Create practical workflows, test streaming

- [ ] Create example bentos (image processing, data pipelines)
- [ ] Implement streaming progress for shell-command neta
- [ ] Test with real automation scenarios
- [ ] Polish progress display based on real usage

**Charm Usage:** Same as Phase 7 (progress display)

---

### Phase 9: Full TUI Mode (Future)

**Status:** 💭 Concept

Interactive TUI for creating and managing workflows:

```bash
bento tui               # Launch full TUI
bento tui workflow.bento.json  # Edit existing workflow
```

**New Charm Libraries:**
- `bubbles` - Lists, tables, text inputs, etc.
- `huh` - Forms for creating new neta
- Possibly `wish` - SSH server for remote access

**Features:**
- Visual workflow editor (inspired by Atomiton's React Flow editor)
- Live execution monitoring
- Log viewer
- Theme switcher
- Workflow library browser

---

## File Output vs Terminal Output

### Two Separate Concerns

| Output | Technology | Purpose |
|--------|-----------|---------|
| **File Logs** | `charm/log` via shoyu | Structured logging for debugging |
| **Terminal** | `bubbletea` + `lipgloss` via miso | Real-time progress display |

### Why Separate?

- **Files need structure** - grep-able, parseable, persistent
- **Terminal needs beauty** - colors, animations, interactivity
- **Different audiences** - files for devs, terminal for users

### Example Flow

```
User runs: bento run workflow.bento.json

Terminal (miso):           File ~/.bento/logs/bento.log (shoyu+charm/log):
┌─────────────────┐       ┌──────────────────────────────────────┐
│ Running...      │       │ time=2025-10-19T10:30:00.000         │
│                 │       │ level=INFO                           │
│ Executing       │       │ msg="Starting bento execution"       │
│    Load CSV...  │       │ bento_id=workflow                    │
│                 │       │                                      │
│ ✓ Devoured      │       │ time=2025-10-19T10:30:01.234         │
│   Load CSV      │       │ level=INFO                           │
│   (1.2s)        │       │ msg="Neta completed"                 │
│                 │       │ neta_id=load-csv                     │
│ 🍙 Nibbling     │       │ neta_type=spreadsheet                │
│    Transform... │       │ duration=1234ms                      │
└─────────────────┘       └──────────────────────────────────────┘
```

---

## Testing Strategy

### Unit Tests

Each miso component has tests:
- `theme_test.go` - Theme loading and switching
- `sequence_test.go` - Step rendering
- `progress_test.go` - Bubbletea model logic

### Integration Tests

- Test progress display with actual bentos
- Verify theme persistence across runs
- Test log tailing with `bento pour`

### Visual Tests

- Manually test each theme variant
- Verify status word variety
- Check emoji assignment
- Test nested bento indentation

---

## Performance Considerations

### Lipgloss Caching

Lipgloss caches style calculations - no performance concern for CLI usage.

### Bubbletea Updates

Updates happen ~60 FPS, but only when visible (spinner animation).

### Theme Loading

Theme loaded once at startup (init function) - no runtime overhead.

---

## Future Enhancements

### Possible Additions

1. **Custom Themes** - User-defined color palettes
2. **Animation Speed** - Control spinner/progress speed
3. **Compact Mode** - Minimal output for CI/CD
4. **JSON Output** - Machine-readable progress
5. **Remote TUI** - SSH into running workflows (via `wish`)

---

## Dependencies

### Current (Phase 7)

```go
require (
    github.com/charmbracelet/lipgloss latest
    github.com/charmbracelet/log latest
    github.com/charmbracelet/bubbletea latest
)
```

### Future (Phase 9+)

```go
require (
    github.com/charmbracelet/bubbles latest
    github.com/charmbracelet/huh latest
    github.com/charmbracelet/wish latest // maybe
)
```

---

## Cross-References

- [STATUS_WORDS.md](./STATUS_WORDS.md) - Approved status words
- [EMOJIS.md](./EMOJIS.md) - Approved emojis
- [BENTO_BOX_PRINCIPLE.md](./BENTO_BOX_PRINCIPLE.md) - Code organization philosophy
- [PACKAGE_NAMING.md](./PACKAGE_NAMING.md) - Package naming conventions

---

## Resources

- [Charm.sh](https://charm.sh/) - Main Charm website
- [Lip Gloss Tutorial](https://github.com/charmbracelet/lipgloss/tree/master/examples)
- [Bubbletea Tutorial](https://github.com/charmbracelet/bubbletea/tree/main/tutorials)
- [Daemon-Combo Example](https://github.com/charmbracelet/bubbletea/blob/main/examples/tui-daemon-combo/main.go)

---

**Last Updated:** 2025-10-19
**Status:** Phase 7 complete, Phase 8 in progress
