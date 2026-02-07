# Bento Box UI - Proof of Concept
**Visual Workflow Builder with Drag & Drop Compartments**

> **2026-02-06 UPDATE:** Visual editor is now **Phase 4** in [CLOUD_DESKTOP_STRATEGY.md](CLOUD_DESKTOP_STRATEGY.md).
> Phase 1 (cloud MVP) uses a JSON code editor (Monaco/CodeMirror). This PoC concept is valid
> but deferred until after monetization justifies the investment.

**Date:** 2025-10-22
**Updated:** 2026-02-06
**Status:** Concept valid, deferred to Phase 4
**Goal:** Validate bnto box metaphor for workflow building with elegant animations
**Inspiration:** Mini Motorways aesthetic + traditional bnto box compartments

---

## Concept Overview

### The Bento Box Metaphor

Instead of a traditional node graph editor (like React Flow), create a **literal bnto box** interface:

```
┌─────────────────────────────────────┐
│     Bento Box (Workflow)            │
├─────────────────────────────────────┤
│  ┌──────┐  ┌──────┐  ┌──────┐      │
│  │ HTTP │  │ CSV  │  │Image │      │
│  │ Req  │  │Read  │  │Proc  │      │
│  └──────┘  └──────┘  └──────┘      │
│                                     │
│  ┌───────────────────────┐         │
│  │  Loop (Nested Box)    │         │
│  │  ┌──────┐  ┌──────┐  │         │
│  │  │Edit  │  │Save  │  │         │
│  │  └──────┘  └──────┘  │         │
│  └───────────────────────┘         │
└─────────────────────────────────────┘
```

### Key Visual Concepts

1. **Ingredients (Neta)** - Individual task boxes you drag in
2. **Compartments** - Sections within the bnto box
3. **Nested Boxes** - Groups/loops create sub-boxes (like multi-tier bnto boxes)
4. **Flow** - Sequential placement (left to right, top to bottom)
5. **Bobbling/Settling** - Smooth animations when dropping items

### Visual Inspiration

**Mini Motorways:**
- Cream/warm color palette (#F5E6D3, #E8D5C4, #D4C4B0)
- Smooth, satisfying animations
- Subtle shadows and depth
- Minimalist icons
- Elastic/spring animations when placing items
- Soft pastel accent colors for different node types

**Traditional Bento Box:**
- Clear compartments
- Natural wood aesthetic (in digital form)
- Organized, neat appearance
- Everything has its place

---

## Technical Stack

### Core Framework
- **Vite + React 18 + TypeScript** - Fast dev environment
- **Tailwind CSS** - Utility-first styling

### UI Components
- **shadcn/ui** - High-quality, accessible components
  - Button, Card, Dialog, Dropdown, etc.
  - Customizable with Tailwind
  - Great TypeScript support

### Drag & Drop
- **@dnd-kit** - Modern drag and drop library
  - Works great with React 18
  - Smooth animations built-in
  - Supports nested droppable areas
  - Better than react-dnd for this use case

### Animations
- **Framer Motion** - Production-ready animation library
  - Spring physics for bobbling effect
  - Layout animations (items shifting)
  - Gesture support
  - Perfect for Mini Motorways-style animations

### State Management
- **Zustand** - Lightweight state management
  - Perfect for PoC
  - Simple API
  - Works well with TypeScript

---

## PoC Features

### Phase 1: Basic Bento Box (Week 1)

**Goal:** Single-level bnto box with draggable ingredients

**Features:**
1. Ingredient Palette (sidebar)
   - List of available neta types
   - Icons and names
   - Drag from palette

2. Bento Box Canvas (main area)
   - Drop zone for ingredients
   - Auto-layout (grid or flex)
   - Visual feedback on hover

3. Ingredient Cards
   - Compact card design
   - Icon + name + key params
   - Delete button (on hover)
   - Smooth drop animation (bobble/settle)

4. Color Palette
   - Cream background (#F5E6D3)
   - Warm wood tones (#E8D5C4, #D4C4B0)
   - Soft shadows
   - Pastel accent colors per node type:
     - HTTP: Soft blue (#A8D5E2)
     - File: Soft green (#B8E2B8)
     - Transform: Soft purple (#D4B8E2)
     - Loop: Soft orange (#F4C4A0)

**Visual Example:**
```
Sidebar (Palette)        Main Canvas (Bento Box)
┌──────────────┐        ┌─────────────────────────┐
│ Ingredients  │        │    My Workflow          │
│              │        ├─────────────────────────┤
│ 🌐 HTTP      │        │ ┌──────┐  ┌──────┐     │
│ 📁 File      │ ──────>│ │ HTTP │  │ File │     │
│ 🔄 Transform │        │ │ Req  │  │ Read │     │
│ 🔁 Loop      │        │ └──────┘  └──────┘     │
│ ⚡ Parallel  │        │                         │
│              │        │   [Drop zone here]      │
└──────────────┘        └─────────────────────────┘
```

### Phase 2: Nested Boxes (Week 2)

**Goal:** Groups and loops create nested compartments

**Features:**
1. Group/Loop Containers
   - Drag a "Loop" or "Group" neta
   - Creates a nested box (compartment)
   - Can drag other neta into it
   - Visual distinction (darker border, slightly inset)

2. Nested Drop Zones
   - Can drop ingredients into nested boxes
   - Nested boxes have their own layout
   - Visual hierarchy (depth via shadows/opacity)

3. Expand/Collapse
   - Nested boxes can minimize
   - Show summary when collapsed
   - Smooth expand/collapse animation

**Visual Example:**
```
┌─────────────────────────────────────────┐
│         Product Automation              │
├─────────────────────────────────────────┤
│  ┌──────┐                               │
│  │ CSV  │                               │
│  │ Read │                               │
│  └──────┘                               │
│                                         │
│  ┌───────────────────────────────────┐ │
│  │  🔁 Loop (For Each Product)       │ │
│  │  ┌──────┐  ┌──────┐  ┌──────┐    │ │
│  │  │Edit  │  │Render│  │ Save │    │ │
│  │  │Fields│  │Image │  │ File │    │ │
│  │  └──────┘  └──────┘  └──────┘    │ │
│  └───────────────────────────────────┘ │
│                                         │
│  ┌──────┐                               │
│  │Email │                               │
│  │Send  │                               │
│  └──────┘                               │
└─────────────────────────────────────────┘
```

### Phase 3: Polish & Interactions (Week 3)

**Goal:** Smooth animations and delightful interactions

**Features:**
1. Smooth Animations
   - **Drop animation:** Spring physics, bobble and settle
   - **Drag preview:** Card lifts with shadow
   - **Reordering:** Other cards smoothly shift
   - **Delete:** Card fades out, others fill space
   - **Expand/collapse:** Smooth height animation

2. Visual Feedback
   - Drop zone highlights on hover
   - Invalid drop zones show feedback
   - Card hover effects (lift slightly)
   - Cursor changes appropriately

3. Property Editing
   - Click card to open property panel (right sidebar)
   - Inline editing for key properties
   - Smooth slide-in animation for panel

4. Mini Motorways Aesthetic
   - Subtle grain texture on background
   - Soft drop shadows (no harsh edges)
   - Rounded corners everywhere
   - Soft color transitions
   - Slight scale-up on hover

**Animation Examples (Framer Motion):**
```typescript
// Drop animation - bobble and settle
<motion.div
  initial={{ scale: 0.8, opacity: 0, y: -20 }}
  animate={{ scale: 1, opacity: 1, y: 0 }}
  transition={{
    type: "spring",
    stiffness: 500,
    damping: 30,
    mass: 1
  }}
>

// Drag preview - lift effect
<motion.div
  whileDrag={{
    scale: 1.05,
    boxShadow: "0 10px 30px rgba(0,0,0,0.15)",
    rotate: 2
  }}
>

// Reorder animation - smooth shift
<motion.div
  layout
  transition={{ type: "spring", stiffness: 500, damping: 30 }}
>
```

---

## Color Palette

### Base Colors (Mini Motorways Inspired)

```css
/* Backgrounds */
--cream-bg: #F5E6D3;        /* Main canvas background */
--warm-bg: #E8D5C4;         /* Card backgrounds */
--wood-border: #D4C4B0;     /* Borders and dividers */

/* Nested boxes */
--nested-bg: #F0DCC8;       /* Slightly darker for depth */
--nested-border: #C4B4A0;   /* Darker border */

/* Text */
--text-primary: #3E3833;    /* Dark warm brown */
--text-secondary: #6E6860;  /* Medium warm gray */

/* Node Type Colors (Soft Pastels) */
--http-blue: #A8D5E2;       /* HTTP Request */
--file-green: #B8E2B8;      /* File System */
--transform-purple: #D4B8E2; /* Transform */
--loop-orange: #F4C4A0;     /* Loop */
--parallel-yellow: #F4E4A0; /* Parallel */
--image-pink: #F4C4D4;      /* Image */
--spreadsheet-teal: #A8E2D5; /* Spreadsheet */
--shell-gray: #C4C4C4;      /* Shell Command */
--group-tan: #E2D5C4;       /* Group */
--edit-lavender: #D4C4E2;   /* Edit Fields */

/* Accents */
--shadow-soft: rgba(62, 56, 51, 0.1);
--shadow-medium: rgba(62, 56, 51, 0.15);
--success: #9BC99F;
--warning: #F4C4A0;
--error: #F4B4A0;
```

### Node Type Icon & Color Mapping

```typescript
const nodeStyles = {
  'http-request': { icon: '🌐', color: '#A8D5E2', emoji: '🌐' },
  'file-system': { icon: '📁', color: '#B8E2B8', emoji: '📁' },
  'transform': { icon: '🔄', color: '#D4B8E2', emoji: '🔄' },
  'loop': { icon: '🔁', color: '#F4C4A0', emoji: '🔁' },
  'parallel': { icon: '⚡', color: '#F4E4A0', emoji: '⚡' },
  'image': { icon: '🖼️', color: '#F4C4D4', emoji: '🖼️' },
  'spreadsheet': { icon: '📊', color: '#A8E2D5', emoji: '📊' },
  'shell-command': { icon: '⚙️', color: '#C4C4C4', emoji: '⚙️' },
  'group': { icon: '📦', color: '#E2D5C4', emoji: '📦' },
  'edit-fields': { icon: '✏️', color: '#D4C4E2', emoji: '✏️' },
}
```

---

## Project Structure

```
~/Code/bnto-box-ui-poc/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
├── index.html
├── public/
│   └── favicon.svg
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── styles/
│   │   └── globals.css
│   ├── components/
│   │   ├── ui/                 # shadcn components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   └── ...
│   │   ├── BntoBox/
│   │   │   ├── BntoBox.tsx    # Main canvas
│   │   │   ├── BntoItem.tsx   # Individual neta card
│   │   │   ├── BntoGroup.tsx  # Nested container
│   │   │   └── DropZone.tsx    # Drop target
│   │   ├── Palette/
│   │   │   ├── Palette.tsx     # Ingredient sidebar
│   │   │   └── PaletteItem.tsx # Draggable item
│   │   └── PropertyPanel/
│   │       └── PropertyPanel.tsx
│   ├── lib/
│   │   ├── utils.ts            # shadcn utils
│   │   └── nodeTypes.ts        # Node type definitions
│   ├── store/
│   │   └── workflowStore.ts    # Zustand store
│   └── types/
│       └── index.ts            # TypeScript types
└── README.md
```

---

## Implementation Plan

### Setup (30 minutes)

```bash
# Create project
cd ~/Code
npm create vite@latest bnto-box-ui-poc -- --template react-ts
cd bnto-box-ui-poc

# Install dependencies
npm install

# Install shadcn/ui
npx shadcn-ui@latest init

# Install additional packages
npm install @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities
npm install framer-motion
npm install zustand
npm install lucide-react  # Icons
```

### Week 1: Basic Bento Box (Phase 1)

**Day 1-2: Setup & Layout**
- Configure Tailwind with cream color palette
- Create basic layout (Palette sidebar + BntoBox canvas)
- Set up shadcn/ui components
- Create base components (BntoBox, Palette, BntoItem)

**Day 3-4: Drag & Drop**
- Implement @dnd-kit draggable/droppable
- Drag from Palette to BntoBox
- Drop animation with Framer Motion (bobble effect)
- Auto-layout of items in BntoBox

**Day 5: Visual Polish**
- Apply cream color palette
- Add soft shadows and borders
- Card hover effects
- Smooth transitions

**Deliverable:** Can drag ingredients into bnto box with smooth animations

### Week 2: Nested Boxes (Phase 2)

**Day 1-2: Container Components**
- Create BntoGroup component (nested box)
- Implement nested drop zones
- Visual hierarchy (shadows, borders)
- Drag neta into groups

**Day 3-4: Interactions**
- Expand/collapse nested boxes
- Reordering within boxes
- Delete items
- Layout animations

**Day 5: Polish**
- Nested box styling
- Depth indicators
- Smooth expand/collapse

**Deliverable:** Can create nested workflows with groups/loops

### Week 3: Polish & Interactions (Phase 3)

**Day 1-2: Animations**
- Perfect the drop animation (Mini Motorways feel)
- Reorder animations
- Delete animations
- Hover effects

**Day 3-4: Property Panel**
- Right sidebar for editing
- Click card to edit properties
- Inline editing for key params
- Smooth slide-in/out

**Day 5: Final Polish**
- Grain texture on background
- Perfect all animations
- Add sound effects (optional, just for demo)
- Record demo video

**Deliverable:** Polished, delightful bnto box UI PoC

---

## Key Animations (Framer Motion Config)

### Drop Animation (Bobble & Settle)
```typescript
const dropAnimation = {
  initial: {
    scale: 0.5,
    opacity: 0,
    y: -50,
    rotate: -10
  },
  animate: {
    scale: 1,
    opacity: 1,
    y: 0,
    rotate: 0
  },
  transition: {
    type: "spring",
    stiffness: 400,
    damping: 25,
    mass: 0.8
  }
}
```

### Drag Animation (Lift)
```typescript
const dragAnimation = {
  whileDrag: {
    scale: 1.08,
    rotate: 3,
    boxShadow: "0 12px 40px rgba(62, 56, 51, 0.2)",
    zIndex: 1000
  },
  transition: {
    type: "spring",
    stiffness: 500,
    damping: 30
  }
}
```

### Layout Animation (Reorder)
```typescript
<motion.div
  layout
  transition={{
    layout: {
      type: "spring",
      stiffness: 500,
      damping: 30
    }
  }}
>
```

### Hover Animation (Lift Slightly)
```typescript
const hoverAnimation = {
  whileHover: {
    scale: 1.02,
    y: -2,
    boxShadow: "0 6px 20px rgba(62, 56, 51, 0.12)"
  },
  transition: {
    type: "spring",
    stiffness: 400,
    damping: 25
  }
}
```

---

## Component Examples

### BntoItem (Ingredient Card)
```typescript
interface BntoItemProps {
  id: string
  type: string
  name: string
  color: string
  icon: string
  onDelete?: () => void
  onClick?: () => void
}

export function BntoItem({ id, type, name, color, icon, onDelete, onClick }: BntoItemProps) {
  return (
    <motion.div
      layout
      initial={{ scale: 0.5, opacity: 0, y: -50 }}
      animate={{ scale: 1, opacity: 1, y: 0 }}
      exit={{ scale: 0.8, opacity: 0 }}
      whileHover={{ scale: 1.02, y: -2 }}
      transition={{ type: "spring", stiffness: 400, damping: 25 }}
      className="relative rounded-lg p-4 cursor-pointer group"
      style={{ backgroundColor: color }}
      onClick={onClick}
    >
      <div className="flex items-center gap-3">
        <span className="text-2xl">{icon}</span>
        <div className="flex-1">
          <div className="font-medium text-warm-900">{name}</div>
          <div className="text-xs text-warm-600">{type}</div>
        </div>
        {onDelete && (
          <button
            onClick={(e) => {
              e.stopPropagation()
              onDelete()
            }}
            className="opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>
    </motion.div>
  )
}
```

### BntoGroup (Nested Container)
```typescript
interface BntoGroupProps {
  id: string
  type: 'loop' | 'group'
  children: React.ReactNode
  expanded: boolean
  onToggle: () => void
}

export function BntoGroup({ id, type, children, expanded, onToggle }: BntoGroupProps) {
  return (
    <motion.div
      layout
      className="rounded-xl border-2 border-wood-border bg-nested-bg p-4 shadow-md"
    >
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="text-xl">{type === 'loop' ? '🔁' : '📦'}</span>
          <span className="font-medium text-warm-900">
            {type === 'loop' ? 'Loop' : 'Group'}
          </span>
        </div>
        <button onClick={onToggle}>
          <ChevronDown className={`w-4 h-4 transition-transform ${expanded ? '' : '-rotate-90'}`} />
        </button>
      </div>

      <motion.div
        initial={false}
        animate={{ height: expanded ? 'auto' : 0, opacity: expanded ? 1 : 0 }}
        transition={{ type: "spring", stiffness: 400, damping: 30 }}
        className="overflow-hidden"
      >
        <div className="space-y-2">
          {children}
        </div>
      </motion.div>
    </motion.div>
  )
}
```

---

## Success Criteria

### Visual Goals
- [ ] Cream/warm color palette matches Mini Motorways aesthetic
- [ ] Smooth, satisfying drop animations (bobble and settle)
- [ ] Cards lift smoothly when dragged
- [ ] Other items smoothly reposition when dropping
- [ ] Nested boxes have clear visual hierarchy
- [ ] Soft shadows, no harsh edges
- [ ] Everything feels tactile and responsive

### Functional Goals
- [ ] Can drag ingredients from palette to bnto box
- [ ] Can drag loop/group to create nested boxes
- [ ] Can drag ingredients into nested boxes
- [ ] Can reorder items within boxes
- [ ] Can delete items
- [ ] Can expand/collapse nested boxes
- [ ] Can click item to edit properties (basic)

### Performance Goals
- [ ] Animations run at 60 FPS
- [ ] No lag when dragging
- [ ] Smooth layout animations
- [ ] Fast initial load (<1 second)

---

## Next Steps After PoC

### If PoC is Successful:

1. **Integrate into Wails Desktop**
   - Replace React Flow in Phase 3 with Bento Box UI
   - Convert from PoC to production components
   - Add full property editing
   - Add workflow validation
   - Add save/load functionality

2. **Enhanced Features**
   - Visual connectors (lines between items)
   - Conditional branches (if/else compartments)
   - Templates (pre-made bnto boxes)
   - Search/filter palette
   - Undo/redo
   - Copy/paste

3. **Polish**
   - Sound effects (satisfying clicks/drops)
   - Haptic feedback (if supported)
   - Dark mode variant
   - Accessibility improvements

### If PoC Reveals Issues:

- Fall back to React Flow approach (Phase 3 as written)
- Hybrid approach: Bento Box for simple workflows, React Flow for complex
- Iterate on metaphor (maybe different visual approach)

---

## Demo Scenarios

### Scenario 1: Simple HTTP → File Workflow
1. Drag "HTTP Request" into bnto box (bobbles and settles)
2. Drag "File Write" into bnto box (smoothly positions next to HTTP)
3. Click items to configure
4. Workflow complete!

### Scenario 2: Loop with Nested Tasks
1. Drag "CSV Read" into bnto box
2. Drag "Loop" into bnto box (creates nested container)
3. Drag "Edit Fields" into loop box (smoothly drops into nested area)
4. Drag "Image Process" into loop box (repositions with smooth animation)
5. Drag "Email Send" into main bnto box (after loop)
6. Collapse loop to see summary

### Scenario 3: Complex Nested Structure
1. Create main bnto box workflow
2. Add Group container
3. Add Parallel container inside Group
4. Add multiple tasks inside Parallel
5. Expand/collapse to navigate
6. Smooth animations throughout

---

## Questions to Answer

Through this PoC, we want to validate:

1. **Is the bnto box metaphor intuitive?**
   - Do users understand where to drag things?
   - Is nesting clear?
   - Does it feel natural?

2. **Do the animations feel good?**
   - Mini Motorways-quality smoothness?
   - Satisfying to use?
   - Not too slow or too fast?

3. **Does it scale?**
   - Works with 5 items? 20 items? 50 items?
   - Nested 3 levels deep?
   - Performance acceptable?

4. **Is it better than node graphs?**
   - For simple workflows: definitely
   - For complex workflows: maybe hybrid?
   - User preference?

5. **Can we build it in Wails?**
   - React + Wails integration
   - Performance implications
   - Maintenance burden

---

## Resources

### Inspiration
- **Mini Motorways**: https://dinopoloclub.com/games/mini-motorways/
- **Bento Box Design**: Traditional Japanese food presentation
- **Motion Design**: https://www.framer.com/motion/

### Technical Docs
- **@dnd-kit**: https://docs.dndkit.com/
- **Framer Motion**: https://www.framer.com/motion/
- **shadcn/ui**: https://ui.shadcn.com/
- **Zustand**: https://github.com/pmndrs/zustand

### Color Palette Generators
- **Coolors**: https://coolors.co/ (generate cream/warm palettes)
- **Huemint**: https://huemint.com/ (AI color palette)

---

## Timeline

**Week 1:** Basic drag & drop with smooth animations
**Week 2:** Nested boxes and hierarchy
**Week 3:** Polish and finalize

**Total:** 3 weeks to production-quality PoC

---

**Status:** Ready to implement
**Location:** `~/Code/bnto-box-ui-poc/`
**Decision Point:** After PoC, decide whether to use for Wails Phase 3 or stick with React Flow
