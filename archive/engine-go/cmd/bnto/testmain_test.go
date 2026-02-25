package main_test

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

func TestMain(m *testing.M) {
	binDir, err := os.MkdirTemp("", "bnto-test-bin-*")
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to create temp dir: %v\n", err)
		os.Exit(1)
	}
	defer os.RemoveAll(binDir)

	binPath := filepath.Join(binDir, "bnto")
	build := exec.Command("go", "build", "-o", binPath, ".")
	build.Stderr = os.Stderr
	if err := build.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "failed to build bnto binary: %v\n", err)
		os.Exit(1)
	}

	os.Setenv("PATH", binDir+string(os.PathListSeparator)+os.Getenv("PATH"))
	os.Exit(m.Run())
}
