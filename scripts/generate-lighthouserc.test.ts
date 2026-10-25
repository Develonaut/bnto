/**
 * Tests for generate-lighthouserc.ts: run the script, read the config it writes.
 * Run: pnpm exec tsx --test scripts/generate-lighthouserc.test.ts
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { createServer, type Server } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, it } from "node:test";

interface Collect {
  url: string[];
  startServerCommand?: string;
}

const SCRIPT = "scripts/generate-lighthouserc.ts";

/** Runs the generator into a temp dir and returns its exit status and collect section. */
function generate(...args: string[]) {
  const dir = mkdtempSync(join(tmpdir(), "lighthouserc-"));
  const out = join(dir, "lighthouserc.json");
  const run = spawnSync("pnpm", ["exec", "tsx", SCRIPT, "--output", out, ...args], {
    encoding: "utf8",
  });
  const config = run.status === 0 ? JSON.parse(readFileSync(out, "utf8")) : undefined;
  rmSync(dir, { recursive: true, force: true });
  const collect: Collect | undefined = config?.ci.collect;
  return { status: run.status, stderr: run.stderr, collect };
}

/** Listens on `port` like a running audit's server; undefined when something already does. */
function hold(port: number) {
  return new Promise<Server | undefined>((resolve) => {
    const server = createServer();
    server.once("error", () => resolve(undefined));
    server.listen(port, () => resolve(server));
  });
}

function portOf(collect: Collect) {
  return Number(new URL(collect.url[0]).port);
}

describe("generate-lighthouserc --with-server", () => {
  it("starts Next on the port every audited URL names", () => {
    const { collect } = generate("--with-server");
    assert.ok(collect);
    const port = new URL(collect.url[0]).port;
    assert.ok(collect.url.every((url) => new URL(url).port === port));
    assert.match(collect.startServerCommand ?? "", new RegExp(`--port ${port}$`));
  });

  it("picks a free port, not the one another audit's server holds", async () => {
    const first = generate("--with-server").collect;
    assert.ok(first);
    const server = await hold(portOf(first));
    try {
      const second = generate("--with-server").collect;
      assert.ok(second);
      assert.notEqual(portOf(second), portOf(first));
      const probe = await hold(portOf(second));
      assert.ok(probe, "the second audit's port is in use");
      probe.close();
    } finally {
      server?.close();
    }
  });

  it("refuses --base-url, whose server it would not start", () => {
    const { status, stderr } = generate("--with-server", "--base-url", "http://localhost:4100");
    assert.notEqual(status, 0);
    assert.match(stderr, /--base-url/);
  });
});

describe("generate-lighthouserc --base-url", () => {
  it("audits the given URL and starts no server", () => {
    const { collect } = generate("--base-url", "https://example.vercel.app");
    assert.ok(collect);
    assert.equal(collect.url[0], "https://example.vercel.app/");
    assert.equal(collect.startServerCommand, undefined);
  });
});
