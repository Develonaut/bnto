// Package main implements the logs command for viewing bento execution logs.
package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"github.com/Develonaut/bento/pkg/logs"
	"github.com/Develonaut/bento/pkg/paths"
	"github.com/spf13/cobra"
)

var (
	followFlag bool
	allFlag    bool
)

var logsCmd = &cobra.Command{
	Use:   "logs",
	Short: "View bento execution logs",
	Long: `View and tail bento execution logs from ~/.bento/logs/

By default, shows the most recent log file.
Use --follow to stream logs in real-time.
Use --all to list all available log files.

Examples:
  bento logs              View most recent log
  bento logs --follow     Tail most recent log in real-time
  bento logs -f           Tail most recent log (short flag)
  bento logs --all        List all available log files`,
	RunE: runLogs,
}

func init() {
	logsCmd.Flags().BoolVarP(&followFlag, "follow", "f", false, "Follow log output in real-time (like tail -f)")
	logsCmd.Flags().BoolVarP(&allFlag, "all", "a", false, "List all available log files")
}

// runLogs executes the logs command logic.
func runLogs(cmd *cobra.Command, args []string) error {
	bentoHome := paths.LoadBentoHome()

	// Ensure logs directory exists (create if needed)
	if err := logs.EnsureLogsDirectory(bentoHome); err != nil {
		return err
	}

	logsDir, err := logs.GetLogsDirectory(bentoHome)
	if err != nil {
		return err
	}

	// List all log files if --all flag is set
	if allFlag {
		return listAllLogs(logsDir)
	}

	// Get most recent log file
	logFile, err := logs.GetMostRecentLog(logsDir)
	if err != nil {
		return err
	}

	// If --follow is set, tail the most recent log file (or wait for one)
	if followFlag {
		// If no log files exist, wait for one
		if logFile == "" {
			printInfo("No log files yet. Waiting for bento execution...")
			printInfo("(Start a bento in another terminal)")
			fmt.Println()
			return waitAndTailNewLog(logsDir)
		}

		// Log file exists - tail it (will show new content as it's appended)
		logPath := filepath.Join(logsDir, logFile)
		printInfo("Ready to stream logs. Run a bento in another terminal.")
		fmt.Println()
		return tailLog(logPath)
	}

	if logFile == "" {
		printInfo("No log files found in ~/.bento/logs/")
		printInfo("Run a bento to create logs.")
		return nil
	}

	logPath := filepath.Join(logsDir, logFile)

	// Follow log if --follow flag is set
	if followFlag {
		return tailLog(logPath)
	}

	// Otherwise just cat the log
	return catLog(logPath)
}

// listAllLogs lists all available log files with timestamps.
func listAllLogs(logsDir string) error {
	logFiles, err := logs.ListLogFiles(logsDir)
	if err != nil {
		return err
	}

	if len(logFiles) == 0 {
		printInfo("No log files found.")
		return nil
	}

	printInfo(fmt.Sprintf("Log files in ~/.bento/logs/ (%d total):", len(logFiles)))
	fmt.Println()

	for i, file := range logFiles {
		if i == 0 {
			fmt.Printf("  📄 %s (most recent)\n", file.Name)
		} else {
			fmt.Printf("  📄 %s\n", file.Name)
		}
	}

	return nil
}

// waitAndTailNewLog polls for a new log file to appear and tails it.
func waitAndTailNewLog(logsDir string) error {
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()

	for range ticker.C {
		logFile, err := logs.GetMostRecentLog(logsDir)
		if err != nil {
			continue
		}

		if logFile != "" {
			logPath := filepath.Join(logsDir, logFile)
			fmt.Printf("🍱 New log detected: %s\n\n", logFile)
			return tailLog(logPath)
		}
	}

	return nil
}

// tailLog tails a log file in real-time using tail -f.
func tailLog(logPath string) error {
	printInfo(fmt.Sprintf("Following log: %s", logPath))
	printInfo("Press Ctrl+C to stop")
	fmt.Println()

	cmd := exec.Command("tail", "-f", logPath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	return cmd.Run()
}

// catLog displays the entire log file.
func catLog(logPath string) error {
	printInfo(fmt.Sprintf("Log file: %s", logPath))
	fmt.Println()

	content, err := os.ReadFile(logPath)
	if err != nil {
		return fmt.Errorf("failed to read log file: %w", err)
	}

	fmt.Print(string(content))
	return nil
}
