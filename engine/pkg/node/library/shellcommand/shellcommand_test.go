package shellcommand_test

import (
	"context"
	"runtime"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/Develonaut/bnto/pkg/node/library/shellcommand"
)

// TestShellCommand_BasicExecution tests basic command execution.
func TestShellCommand_BasicExecution(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{"hello", "world"},
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	if !strings.Contains(stdout, "hello world") {
		t.Errorf("stdout = %q, want to contain 'hello world'", stdout)
	}

	exitCode, ok := output["exitCode"].(int)
	if !ok {
		t.Fatalf("exitCode is not an int: %T", output["exitCode"])
	}

	if exitCode != 0 {
		t.Errorf("exitCode = %v, want 0", exitCode)
	}
}

// TestShellCommand_WithArguments tests command execution with arguments.
func TestShellCommand_WithArguments(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		// Windows command
		params = map[string]interface{}{
			"command": "cmd",
			"args":    []interface{}{"/C", "echo", "test-arg-123"},
		}
	} else {
		// Unix command
		params = map[string]interface{}{
			"command": "echo",
			"args":    []interface{}{"test-arg-123"},
		}
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	if !strings.Contains(stdout, "test-arg-123") {
		t.Errorf("stdout = %q, want to contain 'test-arg-123'", stdout)
	}
}

// TestShellCommand_StdoutStderr tests stdout and stderr capture.
func TestShellCommand_StdoutStderr(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		// Windows: redirect stderr
		params = map[string]interface{}{
			"command": "cmd",
			"args":    []interface{}{"/C", "echo error message 1>&2"},
		}
	} else {
		// Unix: redirect stderr
		params = map[string]interface{}{
			"command": "sh",
			"args":    []interface{}{"-c", "echo 'error message' >&2"},
		}
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	stderr := output["stderr"].(string)

	if !strings.Contains(stderr, "error message") {
		t.Errorf("stderr = %q, want to contain 'error message'", stderr)
	}
}

// TestShellCommand_ExitCode tests capturing non-zero exit codes.
func TestShellCommand_ExitCode(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		params = map[string]interface{}{
			"command": "cmd",
			"args":    []interface{}{"/C", "exit", "42"},
		}
	} else {
		params = map[string]interface{}{
			"command": "sh",
			"args":    []interface{}{"-c", "exit 42"},
		}
	}

	result, err := sc.Execute(ctx, params)
	// Non-zero exit should NOT be an error - we capture it in exitCode
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	exitCode := output["exitCode"].(int)

	if exitCode != 42 {
		t.Errorf("exitCode = %v, want 42", exitCode)
	}
}

// TestShellCommand_ConfigurableTimeout tests that timeout can be configured.
func TestShellCommand_ConfigurableTimeout(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		// Windows: sleep for 2 seconds using timeout command
		params = map[string]interface{}{
			"command": "timeout",
			"args":    []interface{}{"/T", "2", "/NOBREAK"},
			"timeout": 5, // 5 second timeout (should NOT timeout)
		}
	} else {
		// Unix: sleep for 2 seconds
		params = map[string]interface{}{
			"command": "sleep",
			"args":    []interface{}{"2"},
			"timeout": 5, // 5 second timeout (should NOT timeout)
		}
	}

	start := time.Now()
	result, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Should complete in about 2 seconds, definitely less than 5
	if duration > 4*time.Second {
		t.Errorf("Command took too long: %v", duration)
	}

	output := result.(map[string]interface{})
	exitCode := output["exitCode"].(int)

	if exitCode != 0 {
		t.Errorf("exitCode = %v, want 0", exitCode)
	}
}

// TestShellCommand_Timeout tests that commands timeout appropriately.
func TestShellCommand_Timeout(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		// Windows: sleep for 5 seconds
		params = map[string]interface{}{
			"command": "timeout",
			"args":    []interface{}{"/T", "5", "/NOBREAK"},
			"timeout": 1, // 1 second timeout (SHOULD timeout)
		}
	} else {
		// Unix: sleep for 5 seconds
		params = map[string]interface{}{
			"command": "sleep",
			"args":    []interface{}{"5"},
			"timeout": 1, // 1 second timeout (SHOULD timeout)
		}
	}

	_, err := sc.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected timeout error, got nil")
	}

	if !strings.Contains(err.Error(), "timeout") && !strings.Contains(err.Error(), "deadline") && !strings.Contains(err.Error(), "killed") {
		t.Errorf("Expected timeout/deadline/killed error, got: %v", err)
	}
}

// TestShellCommand_StreamingOutput tests streaming output callback.
func TestShellCommand_StreamingOutput(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}
	var outputLines []string

	// Callback to capture streaming output
	onOutput := func(line string) {
		outputLines = append(outputLines, line)
	}

	if runtime.GOOS == "windows" {
		params = map[string]interface{}{
			"command":   "cmd",
			"args":      []interface{}{"/C", "for /L %i in (1,1,3) do @echo %i"},
			"stream":    true,
			"_onOutput": onOutput,
		}
	} else {
		params = map[string]interface{}{
			"command":   "sh",
			"args":      []interface{}{"-c", "for i in 1 2 3; do echo $i; done"},
			"stream":    true,
			"_onOutput": onOutput,
		}
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Verify we got streaming output
	if len(outputLines) < 3 {
		t.Errorf("Expected at least 3 output lines, got %d: %v", len(outputLines), outputLines)
	}

	// Verify final result also has stdout
	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	if !strings.Contains(stdout, "1") || !strings.Contains(stdout, "2") || !strings.Contains(stdout, "3") {
		t.Errorf("stdout missing expected numbers: %q", stdout)
	}
}

// TestShellCommand_LongRunning tests long-running commands.
func TestShellCommand_LongRunning(t *testing.T) {
	if testing.Short() {
		t.Skip("Skipping long-running test in short mode")
	}

	ctx := context.Background()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		params = map[string]interface{}{
			"command": "timeout",
			"args":    []interface{}{"/T", "3", "/NOBREAK"},
			"timeout": 10, // 10 second timeout (won't timeout)
		}
	} else {
		params = map[string]interface{}{
			"command": "sleep",
			"args":    []interface{}{"3"},
			"timeout": 10, // 10 second timeout (won't timeout)
		}
	}

	start := time.Now()
	result, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Should take at least 3 seconds
	if duration < 2*time.Second {
		t.Errorf("Command finished too quickly: %v", duration)
	}

	output := result.(map[string]interface{})
	exitCode := output["exitCode"].(int)

	if exitCode != 0 {
		t.Errorf("exitCode = %v, want 0", exitCode)
	}
}

// TestShellCommand_ContextCancellation tests that context cancellation stops the command.
func TestShellCommand_ContextCancellation(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	sc := shellcommand.New()

	var params map[string]interface{}

	if runtime.GOOS == "windows" {
		// Sleep for 10 seconds (will be cancelled)
		params = map[string]interface{}{
			"command": "timeout",
			"args":    []interface{}{"/T", "10", "/NOBREAK"},
		}
	} else {
		params = map[string]interface{}{
			"command": "sleep",
			"args":    []interface{}{"10"},
		}
	}

	_, err := sc.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected context cancellation error, got nil")
	}

	if !strings.Contains(err.Error(), "context") && !strings.Contains(err.Error(), "killed") && !strings.Contains(err.Error(), "timeout") {
		t.Errorf("Expected context/killed/timeout error, got: %v", err)
	}
}

// TestShellCommand_DefaultTimeout tests that a default timeout is applied.
func TestShellCommand_DefaultTimeout(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{"test"},
		// No timeout specified - should use default (120 seconds)
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["exitCode"].(int) != 0 {
		t.Errorf("exitCode = %v, want 0", output["exitCode"])
	}
}

// TestShellCommand_RetryOnTimeout tests that commands are retried on timeout.
func TestShellCommand_RetryOnTimeout(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping retry test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	// This command sleeps longer than the timeout - will timeout and retry
	params := map[string]interface{}{
		"command":    "sleep",
		"args":       []interface{}{"5"},
		"timeout":    1, // 1 second timeout
		"retry":      2,
		"retryDelay": 0, // No delay for faster test
	}

	start := time.Now()
	_, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	if err == nil {
		t.Fatal("Expected error after all retries exhausted, got nil")
	}

	// Should fail after 3 attempts
	if !strings.Contains(err.Error(), "3 attempts") {
		t.Errorf("Expected '3 attempts' in error, got: %v", err)
	}

	// Should complete in about 3 seconds (3 attempts * 1 second timeout each)
	if duration < 2*time.Second || duration > 6*time.Second {
		t.Errorf("Unexpected duration: %v (expected ~3s)", duration)
	}
}

// TestShellCommand_RetrySucceedsAfterStall tests retry recovers after stall.
func TestShellCommand_RetrySucceedsAfterStall(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping retry test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	// Use a temp file to track attempts
	tmpFile := "/tmp/bnto_retry_test_stall"

	// Clean up before test
	params := map[string]interface{}{
		"command": "rm",
		"args":    []interface{}{"-f", tmpFile},
	}
	sc.Execute(ctx, params)

	// Command that stalls on first attempt, succeeds on second
	// First attempt: create marker file and stall
	// Second attempt: see marker file, succeed immediately
	params = map[string]interface{}{
		"command": "sh",
		"args": []interface{}{"-c", `
			if [ -f "` + tmpFile + `" ]; then
				rm "` + tmpFile + `"
				echo "success"
				exit 0
			else
				touch "` + tmpFile + `"
				echo "stalling..."
				sleep 10
			fi
		`},
		"stream":       true,
		"timeout":      30,
		"stallTimeout": 1, // Kill after 1 second
		"retry":        2,
		"retryDelay":   0,
	}

	start := time.Now()
	result, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	if !strings.Contains(stdout, "success") {
		t.Errorf("Expected 'success' in output, got: %s", stdout)
	}

	// Should complete in about 1-2 seconds (first stalls for 1s, second succeeds)
	if duration > 5*time.Second {
		t.Errorf("Took too long: %v", duration)
	}
}

// TestShellCommand_StallDetection tests that stall detection kills hung processes.
func TestShellCommand_StallDetection(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping stall detection test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	var outputLines []string
	var mu sync.Mutex
	onOutput := func(line string) {
		mu.Lock()
		outputLines = append(outputLines, line)
		mu.Unlock()
	}

	// Command that outputs once then stalls
	params := map[string]interface{}{
		"command": "sh",
		"args": []interface{}{"-c", `
			echo "Starting..."
			sleep 10
			echo "This should not appear"
		`},
		"stream":       true,
		"_onOutput":    onOutput,
		"timeout":      30,          // Long overall timeout
		"stallTimeout": 2,           // Kill after 2 seconds of no output
		"retry":        0,           // No retries for this test
	}

	start := time.Now()
	_, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	// Should fail due to stall
	if err == nil {
		t.Fatal("Expected stall detection error, got nil")
	}

	if !strings.Contains(err.Error(), "stalled") {
		t.Errorf("Expected 'stalled' in error, got: %v", err)
	}

	// Should complete in about 2-3 seconds (stallTimeout), not 10 seconds
	if duration > 5*time.Second {
		t.Errorf("Stall detection took too long: %v", duration)
	}
}

// TestShellCommand_StallDetectionWithRetry tests stall detection combined with retry.
func TestShellCommand_StallDetectionWithRetry(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping stall detection test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	// Command that always stalls - with retry=2, should attempt 3 times
	params := map[string]interface{}{
		"command": "sh",
		"args": []interface{}{"-c", `
			echo "Attempt..."
			sleep 10
		`},
		"stream":       true,
		"timeout":      30,
		"stallTimeout": 1, // Kill after 1 second of no output
		"retry":        2, // Retry 2 times (3 total attempts)
		"retryDelay":   0, // No delay for faster test
	}

	start := time.Now()
	_, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	// Should fail after all retries
	if err == nil {
		t.Fatal("Expected error after retries exhausted, got nil")
	}

	if !strings.Contains(err.Error(), "3 attempts") {
		t.Errorf("Expected '3 attempts' in error, got: %v", err)
	}

	// Should complete in about 3-6 seconds (3 attempts * ~1-2 seconds each)
	if duration > 10*time.Second {
		t.Errorf("Stall detection with retry took too long: %v", duration)
	}
}

// TestShellCommand_MissingCommand tests error for missing command parameter.
func TestShellCommand_MissingCommand(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"args": []interface{}{"-la"},
	}

	_, err := sc.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for missing command, got nil")
	}
}

// TestShellCommand_TimeoutAsFloat tests timeout provided as float64 (JSON numbers).
func TestShellCommand_TimeoutAsFloat(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{"test"},
		"timeout": float64(30),
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["exitCode"].(int) != 0 {
		t.Errorf("exitCode = %v, want 0", output["exitCode"])
	}
}

// TestShellCommand_TimeoutAsString tests timeout provided as string (from template resolution).
func TestShellCommand_TimeoutAsString(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{"test"},
		"timeout": "30",
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["exitCode"].(int) != 0 {
		t.Errorf("exitCode = %v, want 0", output["exitCode"])
	}
}

// TestShellCommand_ExtractIntFloat tests extractInt with float64 values.
func TestShellCommand_ExtractIntFloat(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command":      "echo",
		"args":         []interface{}{"test"},
		"retry":        float64(2),
		"retryDelay":   float64(0),
		"stallTimeout": "5",
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	if output["exitCode"].(int) != 0 {
		t.Errorf("exitCode = %v, want 0", output["exitCode"])
	}
}

// TestShellCommand_RetryWithDelay tests retry with non-zero delay.
func TestShellCommand_RetryWithDelay(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping retry test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	var outputLines []string
	var mu sync.Mutex
	onOutput := func(line string) {
		mu.Lock()
		outputLines = append(outputLines, line)
		mu.Unlock()
	}

	// Command that always times out — with retry=1 and delay=1
	params := map[string]interface{}{
		"command":    "sleep",
		"args":       []interface{}{"10"},
		"timeout":    1,
		"retry":      1,
		"retryDelay": 1,
		"_onOutput":  onOutput,
	}

	start := time.Now()
	_, err := sc.Execute(ctx, params)
	duration := time.Since(start)

	if err == nil {
		t.Fatal("Expected error after retries, got nil")
	}

	// Should take at least 2 seconds (1s timeout + 1s delay + 1s timeout)
	if duration < 2*time.Second {
		t.Errorf("Expected at least 2s for retry with delay, got %v", duration)
	}
}

// TestShellCommand_NonStringArg tests error for non-string argument.
func TestShellCommand_NonStringArg(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{123},
	}

	_, err := sc.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for non-string arg, got nil")
	}
}

// TestShellCommand_StreamingWithoutCallback tests streaming mode without callback.
func TestShellCommand_StreamingWithoutCallback(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "echo",
		"args":    []interface{}{"hello"},
		"stream":  true,
		// No _onOutput callback — should fall through to buffered execution
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	if !strings.Contains(stdout, "hello") {
		t.Errorf("stdout = %q, want to contain 'hello'", stdout)
	}
}

// TestShellCommand_CommandNotFound tests execution of nonexistent command.
func TestShellCommand_CommandNotFound(t *testing.T) {
	ctx := context.Background()

	sc := shellcommand.New()

	params := map[string]interface{}{
		"command": "nonexistent-command-xyz-12345",
	}

	_, err := sc.Execute(ctx, params)
	if err == nil {
		t.Fatal("Expected error for nonexistent command, got nil")
	}
}

// TestShellCommand_NoStallWhenOutputContinues tests that active output prevents stall detection.
func TestShellCommand_NoStallWhenOutputContinues(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Skipping stall detection test on Windows")
	}

	ctx := context.Background()
	sc := shellcommand.New()

	// Command that outputs continuously - should NOT trigger stall
	params := map[string]interface{}{
		"command": "sh",
		"args": []interface{}{"-c", `
			for i in 1 2 3 4 5; do
				echo "Line $i"
				sleep 0.5
			done
		`},
		"stream":       true,
		"timeout":      30,
		"stallTimeout": 2, // 2 second stall timeout
	}

	result, err := sc.Execute(ctx, params)
	if err != nil {
		t.Fatalf("Execute failed (should not stall): %v", err)
	}

	output := result.(map[string]interface{})
	stdout := output["stdout"].(string)

	// Should have all 5 lines
	for i := 1; i <= 5; i++ {
		expected := "Line " + string('0'+byte(i))
		if !strings.Contains(stdout, expected) {
			t.Errorf("Expected '%s' in output, got: %s", expected, stdout)
		}
	}
}
