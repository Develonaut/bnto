# Theming & Design Tokens

The bnto theme is fully defined. Use these tokens — never hardcode colors, radii, shadows, or fonts.

---

## Font

**Geist** (sans) + **Geist Mono** (mono) — loaded via `next/font/google`, bound to `--font-sans` and `--font-mono`.

```tsx
// apps/web/app/layout.tsx
import { Geist, Geist_Mono } from "next/font/google";

const fontSans = Geist({
  subsets: ["latin"],
  variable: "--font-sans",
});

const fontMono = Geist_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
});

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className={`${fontSans.variable} ${fontMono.variable} antialiased`}>
        {children}
      </body>
    </html>
  );
}
```

**Rules:**
- Always use `font-sans` (resolves to Geist via `--font-sans`)
- Use `font-mono` (resolves to Geist Mono via `--font-mono`) for code/technical output — logs, `.bnto.json` previews, node type labels
- Never load Geist via `<link>` or `@import` — `next/font` handles it with zero layout shift

---

## Color Tokens

All colors use OKLCH. Reference by semantic token, never by raw value.

### Light Mode

| Token | Value | Use |
|-------|-------|-----|
| `--background` | `oklch(0.9899 0.0164 95.22)` | Warm cream — page backgrounds |
| `--foreground` | `oklch(0.2628 0.0204 31.40)` | Warm dark brown — body text |
| `--card` | `oklch(1.0000 0 0)` | Pure white — card surfaces |
| `--card-foreground` | Same as foreground | Text on cards |
| `--primary` | `oklch(0.6751 0.1788 35.19)` | Terracotta — primary actions, CTAs |
| `--primary-foreground` | `oklch(1.0000 0 0)` | White — text on primary |
| `--secondary` | `oklch(0.8657 0.0705 189.39)` | Soft teal — secondary actions |
| `--secondary-foreground` | `oklch(0.4372 0.0600 187.70)` | Deep teal — text on secondary |
| `--muted` | `oklch(0.9491 0.0133 95.19)` | Warm off-white — muted surfaces |
| `--muted-foreground` | `oklch(0.5452 0.0251 31.20)` | Warm gray — secondary text, placeholders |
| `--accent` | `oklch(0.8885 0.1338 91.06)` | Golden yellow — accent, highlights |
| `--accent-foreground` | Same as foreground | Text on accent |
| `--destructive` | `oklch(0.6356 0.2082 25.38)` | Warm red — errors, destructive actions |
| `--border` | `oklch(0.8976 0.0168 95.25)` | Warm light — borders |
| `--input` | `oklch(0.9320 0.0111 95.17)` | Slightly deeper cream — input backgrounds |
| `--ring` | Same as primary | Focus rings |

### Dark Mode

Dark mode uses a cool slate base (not black) — the warm palette carries through in primary, accent, and foreground.

| Token | Value | Note |
|-------|-------|------|
| `--background` | `oklch(0.2612 0.0154 264.26)` | Cool dark slate |
| `--card` | `oklch(0.3103 0.0197 264.23)` | Slightly lighter slate |
| `--primary` | Same as light | Terracotta unchanged |
| `--accent` | `oklch(0.8197 0.1575 88.39)` | Golden, slightly richer |
| `--muted` | `oklch(0.3576 0.0238 264.21)` | Dark slate surface |

---

## Tailwind Usage

```tsx
// Backgrounds
className="bg-background"        // Warm cream page base
className="bg-card"              // White card surface
className="bg-muted"             // Subtle section backgrounds
className="bg-primary"           // Terracotta CTA
className="bg-accent"            // Golden highlight

// Text
className="text-foreground"      // Body text
className="text-muted-foreground" // Secondary / placeholder text
className="text-primary"         // Terracotta text links/labels

// Borders
className="border-border"        // Standard border
className="ring-ring"            // Focus ring

// Inputs
className="bg-input"             // Input field background
```

---

## Border Radius

Base radius is `1.25rem` (20px) — generously rounded, warm and friendly.

| Token | Value | Use |
|-------|-------|-----|
| `rounded-sm` | `calc(1.25rem - 4px)` = `~1rem` | Tight elements — badges, tags |
| `rounded-md` | `calc(1.25rem - 2px)` = `~1.125rem` | Inputs, small buttons |
| `rounded-lg` | `1.25rem` | Cards, panels, standard containers |
| `rounded-xl` | `calc(1.25rem + 4px)` = `~1.5rem` | Feature cards, hero elements |

**Rule:** Default to `rounded-lg`. Go rounder (`rounded-xl`) for prominent surfaces, tighter (`rounded-sm`) for small inline elements. Never use `rounded-none` or sharp corners in brand UI.

---

## Shadows

Shadows use a warm-tinted base color (`hsl(10, 20%, 15%)`) in light mode — they never look cold or gray.

```tsx
className="shadow-sm"   // Subtle lift — inputs, small cards
className="shadow"      // Standard card elevation
className="shadow-md"   // Modals, dropdowns
className="shadow-lg"   // Popovers, overlays
className="shadow-xl"   // Featured elements, hero cards
```

Dark mode shadows use pure black at higher opacity — the system handles this automatically via CSS variables.

---

## Typography Scale

Geist is a clean, modern sans-serif designed by Vercel. It reads sharp and professional at all sizes.

```tsx
// Display / Hero
className="text-4xl font-bold tracking-tight"

// Page headings
className="text-2xl font-semibold"

// Section headings  
className="text-lg font-semibold"

// Body
className="text-base"   // tracking-normal (0.02em) applied globally via body styles

// Small / labels
className="text-sm text-muted-foreground"

// Technical / metadata (mono)
className="text-xs font-mono tracking-wider text-muted-foreground uppercase"
```

**Letter spacing:** `0.02em` applied globally. Don't override unless intentional.

---

## Chart Colors

Five chart tokens for data visualization — all harmonious with the warm palette:

| Token | Color | Character |
|-------|-------|-----------|
| `--chart-1` | Terracotta | Primary data series |
| `--chart-2` | Soft blue | Secondary series |
| `--chart-3` | Golden | Tertiary / accent series |
| `--chart-4` | Sage green | Quaternary |
| `--chart-5` | Deep terracotta | Fifth series |

---

## Globals CSS Location

```
apps/web/app/globals.css    # Full token definitions, dark mode, @theme inline block
```

The `@theme inline` block maps CSS variables to Tailwind's color/shadow/radius system. Do not duplicate token values — always reference via CSS variable or Tailwind utility.

---

## Rules for Claude Code

- **Never hardcode a color value** — always use semantic tokens (`bg-primary`, `text-muted-foreground`, etc.)
- **Never hardcode a radius** — always use `rounded-{sm|md|lg|xl}`
- **Never use `font-['Geist']`** — use `font-sans` (the variable resolves it)
- **Never add a `<link>` for Geist** — `next/font` is already handling it in `layout.tsx`
- **Dark mode is automatic** — tokens swap via `.dark` class, no manual dark: overrides needed for semantic tokens
- **Shadows are warm** — use the shadow scale, don't write custom `box-shadow` values
- **When in doubt, round more** — the brand is warm and friendly, not sharp
