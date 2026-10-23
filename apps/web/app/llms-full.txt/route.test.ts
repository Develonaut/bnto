import { describe, expect, it } from "vitest";
import { GET } from "./route";

async function recipeSection(name: string) {
  const text = await GET().text();
  return text.split("### ").find((section) => section.startsWith(`${name}\n`)) ?? "";
}

describe("llms-full.txt", () => {
  it("gives a CLI-only recipe its run command", async () => {
    const section = await recipeSection("Keep Awake");
    expect(section).toContain("- Run: bnto run keep-awake\n");
    expect(section).not.toContain("Accepts:");
  });

  it("names the URL argument for a URL recipe", async () => {
    expect(await recipeSection("Download Video")).toContain("- Run: bnto run download-video <url>");
  });

  it("says which operating systems a CLI-only recipe runs on", async () => {
    expect(await recipeSection("Keep Awake")).toContain("- Runs on: macOS, Linux, Windows\n");
  });

  it("leaves the OS line off a browser recipe, which runs in any browser", async () => {
    expect(await recipeSection("Compress Images")).not.toContain("Runs on:");
  });

  it("lists accepted formats for a browser recipe", async () => {
    expect(await recipeSection("Compress Images")).toContain("- Accepts: ");
  });
});
