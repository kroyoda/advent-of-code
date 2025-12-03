package main

import (
	"fmt"
	"io"
	"os"
)

func run(args []string, _ io.Reader, stdout io.Writer) error {
	switch args[0] {
	default:
		return nil
	}
}

func main() {
	if err := run(os.Args[1:], os.Stdin, os.Stdout); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}
