package day7_test

import (
	"testing"

	"github.com/kroyoda/aoc/day7"
)

const input = `
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
`

func Test_Day7(t *testing.T) {
	t.Run("T1 - Parses input", func(t *testing.T) {
		got := day7.ParseInput(input)
		want := day7.NewManifold(7, []int{7}, []int{6, 8}, []int{5, 7, 9}, []int{4, 6, 10}, []int{3, 5, 9, 11}, []int{2, 6, 12}, []int{1, 3, 5, 7, 9, 13})

		if !got.Equal(want) {
			t.Errorf("got %v, want %v", got, want)
		}
	})

	t.Run("T2 - Solves part 1", func(t *testing.T) {
		got := day7.Part1(input)
		want := 21

		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T3 - Solves part 2", func(t *testing.T) {
		got := day7.Part2(input)
		want := 40

		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}
