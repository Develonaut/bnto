package integration

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// CLI smoke tests exercise the bento binary against real fixture files.
// These complement cmd/bento/commands_test.go (which uses synthetic bentos)
// by testing with actual multi-node workflow fixtures.

func TestCLI_RunEditFieldsPipeline(t *testing.T) {
	skipIfNoBento(t)

	output, err := RunBento(t, "tests/fixtures/workflows/edit-fields-pipeline.bento.json", nil)
	if err != nil {
		t.Fatalf("bento run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "Delicious") {
		t.Errorf("Expected success message, got: %s", output)
	}
}

func TestCLI_RunCSVPipeline(t *testing.T) {
	skipIfNoBento(t)

	output, err := RunBento(t, "tests/fixtures/workflows/csv-data-pipeline.bento.json", nil)
	if err != nil {
		t.Fatalf("bento run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "Delicious") {
		t.Errorf("Expected success message, got: %s", output)
	}
}

func TestCLI_ValidateValidFixture(t *testing.T) {
	skipIfNoBento(t)

	output, err := runBentoValidate(t, "tests/fixtures/workflows/edit-fields-pipeline.bento.json")
	if err != nil {
		t.Fatalf("bento validate failed: %v\nOutput: %s", err, output)
	}

	lower := strings.ToLower(output)
	if !strings.Contains(lower, "valid") {
		t.Errorf("Expected validation success message, got: %s", output)
	}
}

func TestCLI_ValidateInvalidFixture(t *testing.T) {
	skipIfNoBento(t)

	output, err := runBentoValidate(t, "tests/fixtures/workflows/invalid-workflow.bento.json")
	if err == nil {
		t.Fatalf("Expected validation to fail, but it succeeded.\nOutput: %s", output)
	}

	lower := strings.ToLower(output)
	if !strings.Contains(lower, "error") && !strings.Contains(lower, "failed") {
		t.Errorf("Expected error in output, got: %s", output)
	}
}

func TestCLI_ListFixtureDir(t *testing.T) {
	skipIfNoBento(t)

	output, err := runBentoList(t, "tests/fixtures/workflows")
	if err != nil {
		t.Fatalf("bento list failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "bento.json") {
		t.Errorf("Expected .bento.json files listed, got: %s", output)
	}

	if !strings.Contains(output, "found") {
		t.Errorf("Expected count summary, got: %s", output)
	}
}

func TestCLI_DryRunFixture(t *testing.T) {
	skipIfNoBento(t)

	output, err := runBentoDryRun(t, "tests/fixtures/workflows/edit-fields-pipeline.bento.json")
	if err != nil {
		t.Fatalf("bento --dry-run failed: %v\nOutput: %s", err, output)
	}

	if !strings.Contains(output, "DRY RUN") {
		t.Errorf("Expected DRY RUN message, got: %s", output)
	}

	if strings.Contains(output, "Delicious") {
		t.Error("Dry run should not execute the workflow")
	}
}

// skipIfNoBento skips the test if the bento binary is not in PATH.
func skipIfNoBento(t *testing.T) {
	t.Helper()
	if _, err := exec.LookPath("bento"); err != nil {
		t.Skip("bento binary not in PATH, skipping CLI smoke test")
	}
}

// runBentoValidate executes bento validate on a fixture file.
func runBentoValidate(t *testing.T, bentoPath string) (string, error) {
	t.Helper()
	return runBentoCommand(t, "validate", bentoPath, nil)
}

// runBentoList executes bento list on a directory.
func runBentoList(t *testing.T, dir string) (string, error) {
	t.Helper()
	return runBentoCommand(t, "list", dir, nil)
}

// runBentoDryRun executes bento run --dry-run on a fixture file.
func runBentoDryRun(t *testing.T, bentoPath string) (string, error) {
	t.Helper()

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	projectRoot := filepath.Join(wd, "../..")

	cmd := exec.Command("bento", "run", bentoPath, "--dry-run")
	cmd.Dir = projectRoot
	cmd.Env = os.Environ()

	output, err := cmd.CombinedOutput()
	return string(output), err
}

// runBentoCommand executes a bento subcommand from the project root.
func runBentoCommand(t *testing.T, subcommand, arg string, envVars map[string]string) (string, error) {
	t.Helper()

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}
	projectRoot := filepath.Join(wd, "../..")

	cmd := exec.Command("bento", subcommand, arg)
	cmd.Dir = projectRoot
	cmd.Env = os.Environ()
	for k, v := range envVars {
		cmd.Env = append(cmd.Env, k+"="+v)
	}

	output, err := cmd.CombinedOutput()
	return string(output), err
}
