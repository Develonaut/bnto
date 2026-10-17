# Decision: Nodes Are Generic Building Blocks

**Created:** October 2026
**Status:** Decided
**Applies to:** Every engine node: new nodes, new params on existing nodes, review of both

---

## The Principle

**A node is a single-purpose, generic building block. A user's problem is solved by combining nodes, not by teaching one node about that problem.**

When a use case needs something no recipe can express yet, find the missing generic capability and add that, as its own node or as an engine feature every node inherits. Don't add a mode, a sidecar file format or a use-case flag to an existing node because one recipe needs it.

## The Test

Before adding a node or a param, ask:

1. **Does it do one thing?** Name it in a short phrase with no "and" and no "or, in this mode…". `file-rename` renames by a template. It does not also look up a table.
2. **Is it useful outside the case that prompted it?** A node that only makes sense for one recipe is that recipe's logic in the wrong place.
3. **Could existing nodes do it if one generic piece existed?** If so, build that piece. The fix is often in the engine (templating, iteration, metadata flow), not in a node.
4. **Would n8n, Node-RED or NiFi split it?** These tools have converged on small steps joined by data. If they model it as two steps, so do we.

## Worked Example: Renaming a Library from a Mapping CSV

A recipe needed ~8,300 sound-effect files renamed from a mapping table (a CSV of old name to new name).

**Rejected:** a `rules` mode on `file-rename`, with a CSV of find/replace rows, first match wins, the row's columns as metadata, an `unmatched` flag and a CLI warning about it. It made `file-rename` two nodes behind one name. The editor couldn't show or hide the rules params cleanly, and the table lookup, the reusable part, was locked inside rename.

**Chosen:** generic pieces, the way other tools do it:

| Step                   | Generic capability                                                         | Prior art                                                                      |
| ---------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Join files to CSV rows | A lookup node: match each item to a table row by key, attach its columns   | n8n Merge "Combine by Matching Fields", NiFi LookupRecord, Zapier Lookup Table |
| Rename from those rows | `file-rename` with `pattern: "{{item.new_name}}"` (per-file templates)     | n8n file-name expressions, NiFi UpdateAttribute, Advanced Renamer `<Csv:X>`    |
| Report what happened   | Generic outputs (unmatched items flagged or routed, a CSV export of items) | NiFi matched/unmatched relationships, n8n Convert to File                      |

Each piece is useful far beyond renaming: tagging files by category, routing on a looked-up value, exporting any run's items.

## What This Means in Review

- A new param that only applies "when X is set" and reshapes what the node does is a red flag. Ask whether it's a second node.
- Use-case vocabulary (a product name, one team's file format) in a node's params or docs means use-case logic has leaked in.
- "Someone needs it for a specific case" is a reason to find the missing building block, not a reason to special-case a node.
