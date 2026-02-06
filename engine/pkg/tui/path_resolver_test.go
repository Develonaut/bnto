package tui

import (
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/Develonaut/bento/pkg/paths"
)

func TestResolvePath_BentoHome(t *testing.T) {
	// Test {{BENTO_HOME}} marker
	testPath := "{{BENTO_HOME}}/bentos/test.json"
	resolved, err := ResolvePath(testPath)
	if err != nil {
		t.Fatalf("ResolvePath failed: %v", err)
	}

	bentoHome := LoadBentoHome()
	expected := filepath.Join(bentoHome, "bentos", "test.json")
	if resolved != expected {
		t.Errorf("Expected %s, got %s", expected, resolved)
	}
}

func TestResolvePath_EnvironmentVariable(t *testing.T) {
	// Set a test env var
	os.Setenv("TEST_VAR", "/test/path")
	defer os.Unsetenv("TEST_VAR")

	testPath := "${TEST_VAR}/subdir/file.txt"
	resolved, err := ResolvePath(testPath)
	if err != nil {
		t.Fatalf("ResolvePath failed: %v", err)
	}

	expected := filepath.Clean("/test/path/subdir/file.txt")
	if resolved != expected {
		t.Errorf("Expected %s, got %s", expected, resolved)
	}
}

func TestResolvePath_DollarSign(t *testing.T) {
	// Test $VAR syntax (without braces)
	os.Setenv("TEST_VAR", "/another/path")
	defer os.Unsetenv("TEST_VAR")

	testPath := "$TEST_VAR/file.txt"
	resolved, err := ResolvePath(testPath)
	if err != nil {
		t.Fatalf("ResolvePath failed: %v", err)
	}

	expected := filepath.Clean("/another/path/file.txt")
	if resolved != expected {
		t.Errorf("Expected %s, got %s", expected, resolved)
	}
}

func TestResolvePath_Combined(t *testing.T) {
	// Test combining special markers and env vars
	os.Setenv("PROJECT", "MyProject")
	defer os.Unsetenv("PROJECT")

	testPath := "{{BENTO_HOME}}/$PROJECT/files"
	resolved, err := ResolvePath(testPath)
	if err != nil {
		t.Fatalf("ResolvePath failed: %v", err)
	}

	bentoHome := LoadBentoHome()
	expected := filepath.Join(bentoHome, "MyProject", "files")
	if resolved != expected {
		t.Errorf("Expected %s, got %s", expected, resolved)
	}
}

func TestResolvePath_EmptyString(t *testing.T) {
	resolved, err := ResolvePath("")
	if err != nil {
		t.Fatalf("ResolvePath failed on empty string: %v", err)
	}

	if resolved != "" {
		t.Errorf("Expected empty string, got %s", resolved)
	}
}

func TestDetectGoogleDrive(t *testing.T) {
	// This test verifies that DetectGoogleDrive doesn't crash
	// We can't reliably test the actual detection without a Google Drive setup
	path := paths.DetectGoogleDrive()

	// On Mac, if Google Drive is installed, path should contain "My Drive"
	if runtime.GOOS == "darwin" && path != "" {
		if !strings.Contains(path, "My Drive") {
			t.Errorf("Expected Google Drive path to contain 'My Drive', got %s", path)
		}
	}

	// Just verify it returns a string (empty is OK if Drive isn't installed)
	t.Logf("Detected Google Drive path: %s", path)
}

func TestDetectDropbox(t *testing.T) {
	// This test verifies that DetectDropbox doesn't crash
	path := paths.DetectDropbox()

	// Just verify it returns a string (empty is OK if Dropbox isn't installed)
	t.Logf("Detected Dropbox path: %s", path)
}

func TestDetectOneDrive(t *testing.T) {
	// This test verifies that DetectOneDrive doesn't crash
	path := paths.DetectOneDrive()

	// Just verify it returns a string (empty is OK if OneDrive isn't installed)
	t.Logf("Detected OneDrive path: %s", path)
}

func TestResolvePathsInMap(t *testing.T) {
	os.Setenv("TEST_ENV", "/env/path")
	defer os.Unsetenv("TEST_ENV")

	input := map[string]string{
		"KEY1": "{{BENTO_HOME}}/file1",
		"KEY2": "${TEST_ENV}/file2",
		"KEY3": "plain/path/file3",
	}

	resolved, err := ResolvePathsInMap(input)
	if err != nil {
		t.Fatalf("ResolvePathsInMap failed: %v", err)
	}

	bentoHome := LoadBentoHome()

	// Check KEY1
	expected1 := filepath.Join(bentoHome, "file1")
	if resolved["KEY1"] != expected1 {
		t.Errorf("KEY1: Expected %s, got %s", expected1, resolved["KEY1"])
	}

	// Check KEY2
	expected2 := filepath.Clean("/env/path/file2")
	if resolved["KEY2"] != expected2 {
		t.Errorf("KEY2: Expected %s, got %s", expected2, resolved["KEY2"])
	}

	// Check KEY3
	expected3 := filepath.Clean("plain/path/file3")
	if resolved["KEY3"] != expected3 {
		t.Errorf("KEY3: Expected %s, got %s", expected3, resolved["KEY3"])
	}
}

func TestResolvePathsInMap_NilInput(t *testing.T) {
	resolved, err := ResolvePathsInMap(nil)
	if err != nil {
		t.Fatalf("ResolvePathsInMap failed on nil input: %v", err)
	}

	if resolved != nil {
		t.Errorf("Expected nil result for nil input, got %v", resolved)
	}
}

func TestExpandSpecialMarkers_GoogleDrive(t *testing.T) {
	testPath := "{{GDRIVE}}/Projects/MyProject"
	expanded, err := ResolvePath(testPath)
	if err != nil {
		t.Fatalf("ResolvePath failed: %v", err)
	}

	// If Google Drive is detected, path should be expanded
	gdrivePath := paths.DetectGoogleDrive()
	if gdrivePath != "" {
		expected := filepath.Join(gdrivePath, "Projects", "MyProject")
		if expanded != expected {
			t.Errorf("Expected %s, got %s", expected, expanded)
		}
	} else {
		// If no Google Drive, marker should remain unchanged
		if expanded != testPath {
			t.Errorf("Expected marker to remain unchanged when Google Drive not detected, got %s", expanded)
		}
	}
}
