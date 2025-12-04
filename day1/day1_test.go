package day1_test

import (
	"testing"

	"github.com/kroyoda/aoc/day1"
)

const input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"

func Test_Day1(t *testing.T) {
	t.Run("T1 - Parses input", func(t *testing.T) {
		got := day1.ParseInput(input)
		want := []day1.Instruction{
			{Turn: day1.Left, Steps: 68},
			{Turn: day1.Left, Steps: 30},
			{Turn: day1.Right, Steps: 48},
			{Turn: day1.Left, Steps: 5},
			{Turn: day1.Right, Steps: 60},
			{Turn: day1.Left, Steps: 55},
			{Turn: day1.Left, Steps: 1},
			{Turn: day1.Left, Steps: 99},
			{Turn: day1.Right, Steps: 14},
			{Turn: day1.Left, Steps: 82},
		}

		if len(got) != len(want) {
			t.Fatalf("got %d instructions, want %d", len(got), len(want))
		}
		for i := range got {
			if got[i] != want[i] {
				t.Errorf("at index %d, got %+v, want %+v", i, got[i], want[i])
			}
		}
	})

	t.Run("T2 - Solves part 1", func(t *testing.T) {
		got := day1.Part1(input)
		want := 3
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T3 - Solves part 2", func(t *testing.T) {
		got := day1.Part2(input)
		want := 6
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}
