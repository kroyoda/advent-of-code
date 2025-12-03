package day3_test

import (
	"testing"

	"github.com/kroyoda/aoc/day3"
)

const input = `
987654321111111
811111111111119
234234234234278
818181911112111
`

func Test_Day3(t *testing.T) {
	t.Run("T1 - Parses input", func(t *testing.T) {
		got := day3.ParseInput(input)
		want := []day3.Bank{
			{9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1},
			{8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9},
			{2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8},
			{8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1},
		}

		if len(got) != len(want) {
			t.Fatalf("got %d banks, want %d", len(got), len(want))
		}
		for i := range got {
			for j := range got[i] {
				if got[i][j] != want[i][j] {
					t.Errorf("got bank %d index %d value %d, want %d", i, j, got[i][j], want[i][j])
				}
			}
		}
	})

	t.Run("T2 - Completes part 1", func(t *testing.T) {
		got := day3.Part1(input)
		want := int64(357)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T3 - Completes part 2", func(t *testing.T) {
		got := day3.Part2(input)
		want := int64(3121910778619)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}

func TestBank_FindMaxJoltage(t *testing.T) {
	t.Parallel()
	bank := day3.NewBankFromString("818181911112111")

	t.Run("T1 - Finds max joltage with 2 batteries", func(t *testing.T) {
		got := bank.FindMaxJoltage(2)
		want := int64(92)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T2 - Finds max joltage with 3 batteries", func(t *testing.T) {
		got := bank.FindMaxJoltage(3)
		want := int64(921)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T3 - Finds max joltage with 12 batteries", func(t *testing.T) {
		got := bank.FindMaxJoltage(12)
		want := int64(888911112111)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}
