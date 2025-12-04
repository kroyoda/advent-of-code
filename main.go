package main

import (
	"fmt"
	"io"
	"os"

	"github.com/kroyoda/aoc/data"
	"github.com/kroyoda/aoc/day1"
	"github.com/kroyoda/aoc/day2"
	"github.com/kroyoda/aoc/day3"
)

func run(args []string, _ io.Reader, stdout io.Writer) error {
	if len(args) < 1 {
		return fmt.Errorf("please provide a day to run")
	}

	switch args[0] {
	case "day1":
		instructions := day1.ParseInput(data.Input1)
		safe := day1.NewSafe()
		fmt.Fprintln(stdout, safe.Execute(instructions))
		return nil
	case "day2":
		fmt.Fprintf(stdout, "Part1: %d\n", day2.Part1(data.Input2))
		fmt.Fprintf(stdout, "Part2: %d\n", day2.Part2(data.Input2))
		return nil
	case "day3":
		fmt.Fprintf(stdout, "Part1: %d\n", day3.Part1(data.Input3))
		fmt.Fprintf(stdout, "Part2: %d\n", day3.Part2(data.Input3))
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
