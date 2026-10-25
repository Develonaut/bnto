/**
 * Tests for brokenLinks. Uses node:test (no test runner dependency).
 * Run: pnpm exec tsx --test scripts/docs/brokenLinks.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { brokenLinks } from "./brokenLinks";

const repo =
  (...files: string[]) =>
  (path: string) =>
    files.includes(path);

describe("brokenLinks", () => {
  it("resolves a link from the folder of the file it's in", () => {
    const exists = repo(".claude/rules/gotchas.md");
    assert.deepEqual(brokenLinks(".claude/PLAN.md", "[g](rules/gotchas.md)", exists), []);
    assert.deepEqual(
      brokenLinks(".claude/skills/pickup/SKILL.md", "[g](../../rules/gotchas.md)", exists),
      [],
    );
  });

  it("names the file, line and target of a link that doesn't resolve", () => {
    const source = "# Plan\n\nSee [pricing](strategy/pricing-model.md).";
    assert.deepEqual(brokenLinks(".claude/skills/groom/SKILL.md", source, repo()), [
      { file: ".claude/skills/groom/SKILL.md", line: 3, target: "strategy/pricing-model.md" },
    ]);
  });

  it("flags a `.claude/` link outside CLAUDE.md, which GitHub resolves from the file's folder", () => {
    const exists = repo(".claude/strategy/io-nodes.md");
    assert.deepEqual(brokenLinks(".claude/PLAN.md", "[io](.claude/strategy/io-nodes.md)", exists), [
      { file: ".claude/PLAN.md", line: 1, target: ".claude/strategy/io-nodes.md" },
    ]);
  });

  it("resolves CLAUDE.md's links from the repo root, where agents read them", () => {
    const exists = repo(".claude/rules/gotchas.md", "scripts/validate-docs.ts");
    const source = [
      "[g](.claude/rules/gotchas.md) [x](.claude/rules/missing.md)",
      "[v](scripts/validate-docs.ts) [r](rules/gotchas.md)",
    ].join("\n");
    assert.deepEqual(brokenLinks(".claude/CLAUDE.md", source, exists), [
      { file: ".claude/CLAUDE.md", line: 1, target: ".claude/rules/missing.md" },
      { file: ".claude/CLAUDE.md", line: 2, target: "rules/gotchas.md" },
    ]);
  });

  it("drops the anchor and decodes the path before resolving", () => {
    const exists = repo(".claude/research/reports/Shell script injection safety.md");
    const source = "[r](research/reports/Shell%20script%20injection%20safety.md#recommendations)";
    assert.deepEqual(brokenLinks(".claude/PLAN.md", source, exists), []);
  });

  it("resolves a `/`-leading link from the repo root, as GitHub does", () => {
    assert.deepEqual(brokenLinks("engine/README.md", "[l](/LICENSE)", repo("LICENSE")), []);
  });

  it("checks image links", () => {
    assert.deepEqual(brokenLinks("README.md", "![logo](docs/logo.png)", repo()), [
      { file: "README.md", line: 1, target: "docs/logo.png" },
    ]);
  });

  it("skips URLs, mail links and same-page anchors", () => {
    const source = "[a](https://bnto.io/x) [b](mailto:hi@bnto.io) [c](#how-it-works)";
    assert.deepEqual(brokenLinks("README.md", source, repo()), []);
  });

  it("skips links shown in code spans and fenced code blocks", () => {
    const source = [
      "Format: `[plan](./plans/x.md)`",
      "```markdown",
      "[plan](./plans/x.md)",
      "```",
      "~~~",
      "[plan](./plans/y.md)",
      "~~~",
    ].join("\n");
    assert.deepEqual(brokenLinks(".claude/plans/p.md", source, repo()), []);
  });

  it("keeps a fenced block open until a fence of its own kind closes it", () => {
    const source = "~~~\n```\n[a](missing.md)\n~~~\n[b](gone.md)";
    assert.deepEqual(brokenLinks("README.md", source, repo()), [
      { file: "README.md", line: 5, target: "gone.md" },
    ]);
  });

  it("closes a fenced block only on a bare fence at least as long as the opening one", () => {
    const gone = [{ file: "README.md", line: 5, target: "gone.md" }];
    const infoString = "```\n```js\n[a](missing.md)\n```\n[b](gone.md)";
    assert.deepEqual(brokenLinks("README.md", infoString, repo()), gone);
    const shorter = "````\n```\n[a](missing.md)\n````\n[b](gone.md)";
    assert.deepEqual(brokenLinks("README.md", shorter, repo()), gone);
  });

  it("checks reference definitions", () => {
    const source = "See [the plan][plan].\n\n[plan]: plans/gone.md";
    assert.deepEqual(brokenLinks(".claude/PLAN.md", source, repo()), [
      { file: ".claude/PLAN.md", line: 3, target: "plans/gone.md" },
    ]);
  });

  it("doesn't read a footnote definition as a link", () => {
    const source = "Text[^1].\n\n[^1]: Some explanation here.";
    assert.deepEqual(brokenLinks("README.md", source, repo()), []);
  });

  it("checks a link with a single-quoted title", () => {
    assert.deepEqual(brokenLinks("README.md", "[x](gone.md 'Gone')", repo()), [
      { file: "README.md", line: 1, target: "gone.md" },
    ]);
  });

  it("checks links again after a fenced block closes", () => {
    const source = "```\n[a](missing.md)\n```\n[b](gone.md)";
    assert.deepEqual(brokenLinks("README.md", source, repo()), [
      { file: "README.md", line: 4, target: "gone.md" },
    ]);
  });
});
