package logger

import (
	"bufio"
	"fmt"
	"io"
)

// StreamReader wraps an io.Reader and calls a callback for each line.
// Used by shell-command node to stream output from long-running processes
// like Blender renders in real-time.
//
// This is CRITICAL for Phase 8: Blender renders can take minutes or hours,
// and users need to see progress in real-time, not buffered at the end.
//
// Example usage with exec.Command:
//
//	cmd := exec.Command("blender", args...)
//	stdout, _ := cmd.StdoutPipe()
//
//	go func() {
//	    if err := logger.StreamReader(stdout, logger, func(line string) {
//	        logger.Stream(line)  // Stream to user in real-time
//	    }); err != nil {
//	        logger.Error("stream error", "error", err)
//	    }
//	}()
//
//	cmd.Run()
//
// The callback is called for EACH line as it's received, ensuring
// minimal latency between the process output and user visibility.
//
// Returns an error if the scanner encounters a read error.
func StreamReader(r io.Reader, logger *Logger, callback func(string)) error {
	scanner := bufio.NewScanner(r)

	for scanner.Scan() {
		line := scanner.Text()
		if callback != nil {
			callback(line)
		}
	}

	// Check for scanner errors (e.g., read errors, interrupted streams)
	if err := scanner.Err(); err != nil {
		return fmt.Errorf("failed to scan stream: %w", err)
	}

	return nil
}
