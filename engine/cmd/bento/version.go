// Package main implements the version command for the bento CLI.
package main

import (
	"fmt"

	"github.com/spf13/cobra"
)

var versionCmd = &cobra.Command{
	Use:     "version",
	Aliases: []string{"v"},
	Short:   "Show version information",
	Long:    `Display the current version of the bento CLI.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("bento version %s\n", version)
	},
}
