// Package main_test provides integration tests for the normalized bnto CLI commands.
//
// These tests verify the new literal command names work correctly:
//   - run (formerly savor)
//   - validate (formerly sample)
//   - list (formerly menu)
//   - new (formerly box)
//   - docs (formerly recipe)
//   - secrets (formerly wasabi)
package main_test

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Test: bnto run command (formerly savor)

// TestRunCommand_ValidBnto verifies that run executes a valid bnto successfully.
func TestRunCommand_ValidBnto(t *testing.T) {
	bntoFile := createTestBnto(t, "test.bnto.json", simpleValidBnto(""))
	defer os.Remove(bntoFile)

	cmd := exec.Command("bnto", "run", bntoFile)
	verifyCommandSuccess(t, cmd, "Delicious")

	// Success is verified by "Delicious" message from itamae
	// Output includes random sushi emoji (🍣, 🍙, etc.) not checkmark
}

// TestRunCommand_InvalidBnto verifies proper error handling for invalid bntos.
func TestRunCommand_InvalidBnto(t *testing.T) {
	bntoFile := createTestBnto(t, "invalid.bnto.json", `{
		"id": "invalid",
		"type": "group",
		"name": "Invalid Bnto"
	}`)
	defer os.Remove(bntoFile)

	verifyCommandError(t, exec.Command("bnto", "run", bntoFile))
}

// TestRunCommand_VerboseFlag verifies verbose output includes details.
func TestRunCommand_VerboseFlag(t *testing.T) {
	bntoFile := createTestBnto(t, "test.bnto.json", simpleValidBnto(""))
	defer os.Remove(bntoFile)

	output := verifyCommandSuccess(t, exec.Command("bnto", "run", bntoFile, "--verbose"), "Delicious")
	if !strings.Contains(output, "node-1") {
		t.Error("Verbose output should mention node IDs")
	}
}

// TestRunCommand_DryRun verifies dry run flag prevents execution.
func TestRunCommand_DryRun(t *testing.T) {
	bntoFile := createTestBnto(t, "test.bnto.json", simpleValidBnto(""))
	defer os.Remove(bntoFile)

	output := verifyCommandSuccess(t, exec.Command("bnto", "run", bntoFile, "--dry-run"), "DRY RUN")

	// Should NOT actually execute
	if strings.Contains(output, "Delicious! Bnto savored successfully") {
		t.Error("Dry run should NOT execute the bnto")
	}

	// Should show what WOULD be executed
	if !strings.Contains(output, "Would execute") {
		t.Error("Dry run should show what would be executed")
	}
}

// TestRunCommand_DryRunVerbose verifies dry run with verbose shows details.
func TestRunCommand_DryRunVerbose(t *testing.T) {
	bntoFile := createTestBnto(t, "test.bnto.json", simpleValidBnto(""))
	defer os.Remove(bntoFile)

	output := verifyCommandSuccess(t, exec.Command("bnto", "run", bntoFile, "--dry-run", "--verbose"), "DRY RUN")

	// Should show node details in verbose mode
	if !strings.Contains(output, "node-1") {
		t.Error("Verbose dry run should show node IDs")
	}
}

// TestRunCommand_HelpText verifies help text has no emojis and is literal.
func TestRunCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "run", "--help"), "Execute")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "🍣") || strings.Contains(output, "Savor") {
		t.Error("Help text should not contain emojis or sushi theme")
	}

	// Should contain literal description
	if !strings.Contains(output, "Execute a bnto workflow") {
		t.Error("Help text should contain literal description")
	}
}

// Test: bnto validate command (formerly sample)

// TestValidateCommand_ValidBnto verifies validate checks without executing.
func TestValidateCommand_ValidBnto(t *testing.T) {
	bntoFile := createTestBnto(t, "test.bnto.json", simpleValidBnto(""))
	defer os.Remove(bntoFile)

	output := verifyCommandSuccess(t, exec.Command("bnto", "validate", bntoFile), "valid")
	if strings.Contains(output, "Delicious! Bnto savored") {
		t.Error("validate should NOT execute the bnto")
	}
}

// TestValidateCommand_InvalidBnto verifies validate reports validation errors clearly.
func TestValidateCommand_InvalidBnto(t *testing.T) {
	bntoFile := createTestBnto(t, "invalid.bnto.json", invalidHTTPBnto())
	defer os.Remove(bntoFile)

	verifyCommandError(t, exec.Command("bnto", "validate", bntoFile))
}

// TestValidateCommand_HelpText verifies help text is literal.
func TestValidateCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "validate", "--help"), "Validate")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "🥢") || strings.Contains(output, "Sample") {
		t.Error("Help text should not contain emojis or sushi theme")
	}

	// Should contain literal description
	if !strings.Contains(output, "Validate a bnto workflow") {
		t.Error("Help text should contain literal description")
	}
}

// Test: bnto list command (formerly menu)

// TestListCommand_ListBntos verifies list shows all bntos in directory.
func TestListCommand_ListBntos(t *testing.T) {
	tmpDir := t.TempDir()
	createTestBntoInDir(t, tmpDir, "workflow1.bnto.json", "Workflow 1")
	createTestBntoInDir(t, tmpDir, "workflow2.bnto.json", "Workflow 2")

	output := verifyCommandSuccess(t, exec.Command("bnto", "list", tmpDir), "2")
	verifyBntosListed(t, output, "workflow1.bnto.json", "workflow2.bnto.json")
}

// TestListCommand_EmptyDirectory verifies list handles empty directory gracefully.
func TestListCommand_EmptyDirectory(t *testing.T) {
	tmpDir := t.TempDir()
	output := verifyCommandSuccess(t, exec.Command("bnto", "list", tmpDir), "")
	if !strings.Contains(output, "0") && !strings.Contains(output, "No") && !strings.Contains(output, "none") {
		t.Errorf("Output should indicate no bntos found: %s", output)
	}
}

// TestListCommand_HelpText verifies help text is literal.
func TestListCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "list", "--help"), "List")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "🍜") || strings.Contains(output, "menu") {
		t.Error("Help text should not contain emojis or menu theme")
	}

	// Should contain literal description
	if !strings.Contains(output, "List") && !strings.Contains(output, "available") {
		t.Error("Help text should contain literal description with 'List' and 'available'")
	}
}

// Test: bnto new command (formerly box)

// TestNewCommand_CreateTemplate verifies new creates a template bnto.
func TestNewCommand_CreateTemplate(t *testing.T) {
	tmpDir := t.TempDir()
	bntoPath := filepath.Join(tmpDir, "my-workflow.bnto.json")

	oldDir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Chdir(tmpDir); err != nil {
		t.Fatal(err)
	}
	defer func() {
		if err := os.Chdir(oldDir); err != nil {
			t.Logf("failed to restore directory: %v", err)
		}
	}()

	verifyCommandSuccess(t, exec.Command("bnto", "new", "my-workflow", "--local"), "Created")
	verifyBntoJSONValid(t, bntoPath, "my-workflow")
}

// TestNewCommand_OverwriteProtection verifies new doesn't overwrite existing files.
func TestNewCommand_OverwriteProtection(t *testing.T) {
	tmpDir := t.TempDir()
	createExistingFile(t, tmpDir, "existing.bnto.json")
	changeToDir(t, tmpDir)

	cmd := exec.Command("bnto", "new", "existing", "--local")
	output, err := cmd.CombinedOutput()

	// Command should error because file exists
	if err == nil {
		outputStr := strings.ToLower(string(output))
		if !strings.Contains(outputStr, "exists") && !strings.Contains(outputStr, "already") {
			t.Errorf("Should warn or error about existing file, got output: %s", output)
		}
	} else {
		// If it errored (which is expected), check that the error message mentions the file exists
		outputStr := strings.ToLower(string(output))
		if !strings.Contains(outputStr, "exists") && !strings.Contains(outputStr, "already") {
			t.Errorf("Error should mention file exists, got: %s", output)
		}
	}
}

// TestNewCommand_DryRun verifies dry run flag prevents file creation.
func TestNewCommand_DryRun(t *testing.T) {
	tmpDir := t.TempDir()
	changeToDir(t, tmpDir)

	bntoPath := filepath.Join(tmpDir, "my-workflow.bnto.json")

	output := verifyCommandSuccess(t, exec.Command("bnto", "new", "my-workflow", "--dry-run"), "DRY RUN")

	// Should NOT create the file
	if _, err := os.Stat(bntoPath); !os.IsNotExist(err) {
		t.Error("Dry run should NOT create the bnto file")
	}

	// Should show what WOULD be created
	if !strings.Contains(output, "Would create") {
		t.Error("Dry run should show what would be created")
	}
}

// TestNewCommand_HelpText verifies help text is literal.
func TestNewCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "new", "--help"), "Create")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "🥡") || strings.Contains(output, "box") {
		t.Error("Help text should not contain emojis or box theme")
	}

	// Should contain literal description
	if !strings.Contains(output, "Create a new bnto") {
		t.Error("Help text should contain literal description")
	}
}

// Test: bnto docs command (formerly recipe)

// TestDocsCommand_ShowsHelp verifies docs command help works.
func TestDocsCommand_ShowsHelp(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "docs", "--help"), "View")
	if !strings.Contains(output, "readme") {
		t.Error("Help should list available docs like 'readme'")
	}
}

// TestDocsCommand_InvalidDoc verifies docs handles invalid doc names.
func TestDocsCommand_InvalidDoc(t *testing.T) {
	cmd := exec.Command("bnto", "docs", "nonexistent-doc")
	output, err := cmd.CombinedOutput()
	if err == nil {
		t.Fatal("Command should fail for invalid doc name")
	}
	outputStr := string(output)
	if !strings.Contains(outputStr, "unknown doc") {
		t.Errorf("Should mention 'unknown doc': %s", outputStr)
	}
}

// TestDocsCommand_HelpText verifies help text is literal.
func TestDocsCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "docs", "--help"), "documentation")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "📖") || strings.Contains(output, "recipe") {
		t.Error("Help text should not contain emojis or recipe theme")
	}

	// Should contain literal description
	if !strings.Contains(output, "View bnto documentation") {
		t.Error("Help text should contain literal description")
	}
}

// Test: bnto secrets command (formerly wasabi)

// TestSecretsCommand_ShowsHelp verifies secrets command help works.
func TestSecretsCommand_ShowsHelp(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "secrets", "--help"), "Manage secrets")

	// Should NOT contain sushi-themed text
	if strings.Contains(output, "🟢") || strings.Contains(output, "wasabi") || strings.Contains(output, "spicy") {
		t.Error("Help text should not contain emojis or wasabi theme")
	}
}

// TestSecretsCommand_ListSubcommand verifies secrets list subcommand exists.
func TestSecretsCommand_ListSubcommand(t *testing.T) {
	cmd := exec.Command("bnto", "secrets", "list")
	output, err := cmd.CombinedOutput()
	// Command might succeed (empty list) or fail (no secrets), but should recognize 'list'
	if err != nil {
		outputStr := strings.ToLower(string(output))
		if strings.Contains(outputStr, "unknown command") {
			t.Error("'secrets list' should be a valid command")
		}
	}
}

// TestSecretsCommand_SetSubcommand verifies secrets set requires arguments.
func TestSecretsCommand_SetSubcommand(t *testing.T) {
	cmd := exec.Command("bnto", "secrets", "set")
	output, err := cmd.CombinedOutput()
	if err == nil {
		t.Fatal("'secrets set' without arguments should fail")
	}
	outputStr := string(output)
	if !strings.Contains(outputStr, "accepts") && !strings.Contains(outputStr, "arg") {
		t.Error("Should indicate missing arguments")
	}
}

// TestSecretsCommand_GetSubcommand verifies secrets get requires arguments.
func TestSecretsCommand_GetSubcommand(t *testing.T) {
	cmd := exec.Command("bnto", "secrets", "get")
	output, err := cmd.CombinedOutput()
	if err == nil {
		t.Fatal("'secrets get' without arguments should fail")
	}
	outputStr := string(output)
	if !strings.Contains(outputStr, "accepts") && !strings.Contains(outputStr, "arg") {
		t.Error("Should indicate missing arguments")
	}
}

// TestSecretsCommand_DeleteSubcommand verifies secrets delete requires arguments.
func TestSecretsCommand_DeleteSubcommand(t *testing.T) {
	cmd := exec.Command("bnto", "secrets", "delete")
	output, err := cmd.CombinedOutput()
	if err == nil {
		t.Fatal("'secrets delete' without arguments should fail")
	}
	outputStr := string(output)
	if !strings.Contains(outputStr, "accepts") && !strings.Contains(outputStr, "arg") {
		t.Error("Should indicate missing arguments")
	}
}

// Test: Root command help text

// TestRootCommand_HelpText verifies root command has no sushi theme.
func TestRootCommand_HelpText(t *testing.T) {
	output := verifyCommandSuccess(t, exec.Command("bnto", "--help"), "Available Commands")

	// Should NOT contain sushi-themed description
	if strings.Contains(output, "sushi-themed") {
		t.Error("Root help should not mention sushi theme")
	}

	// Should show all new command names
	expectedCommands := []string{"run", "validate", "list", "new", "docs", "secrets", "version"}
	for _, cmd := range expectedCommands {
		if !strings.Contains(output, cmd) {
			t.Errorf("Root help should list '%s' command", cmd)
		}
	}

	// Should NOT show old command names
	oldCommands := []string{"savor", "sample", "menu", "box", "recipe", "wasabi"}
	for _, cmd := range oldCommands {
		if strings.Contains(output, cmd) {
			t.Errorf("Root help should NOT list old command '%s'", cmd)
		}
	}
}
