/**
 * Generate CSS + Rust palette from theme/palette.toml.
 *
 * Reads semantic color tokens from the shared TOML config and produces:
 *   - apps/web/app/theme-colors.generated.css   (OKLCH values)
 *   - engine/crates/bnto/src/tui/palette.rs      (RGB values)
 *
 * Validates OKLCH → RGB conversion matches declared RGB (delta-E ≤ 3.0),
 * and ensures every theme variant has the same set of token names.
 *
 * Flags:
 *   --check          Diff against committed files, exit non-zero if stale
 *   --validate-only  Run validation without writing files
 *
 * Usage:
 *   npx tsx scripts/generate-theme.ts
 *   npx tsx scripts/generate-theme.ts --check
 *   npx tsx scripts/generate-theme.ts --validate-only
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "fs";
import { dirname, resolve } from "path";

import { differenceEuclidean, formatRgb, oklch as toOklch, parse } from "culori";
import { parse as parseTOML } from "smol-toml";

const ROOT = resolve(__dirname, "..");
const TOML_PATH = resolve(ROOT, "theme/palette.toml");
const CSS_OUT = resolve(ROOT, "apps/web/app/theme-colors.generated.css");
const RUST_OUT = resolve(ROOT, "engine/crates/bnto/src/tui/palette.rs");

const CHECK_MODE = process.argv.includes("--check");
const VALIDATE_ONLY = process.argv.includes("--validate-only");

// Delta-E tolerance for OKLCH → sRGB validation.
const DELTA_E_TOLERANCE = 3.0;

// Theme variant → CSS selector mapping.
const THEME_SELECTORS: Record<string, string> = {
  light: ":root",
  tokyo: ".tokyo",
  monaco: ".monaco",
};

// Ordered list — light first (default), then dark variants.
const THEME_ORDER = ["light", "tokyo", "monaco"];

// ── Types ──────────────────────────────────────────────────────────────

interface ColorToken {
  name: string;
  oklch: string;
  rgb: [number, number, number];
}

interface ThemeTokens {
  variant: string;
  tokens: ColorToken[];
}

// ── Parse ──────────────────────────────────────────────────────────────

function parseToml(): ThemeTokens[] {
  const raw = readFileSync(TOML_PATH, "utf-8");
  const doc = parseTOML(raw) as Record<string, unknown>;
  const themes: ThemeTokens[] = [];

  for (const variant of THEME_ORDER) {
    const section = doc[variant] as Record<string, Record<string, unknown>> | undefined;
    if (!section) {
      throw new Error(`Missing theme variant: [${variant}] in palette.toml`);
    }

    const tokens: ColorToken[] = [];
    for (const [name, entry] of Object.entries(section)) {
      const oklchVal = entry.oklch as string | undefined;
      const rgbVal = entry.rgb as number[] | undefined;
      if (!oklchVal || !rgbVal || rgbVal.length !== 3) {
        throw new Error(
          `Invalid token [${variant}.${name}]: needs oklch (string) and rgb ([r,g,b])`,
        );
      }
      tokens.push({
        name,
        oklch: oklchVal,
        rgb: rgbVal as [number, number, number],
      });
    }

    themes.push({ variant, tokens });
  }

  return themes;
}

// ── Validate ───────────────────────────────────────────────────────────

function validate(themes: ThemeTokens[]): void {
  // 1. Every theme variant has the same set of token names.
  const tokenSets = themes.map((t) => new Set(t.tokens.map((tok) => tok.name)));
  const reference = tokenSets[0];
  for (let i = 1; i < themes.length; i++) {
    const current = tokenSets[i];
    for (const name of reference) {
      if (!current.has(name)) {
        throw new Error(
          `Token "${name}" exists in [${themes[0].variant}] but missing in [${themes[i].variant}]`,
        );
      }
    }
    for (const name of current) {
      if (!reference.has(name)) {
        throw new Error(
          `Token "${name}" exists in [${themes[i].variant}] but missing in [${themes[0].variant}]`,
        );
      }
    }
  }

  // 2. OKLCH → sRGB matches declared RGB within tolerance.
  const diffEuclid = differenceEuclidean("oklch");
  for (const theme of themes) {
    for (const token of theme.tokens) {
      const [l, c, h] = token.oklch.split(" ").map(Number);
      const oklchColor = toOklch({ mode: "oklch", l, c, h: h || undefined });
      const rgbStr = formatRgb(oklchColor);
      const match = rgbStr.match(/rgb\((\d+), (\d+), (\d+)\)/);
      if (!match) {
        throw new Error(
          `[${theme.variant}.${token.name}] OKLCH "${token.oklch}" produced unparseable RGB: ${rgbStr}`,
        );
      }

      const computed = [Number(match[1]), Number(match[2]), Number(match[3])] as const;
      const declared = token.rgb;

      // Compare in OKLCH space for perceptual accuracy.
      const declaredColor = parse(`rgb(${declared[0]}, ${declared[1]}, ${declared[2]})`);
      const computedColor = parse(`rgb(${computed[0]}, ${computed[1]}, ${computed[2]})`);
      if (declaredColor && computedColor) {
        const deltaE = diffEuclid(declaredColor, computedColor);
        if (deltaE > DELTA_E_TOLERANCE) {
          throw new Error(
            `[${theme.variant}.${token.name}] OKLCH→RGB mismatch: ` +
              `declared rgb(${declared.join(",")}) vs computed rgb(${computed.join(",")}), ` +
              `delta-E=${deltaE.toFixed(2)} (max ${DELTA_E_TOLERANCE})`,
          );
        }
      }
    }
  }
}

// ── Generate CSS ───────────────────────────────────────────────────────

function generateCSS(themes: ThemeTokens[]): string {
  const lines: string[] = ["/* AUTO-GENERATED from theme/palette.toml — DO NOT EDIT */", ""];

  for (const theme of themes) {
    const selector = THEME_SELECTORS[theme.variant];
    lines.push(`${selector} {`);
    for (const token of theme.tokens) {
      const cssName = token.name.replace(/_/g, "-");
      lines.push(`  --${cssName}: oklch(${token.oklch});`);
    }
    lines.push("}", "");
  }

  return lines.join("\n");
}

// ── Generate Rust ──────────────────────────────────────────────────────

function toRustConst(name: string): string {
  return name.toUpperCase();
}

function generateRustModule(tokens: ColorToken[], indent: string): string {
  const lines: string[] = [];
  for (const token of tokens) {
    const constName = toRustConst(token.name);
    const [r, g, b] = token.rgb;
    lines.push(`${indent}pub const ${constName}: Color = Color::Rgb(${r}, ${g}, ${b});`);
  }
  return lines.join("\n");
}

function generateRust(themes: ThemeTokens[]): string {
  const lines: string[] = [
    "//! AUTO-GENERATED from theme/palette.toml — DO NOT EDIT.",
    "//!",
    "//! Semantic color constants for the TUI. Light-theme constants",
    "//! are top-level; dark/sunset themes are child modules.",
    "",
    "#![allow(dead_code)]",
    "",
    "use ratatui::style::Color;",
    "",
  ];

  // Light theme — top-level constants.
  const lightTheme = themes.find((t) => t.variant === "light");
  if (!lightTheme) throw new Error("Missing light theme");
  lines.push(generateRustModule(lightTheme.tokens, ""));

  // Dark/sunset themes — child modules.
  for (const theme of themes) {
    if (theme.variant === "light") continue;
    lines.push("");
    lines.push(`pub mod ${theme.variant} {`);
    lines.push("    use ratatui::style::Color;");
    lines.push("");
    lines.push(generateRustModule(theme.tokens, "    "));
    lines.push("}");
  }

  lines.push("");
  return lines.join("\n");
}

// ── Write if changed ───────────────────────────────────────────────────

function writeIfChanged(path: string, content: string): boolean {
  if (existsSync(path)) {
    const existing = readFileSync(path, "utf-8");
    if (existing === content) return false;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content, "utf-8");
  return true;
}

// ── Main ───────────────────────────────────────────────────────────────

function main(): void {
  const themes = parseToml();
  validate(themes);
  console.log(`Validated ${themes.length} theme variants, all tokens consistent.`);

  if (VALIDATE_ONLY) {
    console.log("Validation passed (--validate-only, no files written).");
    return;
  }

  const css = generateCSS(themes);
  const rust = generateRust(themes);

  if (CHECK_MODE) {
    let stale = false;

    if (!existsSync(CSS_OUT) || readFileSync(CSS_OUT, "utf-8") !== css) {
      console.error(`STALE: ${CSS_OUT}`);
      stale = true;
    }
    if (!existsSync(RUST_OUT) || readFileSync(RUST_OUT, "utf-8") !== rust) {
      console.error(`STALE: ${RUST_OUT}`);
      stale = true;
    }

    if (stale) {
      console.error("Generated theme files are out of date. Run: task theme:generate");
      process.exit(1);
    }

    console.log("Generated theme files are up to date.");
    return;
  }

  let written = 0;
  let unchanged = 0;

  if (writeIfChanged(CSS_OUT, css)) {
    written++;
    console.log(`Wrote: ${CSS_OUT}`);
  } else {
    unchanged++;
  }

  if (writeIfChanged(RUST_OUT, rust)) {
    written++;
    console.log(`Wrote: ${RUST_OUT}`);
  } else {
    unchanged++;
  }

  console.log(`Done. ${written} written, ${unchanged} unchanged.`);
}

main();
