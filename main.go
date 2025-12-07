package main

import (
	"errors"
	"fmt"
	"io"
	"os"

	"github.com/kroyoda/aoc/data"
	"github.com/kroyoda/aoc/day1"
	"github.com/kroyoda/aoc/day2"
	"github.com/kroyoda/aoc/day3"
	"github.com/kroyoda/aoc/day4"
	"github.com/kroyoda/aoc/day5"
	"github.com/kroyoda/aoc/day6"
)

var (
	ErrInvalidArgs = errors.New("invalid arguments")
	ErrUnknownDay  = errors.New("unknown day")
)

func run(args []string, _ io.Reader, stdout io.Writer) error {
	if len(args) < 1 {
		fmt.Fprintf(stdout, "Please provide a day")
		return ErrInvalidArgs
	}

	switch args[0] {
	case "day1":
		fmt.Fprintf(stdout, "Part1: %d\n", day1.Part1(data.Input1))
		fmt.Fprintf(stdout, "Part2: %d\n", day1.Part2(data.Input1))
	case "day2":
		fmt.Fprintf(stdout, "Part1: %d\n", day2.Part1(data.Input2))
		fmt.Fprintf(stdout, "Part2: %d\n", day2.Part2(data.Input2))
	case "day3":
		fmt.Fprintf(stdout, "Part1: %d\n", day3.Part1(data.Input3))
		fmt.Fprintf(stdout, "Part2: %d\n", day3.Part2(data.Input3))
	case "day4":
		fmt.Fprintf(stdout, "Part1: %d\n", day4.Part1(data.Input4))
		fmt.Fprintf(stdout, "Part2: %d\n", day4.Part2(data.Input4))
	case "day5":
		fmt.Fprintf(stdout, "Part1: %d\n", day5.Part1(data.Input5))
		fmt.Fprintf(stdout, "Part2: %d\n", day5.Part2(data.Input5))
	case "day6":
		fmt.Fprintf(stdout, "Part1: %d\n", day6.Part1(data.Input6))
		fmt.Fprintf(stdout, "Part2: %d\n", day6.Part2(data.Input6))
	default:
		return errors.Join(ErrUnknownDay, fmt.Errorf("day: %s", args[0]))
	}
	return nil
}

func main() {
	if err := run(os.Args[1:], os.Stdin, os.Stdout); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}
