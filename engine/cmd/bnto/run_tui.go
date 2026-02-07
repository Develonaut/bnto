// Package main implements TUI execution for the run command.
//
// This file contains the executeTUI function which runs bnto workflows
// with log output directly to stdout.
package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/Develonaut/bnto/pkg/api"
	"github.com/Develonaut/bnto/pkg/engine"
	"github.com/Develonaut/bnto/pkg/node"
	"github.com/Develonaut/bnto/pkg/paths"
)

// executeTUI executes bnto with detailed log output to stdout.
func executeTUI(def *node.Definition) error {
	// Create registry and file logger
	p := api.DefaultRegistry()
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

	// Create engine with logger (no messenger for log output mode)
	eng := engine.New(p, logger)

	// Load slowMo delay from config
	slowMoMs := paths.LoadSlowMoDelay()
	eng.SetSlowMoDelay(time.Duration(slowMoMs) * time.Millisecond)

	// Execute bnto
	ctx, cancel := context.WithTimeout(context.Background(), timeoutFlag)
	defer cancel()

	result, err := eng.Serve(ctx, def)

	if err != nil {
		os.Exit(1)
	}

	_ = result
	return nil
}
