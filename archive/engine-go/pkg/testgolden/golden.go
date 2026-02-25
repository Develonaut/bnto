// Package testgolden provides golden file (snapshot) testing utilities.
//
// Golden files lock in the exact output of engine operations. When output
// changes, the test fails with a diff showing exactly what changed.
//
// # Updating golden files
//
// Set UPDATE_GOLDEN=1 to overwrite golden files with current output:
//
//	UPDATE_GOLDEN=1 go test ./...
package testgolden

import (
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strings"
	"testing"
)

// AssertGolden compares got against a golden file in the caller's testdata/ dir.
//
// The name is used as the filename: testdata/<name>.golden.json.
// When UPDATE_GOLDEN=1 is set, overwrites the golden file instead of comparing.
func AssertGolden(t *testing.T, name string, got any) {
	t.Helper()

	data, err := json.MarshalIndent(got, "", "  ")
	if err != nil {
		t.Fatalf("Failed to marshal result to JSON: %v", err)
	}

	normalized := NormalizeJSON(data)

	goldenPath := goldenFilePath(t, name)

	if os.Getenv("UPDATE_GOLDEN") != "" {
		if err := os.MkdirAll(filepath.Dir(goldenPath), 0755); err != nil {
			t.Fatalf("Failed to create testdata dir: %v", err)
		}
		if err := os.WriteFile(goldenPath, normalized, 0644); err != nil {
			t.Fatalf("Failed to write golden file %s: %v", goldenPath, err)
		}
		t.Logf("Updated golden file: %s", goldenPath)
		return
	}

	expected, err := os.ReadFile(goldenPath)
	if err != nil {
		t.Fatalf("Golden file not found: %s\nRun with UPDATE_GOLDEN=1 to create it.", goldenPath)
	}

	if string(normalized) != string(expected) {
		diff := unifiedDiff(string(expected), string(normalized))
		t.Errorf("Golden file mismatch: %s\n%s\nRun with UPDATE_GOLDEN=1 to update.", goldenPath, diff)
	}
}

// NormalizeJSON strips volatile fields from JSON for stable comparison.
//
// Replaces:
//   - Absolute temp paths (/tmp/..., /var/folders/..., os.TempDir()) with <TMPDIR>
//   - Duration values (e.g., "1.234s", "567ms") with <DURATION>
//   - Timestamps (ISO 8601 and Unix-style) with <TIMESTAMP>
func NormalizeJSON(data []byte) []byte {
	s := string(data)

	// Replace OS-specific temp dir prefix first (most specific match)
	tmpDir := os.TempDir()
	if tmpDir != "" {
		s = strings.ReplaceAll(s, tmpDir, "<TMPDIR>")
	}

	// Replace remaining absolute temp paths
	tempPaths := regexp.MustCompile(`(?:/tmp|/var/folders|/private/var/folders|C:\\\\Users\\\\[^"\\\\]+\\\\AppData\\\\Local\\\\Temp)[^"]*`)
	s = tempPaths.ReplaceAllString(s, "<TMPDIR>")

	// Normalize Go test temp dir random suffixes:
	// <TMPDIR>TestFoo1234567890/001/file.txt -> <TMPDIR>/file.txt
	goTestTmp := regexp.MustCompile(`<TMPDIR>(Test[A-Za-z_]+)\d+/\d+/`)
	s = goTestTmp.ReplaceAllString(s, "<TMPDIR>/")

	// Replace duration values in JSON strings
	durations := regexp.MustCompile(`"(\d+(\.\d+)?(ms|s|m|h))"`)
	s = durations.ReplaceAllString(s, `"<DURATION>"`)

	// Replace ISO 8601 timestamps
	isoTimestamp := regexp.MustCompile(`"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[^"]*"`)
	s = isoTimestamp.ReplaceAllString(s, `"<TIMESTAMP>"`)

	return []byte(s)
}

// goldenFilePath returns the path to the golden file in the caller's testdata/.
func goldenFilePath(t *testing.T, name string) string {
	t.Helper()

	_, callerFile, _, ok := runtime.Caller(2)
	if !ok {
		t.Fatal("Failed to determine caller file for golden path")
	}

	return filepath.Join(filepath.Dir(callerFile), "testdata", name+".golden.json")
}

// unifiedDiff produces a simple line-by-line diff between two strings.
func unifiedDiff(expected, actual string) string {
	expectedLines := strings.Split(expected, "\n")
	actualLines := strings.Split(actual, "\n")

	var b strings.Builder
	b.WriteString("--- expected\n+++ actual\n")

	maxLines := max(len(expectedLines), len(actualLines))

	for i := range maxLines {
		var exp, act string
		if i < len(expectedLines) {
			exp = expectedLines[i]
		}
		if i < len(actualLines) {
			act = actualLines[i]
		}

		if exp != act {
			if i < len(expectedLines) {
				b.WriteString("- " + exp + "\n")
			}
			if i < len(actualLines) {
				b.WriteString("+ " + act + "\n")
			}
		}
	}

	return b.String()
}
