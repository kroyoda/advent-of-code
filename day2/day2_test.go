package day2_test

import (
	"testing"

	"github.com/kroyoda/aoc/day2"
)

const input = `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124`

func Test_Day2(t *testing.T) {
	t.Run("T1 - Parses ranges", func(t *testing.T) {
		got := day2.ParseInput(input)
		want := []day2.Range{
			day2.NewRange(11, 22),
			day2.NewRange(95, 115),
			day2.NewRange(998, 1012),
			day2.NewRange(1188511880, 1188511890),
			day2.NewRange(222220, 222224),
			day2.NewRange(1698522, 1698528),
			day2.NewRange(446443, 446449),
			day2.NewRange(38593856, 38593862),
			day2.NewRange(565653, 565659),
			day2.NewRange(824824821, 824824827),
			day2.NewRange(2121212118, 2121212124),
		}

		if len(got) != len(want) {
			t.Fatalf("got %d ranges, want %d", len(got), len(want))
		}
		for i := range got {
			if got[i] != want[i] {
				t.Errorf("range %d: got %+v, want %+v", i, got[i], want[i])
			}
		}
	})

	t.Run("T2 - Detects invalid ids", func(t *testing.T) {
		tests := []struct {
			id    int64
			valid bool
		}{}

		t.Parallel()
		for _, tc := range tests {
			got := day2.Part1Validation(tc.id)
			if got != tc.valid {
				t.Errorf("ID %d: got valid=%v, want valid=%v", tc.id, got, tc.valid)
			}
		}
	})

	t.Run("T3 - Gets all invalid ids in test input", func(t *testing.T) {
		ranges := day2.ParseInput(input)
		invalidIDs := day2.VerifyRanges(ranges, day2.Part1Validation)

		if len(invalidIDs) != 8 {
			t.Errorf("got %d invalid IDs, want 8", len(invalidIDs))
		}
	})

	t.Run("T4 - Invalid ids", func(t *testing.T) {
		if day2.Part1Validation(11) != false {
			t.Errorf("ID 11 should be invalid")
		}
		if day2.Part1Validation(99) != false {
			t.Errorf("ID 99 should be invalid")
		}
		if day2.Part1Validation(115) != true {
			t.Errorf("ID 115 should be valid")
		}
		if day2.Part1Validation(1188511885) != false {
			t.Errorf("ID 1188511885 should be invalid")
		}
		if day2.Part1Validation(38593856) != true {
			t.Errorf("ID 38593856 should be valid")
		}
		if day2.Part1Validation(446449) != true {
			t.Errorf("ID 446449 should be valid")
		}
	})

	t.Run("T5 - Completes part 1", func(t *testing.T) {
		got := day2.Part1(input)
		want := int64(1227775554)
		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})

	t.Run("T6 - New validation strategy", func(t *testing.T) {
		if day2.Part2Validation(11) != false {
			t.Errorf("ID 11 should be invalid")
		}
		if day2.Part2Validation(22) != false {
			t.Errorf("ID 22 should be invalid")
		}
		if day2.Part2Validation(99) != false {
			t.Errorf("ID 99 should be invalid")
		}
		if day2.Part2Validation(111) != false {
			t.Errorf("ID 111 should be invalid")
		}
		if day2.Part2Validation(999) != false {
			t.Errorf("ID 999 should be invalid")
		}
		if day2.Part2Validation(1010) != false {
			t.Errorf("ID 1010 should be invalid")
		}
		if day2.Part2Validation(565656) != false {
			t.Errorf("ID 565656 should be invalid")
		}
		if day2.Part2Validation(1188511885) != false {
			t.Errorf("ID 1188511885 should be invalid")
		}
		if day2.Part2Validation(95) != true {
			t.Errorf("ID 95 should be valid")
		}
		if day2.Part2Validation(222221) != true {
			t.Errorf("ID 222221 should be valid")
		}
		if day2.Part2Validation(565456) != true {
			t.Errorf("ID 565456 should be valid")
		}
		if day2.Part2Validation(12) != true {
			t.Errorf("ID 12 should be valid")
		}
		if day2.Part2Validation(998) != true {
			t.Errorf("ID 998 should be valid")
		}
		if day2.Part2Validation(1012) != true {
			t.Errorf("ID 1012 should be valid")
		}
	})

	t.Run("T7 - Gets all invalid ids in test input with new strategy", func(t *testing.T) {
		ranges := day2.ParseInput(input)
		invalidIDs := day2.VerifyRanges(ranges, day2.Part2Validation)

		if len(invalidIDs) != 13 {
			t.Errorf("got %d invalid IDs, want 13", len(invalidIDs))
		}
	})

	t.Run("T8 - Completes part 2", func(t *testing.T) {
		ranges := day2.ParseInput(input)
		invalidIDs := day2.VerifyRanges(ranges, day2.Part2Validation)

		var sum int64
		for _, id := range invalidIDs {
			sum += id
		}

		want := int64(4174379265)
		if sum != want {
			t.Errorf("got sum %d, want %d", sum, want)
		}
	})
}

func Benchmark_Day2_Part1(b *testing.B) {
	for b.Loop() {
		day2.Part1(input)
	}
}

func Benchmark_Day2_Part2(b *testing.B) {
	for b.Loop() {
		day2.Part2(input)
	}
}
