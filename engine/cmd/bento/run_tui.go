// Package main implements TUI execution for the run command.
//
// This file contains the executeTUI function which runs bento workflows
// with log output directly to stdout.
package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/tui"
	"github.com/Develonaut/bento/pkg/neta"
)

// executeTUI executes bento with detailed log output to stdout.
func executeTUI(def *neta.Definition) error {
	// Create pantry and file logger
	p := createPantry()
	logger, logFile, err := createFileLogger()
	if err != nil {
		printError(fmt.Sprintf("Warning: Failed to create log file: %v", err))
		// Continue without file logging
	}
	if logFile != nil {
		defer logFile.Close()
	}

	// Use dual logger to output detailed logs to both file and stdout
	if logger != nil {
		logger = createDualLogger(logger)
	}

	// Create chef with logger (no messenger for TUI mode)
	chef := engine.New(p, logger)

	// Load slowMo delay from config for animations
	slowMoMs := tui.LoadSlowMoDelay()
	chef.SetSlowMoDelay(time.Duration(slowMoMs) * time.Millisecond)

	// Execute bento
	ctx, cancel := context.WithTimeout(context.Background(), timeoutFlag)
	defer cancel()

	result, err := chef.Serve(ctx, def)

	if err != nil {
		// Error message is already logged by itamae
		os.Exit(1)
	}

	// Success message is already logged by itamae
	// Just exit cleanly
	_ = result
	return nil
}
