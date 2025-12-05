package day5_test

import (
	"testing"

	"github.com/kroyoda/aoc/day5"
)

const input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"

func Test_Day5(t *testing.T) {
	t.Run("T1 - Parses input", func(t *testing.T) {
		fresh, available := day5.ParseInput(input)
		wantFresh := day5.Ranges{{3, 5}, {10, 14}, {16, 20}, {12, 18}}
		wantAvailable := []int64{1, 5, 8, 11, 17, 32}

		if len(fresh) != len(wantFresh) {
			t.Fatalf("got %v, want %v", fresh, wantFresh)
		}
		for i := range fresh {
			if fresh[i] != wantFresh[i] {
				t.Errorf("got %v, want %v", fresh, wantFresh)
			}
		}

		if len(available) != len(wantAvailable) {
			t.Fatalf("got %v, want %v", available, wantAvailable)
		}
		for i := range available {
			if available[i] != wantAvailable[i] {
				t.Errorf("got %v, want %v", available, wantAvailable)
			}
		}
	})

	t.Run("T2 - Solves part 1", func(t *testing.T) {
		got := day5.Part1(input)
		want := 3
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T3 - Solves part 2", func(t *testing.T) {
		got := day5.Part2(input)
		want := 14
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}
