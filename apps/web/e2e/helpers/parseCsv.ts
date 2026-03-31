/**
 * CSV output parser — split CSV text into headers + rows with quoted field support.
 */

/**
 * Parse CSV text into headers + rows. Handles quoted fields with commas.
 */
export function parseCsvOutput(data: Buffer | string): {
  headers: string[];
  rows: string[][];
} {
  const text = typeof data === "string" ? data : data.toString("utf-8");
  const lines = text
    .trim()
    .split("\n")
    .filter((l) => l.trim() !== "");
  if (lines.length === 0) return { headers: [], rows: [] };

  const parseLine = (line: string): string[] => {
    const cells: string[] = [];
    let current = "";
    let inQuotes = false;
    for (const ch of line) {
      if (ch === '"') {
        inQuotes = !inQuotes;
      } else if (ch === "," && !inQuotes) {
        cells.push(current.trim());
        current = "";
      } else {
        current += ch;
      }
    }
    cells.push(current.trim());
    return cells;
  };

  const headers = parseLine(lines[0]);
  const rows = lines.slice(1).map(parseLine);
  return { headers, rows };
}
