// Package main implements the version command for the bnto CLI.
package main

import (
	"fmt"

	"github.com/spf13/cobra"
)

var versionCmd = &cobra.Command{
	Use:     "version",
	Aliases: []string{"v"},
	Short:   "Show version information",
	Long:    `Display the current version of the bnto CLI.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("bnto version %s\n", version)
	},
}
