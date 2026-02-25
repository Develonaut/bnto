package integration

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// CLI smoke tests exercise the bnto binary against real fixture files.
// These complement cmd/bnto/commands_test.go (which uses synthetic bntos)
// by testing with actual multi-node workflow fixtures.

func TestCLI_RunEditFieldsPipeline(t *testing.T) {
	skipIfNoBnto(t)

	output, err := RunBnto(t, "tests/fixtures/workflows/edit-fields-pipeline.bnto.json", nil)
	if err != nil {
		t.Fatalf("bnto run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "Delicious") {
		t.Errorf("Expected success message, got: %s", output)
	}
}

func TestCLI_RunCSVPipeline(t *testing.T) {
	skipIfNoBnto(t)

	output, err := RunBnto(t, "tests/fixtures/workflows/csv-data-pipeline.bnto.json", nil)
	if err != nil {
		t.Fatalf("bnto run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "Delicious") {
		t.Errorf("Expected success message, got: %s", output)
	}
}

func TestCLI_ValidateValidFixture(t *testing.T) {
	skipIfNoBnto(t)

	output, err := runBntoValidate(t, "tests/fixtures/workflows/edit-fields-pipeline.bnto.json")
	if err != nil {
		t.Fatalf("bnto validate failed: %v\nOutput: %s", err, output)
	}

	lower := strings.ToLower(output)
	if !strings.Contains(lower, "valid") {
		t.Errorf("Expected validation success message, got: %s", output)
	}
}

func TestCLI_ValidateInvalidFixture(t *testing.T) {
	skipIfNoBnto(t)

	output, err := runBntoValidate(t, "tests/fixtures/workflows/invalid-workflow.bnto.json")
	if err == nil {
		t.Fatalf("Expected validation to fail, but it succeeded.\nOutput: %s", output)
	}

	lower := strings.ToLower(output)
	if !strings.Contains(lower, "error") && !strings.Contains(lower, "failed") {
		t.Errorf("Expected error in output, got: %s", output)
	}
}

func TestCLI_ListFixtureDir(t *testing.T) {
	skipIfNoBnto(t)

	output, err := runBntoList(t, "tests/fixtures/workflows")
	if err != nil {
		t.Fatalf("bnto list failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "bnto.json") {
		t.Errorf("Expected .bnto.json files listed, got: %s", output)
	}

	if !strings.Contains(output, "found") {
		t.Errorf("Expected count summary, got: %s", output)
	}
}

func TestCLI_DryRunFixture(t *testing.T) {
	skipIfNoBnto(t)

	output, err := runBntoDryRun(t, "tests/fixtures/workflows/edit-fields-pipeline.bnto.json")
	if err != nil {
		t.Fatalf("bnto --dry-run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "DRY RUN") {
		t.Errorf("Expected DRY RUN message, got: %s", output)
	}

	if strings.Contains(output, "Delicious") {
		t.Error("Dry run should not execute the workflow")
	}
}

// skipIfNoBnto skips the test if the bnto binary is not in PATH.
func skipIfNoBnto(t *testing.T) {
	t.Helper()
	if _, err := exec.LookPath("bnto"); err != nil {
		t.Skip("bnto binary not in PATH, skipping CLI smoke test")
	}
}

// runBntoValidate executes bnto validate on a fixture file.
func runBntoValidate(t *testing.T, bntoPath string) (string, error) {
	t.Helper()
	return runBntoCommand(t, "validate", bntoPath, nil)
}

// runBntoList executes bnto list on a directory.
func runBntoList(t *testing.T, dir string) (string, error) {
	t.Helper()
	return runBntoCommand(t, "list", dir, nil)
}

// runBntoDryRun executes bnto run --dry-run on a fixture file.
func runBntoDryRun(t *testing.T, bntoPath string) (string, error) {
	t.Helper()

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	projectRoot := filepath.Join(wd, "../..")

	cmd := exec.Command("bnto", "run", bntoPath, "--dry-run")
	cmd.Dir = projectRoot
	cmd.Env = os.Environ()

	output, err := cmd.CombinedOutput()
	return string(output), err
}

// runBntoCommand executes a bnto subcommand from the project root.
func runBntoCommand(t *testing.T, subcommand, arg string, envVars map[string]string) (string, error) {
	t.Helper()

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	projectRoot := filepath.Join(wd, "../..")

	cmd := exec.Command("bnto", subcommand, arg)
	cmd.Dir = projectRoot
	cmd.Env = os.Environ()
	for k, v := range envVars {
		cmd.Env = append(cmd.Env, k+"="+v)
	}

	output, err := cmd.CombinedOutput()
	return string(output), err
}
