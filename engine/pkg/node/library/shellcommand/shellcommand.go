// Package shellcommand provides shell command execution for the bento workflow system.
//
// The shellcommand node allows you to execute shell commands and capture their output.
// This is CRITICAL for Phase 8 (Blender automation) which requires:
//   - Long-running commands (5-30 minute Blender renders)
//   - Streaming output (line-by-line render progress)
//   - Configurable timeouts
//   - Exit code capture
//   - Stall detection and automatic retry
//
// Example usage:
//
//	// Basic command execution
//	params := map[string]interface{}{
//	    "command": "ls",
//	    "args": []string{"-la"},
//	}
//
//	// Long-running download with stall detection and retry
//	params := map[string]interface{}{
//	    "command": "yt-dlp",
//	    "args": []string{"--cookies-from-browser", "chrome", "URL"},
//	    "timeout": 10800,     // 3 hours total timeout
//	    "stream": true,       // Enable line-by-line output streaming
//	    "retry": 10,          // Retry up to 10 times on failure/stall
//	    "retryDelay": 5,      // Wait 5 seconds between retries
//	    "stallTimeout": 120,  // Kill if no output for 2 minutes
//	}
//
// The result contains:
//   - stdout: Command standard output
//   - stderr: Command standard error
//   - exitCode: Process exit code
//
// Learn more about Go's os/exec package: https://pkg.go.dev/os/exec
package shellcommand

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"os/exec"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/Develonaut/bento/pkg/node"
)

// ErrStallDetected is returned when the process produces no output for stallTimeout seconds.
var ErrStallDetected = errors.New("process stalled: no output received within timeout")

const (
	// DefaultTimeout is the default command timeout in seconds.
	// Set to 2 minutes for typical commands, but can be overridden
	// for long-running operations like Blender renders (30+ minutes).
	DefaultTimeout = 120

	// DefaultStallTimeout is the default stall detection timeout in seconds.
	// If no output is received for this duration, the process is killed.
	// Set to 0 to disable stall detection (default).
	DefaultStallTimeout = 0

	// DefaultRetry is the default number of retry attempts.
	// Set to 0 for no retries (default).
	DefaultRetry = 0

	// DefaultRetryDelay is the default delay between retries in seconds.
	DefaultRetryDelay = 5
)

// ShellCommandNeta implements shell command execution.
type ShellCommandNeta struct{}

// commandParams holds extracted and validated command parameters.
type commandParams struct {
	command      string
	args         []string
	timeout      int
	stream       bool
	onOutput     func(string)
	retry        int // Number of retry attempts (0 = no retries)
	retryDelay   int // Delay between retries in seconds
	stallTimeout int // Kill process if no output for this many seconds (0 = disabled)
}

// New creates a new shellcommand node instance.
func New() node.Executable {
	return &ShellCommandNeta{}
}

// Execute runs a shell command based on the provided parameters.
//
// Parameters:
//   - command (string, required): The command to execute
//   - args ([]interface{}, optional): Command arguments
//   - timeout (int, optional): Timeout in seconds (default: 120)
//   - stream (bool, optional): Enable line-by-line output streaming
//   - _onOutput (func(string), optional): Callback for streaming output
//   - retry (int, optional): Number of retry attempts (default: 0)
//   - retryDelay (int, optional): Seconds between retries (default: 5)
//   - stallTimeout (int, optional): Kill if no output for N seconds (default: 0 = disabled)
//
// Returns a map with:
//   - stdout (string): Standard output
//   - stderr (string): Standard error
//   - exitCode (int): Exit code (0 = success)
func (s *ShellCommandNeta) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	cmdParams, err := s.extractCommandParams(params)
	if err != nil {
		return nil, err
	}

	// Debug: log stall detection config via callback (appears in bento logs)
	if cmdParams.onOutput != nil && (cmdParams.stallTimeout > 0 || cmdParams.retry > 0) {
		debugMsg := fmt.Sprintf("[bento-shellcmd] command=%s stream=%v stallTimeout=%d retry=%d",
			cmdParams.command, cmdParams.stream, cmdParams.stallTimeout, cmdParams.retry)
		cmdParams.onOutput(debugMsg)
	}

	return s.executeWithRetry(ctx, cmdParams)
}

// executeWithRetry executes the command with optional retry logic.
func (s *ShellCommandNeta) executeWithRetry(ctx context.Context, cmdParams *commandParams) (interface{}, error) {
	var lastErr error
	maxAttempts := cmdParams.retry + 1 // retry=0 means 1 attempt, retry=10 means 11 attempts

	for attempt := 0; attempt < maxAttempts; attempt++ {
		// Check if context was cancelled before starting
		if ctx.Err() != nil {
			return nil, ctx.Err()
		}

		// Wait before retry (not on first attempt)
		if attempt > 0 && cmdParams.retryDelay > 0 {
			if cmdParams.onOutput != nil {
				retryMsg := fmt.Sprintf("[bento] RETRY attempt %d/%d after %ds delay",
					attempt+1, maxAttempts, cmdParams.retryDelay)
				cmdParams.onOutput(retryMsg)
			}
			select {
			case <-ctx.Done():
				return nil, ctx.Err()
			case <-time.After(time.Duration(cmdParams.retryDelay) * time.Second):
			}
		}

		result, err := s.executeSingleAttempt(ctx, cmdParams, attempt+1, maxAttempts)
		if err == nil {
			return result, nil
		}

		lastErr = err

		// Don't retry on context cancellation from parent
		if ctx.Err() != nil {
			return nil, err
		}
	}

	return nil, fmt.Errorf("command failed after %d attempts: %w", maxAttempts, lastErr)
}

// executeSingleAttempt executes one attempt of the command.
func (s *ShellCommandNeta) executeSingleAttempt(ctx context.Context, cmdParams *commandParams, attempt, maxAttempts int) (interface{}, error) {
	cmdCtx, cancel := context.WithTimeout(ctx, time.Duration(cmdParams.timeout)*time.Second)
	defer cancel()

	cmd := exec.CommandContext(cmdCtx, cmdParams.command, cmdParams.args...)

	// Use stall-aware streaming if stallTimeout is set and streaming is enabled
	if cmdParams.stream && cmdParams.stallTimeout > 0 {
		return s.executeStreamingWithStallDetection(cmdCtx, cmd, cmdParams, cancel)
	}

	if cmdParams.stream && cmdParams.onOutput != nil {
		return s.executeStreaming(cmdCtx, cmd, cmdParams)
	}
	return s.executeBuffered(cmdCtx, cmd, cmdParams.timeout)
}

// extractCommandParams extracts and validates command parameters from the params map.
func (s *ShellCommandNeta) extractCommandParams(params map[string]interface{}) (*commandParams, error) {
	command, ok := params["command"].(string)
	if !ok {
		return nil, fmt.Errorf("command parameter is required and must be a string")
	}

	args, err := s.extractArgs(params)
	if err != nil {
		return nil, err
	}

	timeout := s.extractTimeout(params)
	stream, _ := params["stream"].(bool)

	var onOutput func(string)
	if callback, ok := params["_onOutput"].(func(string)); ok {
		onOutput = callback
	}

	return &commandParams{
		command:      command,
		args:         args,
		timeout:      timeout,
		stream:       stream,
		onOutput:     onOutput,
		retry:        s.extractInt(params, "retry", DefaultRetry),
		retryDelay:   s.extractInt(params, "retryDelay", DefaultRetryDelay),
		stallTimeout: s.extractInt(params, "stallTimeout", DefaultStallTimeout),
	}, nil
}

// extractInt extracts an int value from params, handling int, float64, and string types.
func (s *ShellCommandNeta) extractInt(params map[string]interface{}, key string, defaultVal int) int {
	if v, ok := params[key].(int); ok {
		return v
	}
	if v, ok := params[key].(float64); ok {
		return int(v)
	}
	if v, ok := params[key].(string); ok {
		if i, err := strconv.Atoi(v); err == nil {
			return i
		}
	}
	return defaultVal
}

// extractArgs extracts and validates command arguments.
func (s *ShellCommandNeta) extractArgs(params map[string]interface{}) ([]string, error) {
	argsRaw, ok := params["args"].([]interface{})
	if !ok {
		return nil, nil
	}

	args := make([]string, len(argsRaw))
	for i, arg := range argsRaw {
		strArg, ok := arg.(string)
		if !ok {
			return nil, fmt.Errorf("all args must be strings, got %T at index %d", arg, i)
		}
		args[i] = strArg
	}
	return args, nil
}

// extractTimeout extracts timeout value, handling int, float64, and string from templates.
func (s *ShellCommandNeta) extractTimeout(params map[string]interface{}) int {
	if t, ok := params["timeout"].(int); ok {
		return t
	}
	if t, ok := params["timeout"].(float64); ok {
		return int(t)
	}
	// Handle string values from template resolution
	if t, ok := params["timeout"].(string); ok {
		if timeout, err := strconv.Atoi(t); err == nil {
			return timeout
		}
	}
	return DefaultTimeout
}

// executeStreaming runs a command with streaming output.
func (s *ShellCommandNeta) executeStreaming(cmdCtx context.Context, cmd *exec.Cmd, params *commandParams) (interface{}, error) {
	var stdoutBuilder, stderrBuilder strings.Builder

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to create stdout pipe: %w", err)
	}

	stderrPipe, err := cmd.StderrPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to create stderr pipe: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("failed to start command: %w", err)
	}

	s.streamOutput(stdoutPipe, &stdoutBuilder, params.onOutput)
	s.streamOutput(stderrPipe, &stderrBuilder, nil)

	err = cmd.Wait()
	return s.handleCommandResult(cmdCtx, err, &stdoutBuilder, &stderrBuilder, params.timeout)
}

// streamOutput reads from a pipe line-by-line and optionally calls a callback.
func (s *ShellCommandNeta) streamOutput(pipe io.ReadCloser, builder *strings.Builder, callback func(string)) {
	scanner := bufio.NewScanner(pipe)
	go func() {
		for scanner.Scan() {
			line := scanner.Text()
			builder.WriteString(line)
			builder.WriteString("\n")
			if callback != nil {
				callback(line)
			}
		}
	}()
}

// executeStreamingWithStallDetection runs a command with stall detection.
// If no output is received for stallTimeout seconds, the process is killed.
func (s *ShellCommandNeta) executeStreamingWithStallDetection(
	cmdCtx context.Context,
	cmd *exec.Cmd,
	params *commandParams,
	cancel context.CancelFunc,
) (interface{}, error) {
	var stdoutBuilder, stderrBuilder strings.Builder
	var mu sync.Mutex

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to create stdout pipe: %w", err)
	}

	stderrPipe, err := cmd.StderrPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to create stderr pipe: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("failed to start command: %w", err)
	}

	// Channel to signal activity (output received)
	activity := make(chan struct{}, 1)
	done := make(chan struct{})
	var stalled bool

	// Stream stdout with activity signaling
	go s.streamOutputWithActivity(stdoutPipe, &stdoutBuilder, params.onOutput, activity, &mu)
	// Stream stderr (also signals activity)
	go s.streamOutputWithActivity(stderrPipe, &stderrBuilder, nil, activity, &mu)

	// Stall detection goroutine
	go func() {
		stallDuration := time.Duration(params.stallTimeout) * time.Second
		timer := time.NewTimer(stallDuration)
		defer timer.Stop()

		activityCount := 0
		for {
			select {
			case <-done:
				return
			case <-activity:
				activityCount++
				// Reset timer on activity
				if !timer.Stop() {
					select {
					case <-timer.C:
					default:
					}
				}
				timer.Reset(stallDuration)
			case <-timer.C:
				// Stall detected - kill the process
				mu.Lock()
				stalled = true
				mu.Unlock()
				// Log stall detection via callback (appears in bento logs)
				if params.onOutput != nil {
					stallMsg := fmt.Sprintf("[bento] STALL DETECTED after %d seconds (received %d activity signals before stall)", params.stallTimeout, activityCount)
					params.onOutput(stallMsg)
				}
				cancel() // Cancel the context to kill the process
				return
			}
		}
	}()

	// Wait for command to complete
	err = cmd.Wait()
	close(done)

	mu.Lock()
	wasStalled := stalled
	mu.Unlock()

	if wasStalled {
		return nil, fmt.Errorf("%w after %d seconds", ErrStallDetected, params.stallTimeout)
	}

	return s.handleCommandResult(cmdCtx, err, &stdoutBuilder, &stderrBuilder, params.timeout)
}

// streamOutputWithActivity reads from a pipe and signals activity on any data received.
// Uses byte-level reading to detect activity from programs that use \r for progress updates.
func (s *ShellCommandNeta) streamOutputWithActivity(
	pipe io.ReadCloser,
	builder *strings.Builder,
	callback func(string),
	activity chan<- struct{},
	mu *sync.Mutex,
) {
	buf := make([]byte, 4096)
	var lineBuffer strings.Builder

	for {
		n, err := pipe.Read(buf)
		if n > 0 {
			// Signal activity on ANY data received (non-blocking)
			select {
			case activity <- struct{}{}:
			default:
			}

			chunk := string(buf[:n])

			mu.Lock()
			builder.WriteString(chunk)
			mu.Unlock()

			// If we have a callback, buffer and emit complete lines
			if callback != nil {
				lineBuffer.WriteString(chunk)
				content := lineBuffer.String()

				// Process complete lines (split on \n or \r)
				for {
					idx := strings.IndexAny(content, "\n\r")
					if idx == -1 {
						break
					}
					line := content[:idx]
					if len(line) > 0 {
						callback(line)
					}
					// Skip the delimiter and any following \n after \r
					content = content[idx+1:]
					if len(content) > 0 && content[0] == '\n' {
						content = content[1:]
					}
				}
				lineBuffer.Reset()
				lineBuffer.WriteString(content)
			}
		}

		if err != nil {
			// Emit any remaining buffered content
			if callback != nil && lineBuffer.Len() > 0 {
				callback(lineBuffer.String())
			}
			break
		}
	}
}

// executeBuffered runs a command with buffered output.
func (s *ShellCommandNeta) executeBuffered(cmdCtx context.Context, cmd *exec.Cmd, timeout int) (interface{}, error) {
	var stdoutBuilder, stderrBuilder strings.Builder

	cmd.Stdout = &stdoutBuilder
	cmd.Stderr = &stderrBuilder

	err := cmd.Run()
	return s.handleCommandResult(cmdCtx, err, &stdoutBuilder, &stderrBuilder, timeout)
}

// handleCommandResult processes command execution results and errors.
func (s *ShellCommandNeta) handleCommandResult(cmdCtx context.Context, err error, stdout, stderr *strings.Builder, timeout int) (interface{}, error) {
	// Check context errors first, but include the original error for debugging
	if cmdCtx.Err() == context.DeadlineExceeded {
		return nil, fmt.Errorf("command timeout after %d seconds (original error: %v)", timeout, err)
	}
	if cmdCtx.Err() == context.Canceled {
		return nil, fmt.Errorf("command killed due to context cancellation (original error: %v, stderr: %s)", err, stderr.String())
	}

	exitCode := 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			return nil, fmt.Errorf("command failed: %w", err)
		}
	}

	return map[string]interface{}{
		"stdout":   stdout.String(),
		"stderr":   stderr.String(),
		"exitCode": exitCode,
	}, nil
}
