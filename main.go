package main

import (
	"fmt"
	"io"
	"os"

	"github.com/kroyoda/aoc/data"
	"github.com/kroyoda/aoc/day1"
)

func run(args []string, _ io.Reader, stdout io.Writer) error {
	switch args[0] {
	case "day1":
		instructions := day1.ParseInput(data.Input1)
		safe := day1.NewSafe()
		fmt.Fprintln(stdout, safe.Execute(instructions))
		return nil
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
