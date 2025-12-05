package day5

import (
	"fmt"
	"strings"

	"github.com/kroyoda/aoc/utils"
)

type Ranges [][2]int64

func (r Ranges) All() int {
	var sum int
	for _, rg := range MergeAll(r) {
		if rg[0] == rg[1] && rg[0] == 0 {
			continue
		}
		sum += int(rg[1] - rg[0] + 1)
	}
	return sum
}

func (r Ranges) Fn() func(int64) bool {
	return func(val int64) bool {
		for _, rg := range r {
			if val >= rg[0] && val <= rg[1] {
				return true
			}
		}
		return false
	}
}

func ShouldMerge(r1, r2 [2]int64) bool {
	return r1[1]+1 >= r2[0] && r2[1]+1 >= r1[0]
}

func Merge(r1, r2 [2]int64) [2]int64 {
	return [2]int64{min(r1[0], r2[0]), max(r1[1], r2[1])}
}

func MergeAll(ranges Ranges) Ranges {
	result := make(Ranges, len(ranges))
	for {
		toMerge := make([]bool, len(ranges))
		merged := false
		idx := 0
		for i := 0; i < len(ranges); i++ {
			if toMerge[i] {
				continue
			}
			current := ranges[i]
			for j := i + 1; j < len(ranges); j++ {
				if toMerge[j] {
					continue
				}
				if ShouldMerge(current, ranges[j]) {
					current = Merge(current, ranges[j])
					toMerge[j] = true
					merged = true
				}
			}
			result[idx] = current
			idx++
		}
		if !merged {
			return result[:idx]
		}
		ranges = result[:idx]
	}
}

func ParseInput(input string) (Ranges, []int64) {
	parts := strings.Split(input, "\n\n")
	freshLines := utils.SplitLines(parts[0])
	availableLines := utils.SplitLines(parts[1])

	var fresh Ranges
	for _, line := range freshLines {
		var start, end int64
		fmt.Sscanf(line, "%d-%d", &start, &end)
		fresh = append(fresh, [2]int64{start, end})
	}

	var available []int64
	for _, line := range availableLines {
		var val int64
		fmt.Sscanf(line, "%d", &val)
		available = append(available, val)
	}

	return fresh, available
}

func Part1(input string) int {
	fresh, available := ParseInput(input)
	checkFn := fresh.Fn()

	var spoiled []int64
	for _, val := range available {
		if !checkFn(val) {
			spoiled = append(spoiled, val)
		}
	}
	return len(available) - len(spoiled)
}

func Part2(input string) int {
	fresh, _ := ParseInput(input)
	return fresh.All()
}
