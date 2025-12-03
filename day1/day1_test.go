package day1_test

import (
	"testing"

	"github.com/kroyoda/aoc/day1"
)

var input = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

func Test_Day1(t *testing.T) {
	t.Run("T1 - Parses left or right rotations", func(t *testing.T) {
		want := []day1.Instruction{
			{"L", 68},
			{"L", 30},
			{"R", 48},
			{"L", 5},
			{"R", 60},
			{"L", 55},
			{"L", 1},
			{"L", 99},
			{"R", 14},
			{"L", 82},
		}

		got := day1.ParseInput(input)

		if len(got) != len(want) {
			t.Fatalf("got %d instructions, want %d", len(got), len(want))
		}
		for i := range got {
			if got[i] != want[i] {
				t.Errorf("instruction %d: got %+v, want %+v", i, got[i], want[i])
			}
		}
	})

	t.Run("T2 - Wraps at 99 and 0", func(t *testing.T) {
		safe := day1.NewSafe()
		safe.Turn("L", 51)
		if safe.Dial() != 99 {
			t.Errorf("got dial at %d, want 99", safe.Dial())
		}
		safe.Turn("R", 2)
		if safe.Dial() != 1 {
			t.Errorf("got dial at %d, want 1", safe.Dial())
		}
	})

	t.Run("T3 - Resets to 50", func(t *testing.T) {
		safe := day1.NewSafe()
		safe.Turn("R", 3)
		if safe.Dial() != 53 {
			t.Errorf("got dial at %d, want 53", safe.Dial())
		}
		safe.Reset()
		if safe.Dial() != 50 {
			t.Errorf("got dial at %d, want 50", safe.Dial())
		}
	})

	t.Run("T4 - Executes a series of instructions", func(t *testing.T) {
		safe := day1.NewSafe()
		instructions := []day1.Instruction{
			{"R", 10},
			{"L", 20},
			{"R", 15},
		}
		safe.Execute(instructions)
		if safe.Dial() != 55 {
			t.Errorf("got dial at %d, want 55", safe.Dial())
		}
	})

	t.Run("T5 - Returns the number of times the dial points to 0", func(t *testing.T) {
		safe := day1.NewSafe()
		got := safe.Execute([]day1.Instruction{
			{"L", 1000},
		})
		want := 10

		if got != want {
			t.Errorf("got %d, want %d", got, want)
		}
	})
}

func Benchmark_Day1(b *testing.B) {
	instructions := day1.ParseInput(input)
	safe := day1.NewSafe()
	for b.Loop() {
		safe.Execute(instructions)
	}
}
