/** The release version: `[workspace.package] version` in engine/Cargo.toml. */
export function parseReleaseVersion(workspaceToml: string): string {
  const table = workspaceToml.split(/^\[/m).find((t) => t.startsWith("workspace.package]"));
  const version = table?.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  if (!version) throw new Error("engine/Cargo.toml has no [workspace.package] version");
  return version;
}
