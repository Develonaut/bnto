package shellcommand_test

import (
	"context"
	"runtime"
	"testing"

	"github.com/Develonaut/bnto/pkg/node/library/shellcommand"
	"github.com/Develonaut/bnto/pkg/testgolden"
)

func TestGolden_EchoBasic(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Unix-only test")
	}

	sc := shellcommand.New()
	result, err := sc.Execute(context.Background(), map[string]any{
		"command": "echo",
		"args":    []any{"hello", "golden"},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "echo_basic", result)
}

func TestGolden_ExitCodeZero(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Unix-only test")
	}

	sc := shellcommand.New()
	result, err := sc.Execute(context.Background(), map[string]any{
		"command": "true",
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "exit_code_zero", result)
}

func TestGolden_ExitCodeNonZero(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Unix-only test")
	}

	sc := shellcommand.New()
	result, err := sc.Execute(context.Background(), map[string]any{
		"command": "false",
	})
	// shell-command node returns the result with exit code, not an error
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "exit_code_nonzero", result)
}

func TestGolden_MultilineOutput(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Unix-only test")
	}

	sc := shellcommand.New()
	result, err := sc.Execute(context.Background(), map[string]any{
		"command": "printf",
		"args":    []any{"line1\nline2\nline3"},
	})
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	testgolden.AssertGolden(t, "multiline_output", result)
}
