# 🍱 Bento Packages

This directory contains the core packages for Bento, organized using sushi/bento-themed names that reflect each package's purpose.

## Package Overview

### 🍙 neta (`pkg/neta/`)
**ネタ** - "Ingredients" or "Toppings" in sushi terminology

The foundation package containing node type definitions. Every workflow is composed of neta (nodes/ingredients) that can be combined and orchestrated.

- `Definition` - Core node structure
- `Executable` - Interface for executable nodes
- Node type implementations (HTTP, transform, conditional, loop, group)

**Dependencies**: None (foundation package)

---

### 👨‍🍳 itamae (`pkg/itamae/`)
**板前** - "Sushi Chef" - The skilled chef who prepares each piece

The orchestration engine that executes workflows. Like a sushi chef carefully preparing each piece, Itamae coordinates the execution of neta definitions.

- Workflow orchestration
- Execution coordination
- Context management
- Sequential and parallel execution

**Dependencies**: `neta`

---

### 🏪 pantry (`pkg/pantry/`)
The registry of available neta types

A thread-safe registry that stores and provides lookup for all available node types. Think of it as the pantry where all your ingredients are organized and ready to use.

- Node type registration
- Type lookup and discovery
- Thread-safe operations

**Dependencies**: `neta`

---

### 🥡 hangiri (`pkg/hangiri/`)
**重箱** - "Stacked Boxes" - Traditional Japanese stacked food containers

The storage layer for managing .bento.yaml files and execution history. Like stacked boxes holding organized meals, hangiri manages your workflow files and tracks execution records.

- .bento.yaml file parsing
- Workflow file management
- Execution history tracking
- File discovery and indexing

**Dependencies**: `neta`

---

### 🏮 omise (`pkg/omise/`)
**お店** - "Shop" - The customer-facing establishment

The Bubble Tea TUI (Terminal User Interface) - the interactive "shop" where users browse workflows, watch executions, and explore available neta types.

- Interactive workflow browser
- Execution viewer with progress
- Neta type explorer (pantry view)
- Settings and help screens
- Styled with Lip Gloss and Bubbles

**Dependencies**: `neta`, `itamae`, `pantry`, `hangiri`

---

### 🌿 kombu (`pkg/kombu/`)
**昆布** - "Kelp" - Foundational ingredient for dashi stock

A foundation package providing cross-platform path resolution utilities. Like kombu forms the base of many Japanese dishes, this package provides the foundational path resolution that other packages depend on.

- Path marker resolution ({{BENTO_HOME}}, {{GDRIVE}}, etc.)
- Cloud storage auto-detection
- Environment variable expansion
- Configuration file management
- Cross-platform path handling

**Dependencies**: None (foundation package)

---

## Dependency Graph

```
Foundation (no dependencies):
├── neta
└── kombu

Depends on foundation:
├── itamae → neta, kombu
├── pantry → neta
├── hangiri → neta
└── miso → kombu

Application layer:
└── omise → neta, itamae, pantry, hangiri, miso, kombu
```

## Package Boundaries

Each package follows the **Bento Box Principle**:

- 🍙 **Single Responsibility** - One clear purpose per package
- 🚫 **No Utility Grab Bags** - No utils/ packages
- 🔲 **Clear Boundaries** - Well-defined interfaces
- 🧩 **Composable** - Small, focused components
- ✂️ **YAGNI** - Only what's needed

## File Organization

Each package is kept small and focused:
- **Files < 250 lines** (500 max)
- **Functions < 20 lines** (30 max)
- Clear separation of concerns

## The Sushi Theme

The naming reflects the organized, compartmentalized nature of both bento boxes and our architecture:

- **Neta** = The ingredients/nodes
- **Kombu** = Foundational kelp for dashi stock
- **Itamae** = The chef who orchestrates
- **Pantry** = Where ingredients are stored and retrieved
- **hangiri** = Stacked containers for organized storage
- **Omise** = The shop where customers interact

Just as a bento box contains organized compartments with different foods, our workflow system contains organized packages with focused responsibilities. 🍱
