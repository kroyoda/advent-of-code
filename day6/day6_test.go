package day6_test

import (
	"testing"

	"github.com/kroyoda/aoc/day6"
)

const input = `
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `

func Test_Day6(t *testing.T) {
	t.Run("T1 - Solves part 1", func(t *testing.T) {
		got := day6.Part1(input)
		want := 4277556
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T2 - Solves part 2", func(t *testing.T) {
		got := day6.Part2(input)
		want := int64(3263827)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}
