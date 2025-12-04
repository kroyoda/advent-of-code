package day4_test

import (
	"testing"

	"github.com/kroyoda/aoc/day4"
)

const (
	input = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`

	wanted = `..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
`
)

func Test_Day4(t *testing.T) {
	t.Run("T1 - Parses input", func(t *testing.T) {
		got := day4.ParseInput("..@\n@@@\n@@@")
		want := day4.NewGrid([]day4.Cell{
			day4.Empty, day4.Empty, day4.Occupied,
			day4.Occupied, day4.Occupied, day4.Occupied,
			day4.Occupied, day4.Occupied, day4.Occupied,
		}, 3, 3)

		if got.Equals(want) == false {
			t.Error("parsed grid is incorrect")
		}
	})

	t.Run("T2 - Solves part 1", func(t *testing.T) {
		got := day4.Part1(input)
		want := 13

		if got != want {
			t.Errorf("part 1 unsovled, want (%d) got (%d)", want, got)
		}
	})

	t.Run("T3 - Solves part 2", func(t *testing.T) {
		got := day4.Part2(input)
		want := 43

		if got != want {
			t.Errorf("part 2 unsovled, want (%d) got (%d)", want, got)
		}
	})
}

func TestGrid_GetNeighbors(t *testing.T) {
	grid := day4.ParseInput(".@.\n@@@\n.@.")
	got := grid.GetNeighbors(1, 1)
	want := []day4.Cell{
		day4.Empty, day4.Occupied, day4.Empty,
		day4.Occupied /*center*/, day4.Occupied,
		day4.Empty, day4.Occupied, day4.Empty,
	}

	for i := range got {
		if got[i] != want[i] {
			t.Errorf("neighbor %d incorrect, want (%v) got (%v)", i, want[i], got[i])
		}
	}
}
