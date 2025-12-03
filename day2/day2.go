package day2

import (
	"fmt"
	"strings"
)

type Range struct {
	first int64
	final int64
}

func NewRange(first, final int64) Range {
	return Range{first: first, final: final}
}

func ParseInput(input string) []Range {
	var ranges []Range
	input = strings.ReplaceAll(input, "\n", "")
	parts := strings.SplitSeq(input, ",")
	for part := range parts {
		var first, final int64
		fmt.Sscanf(part, "%d-%d", &first, &final)
		ranges = append(ranges, NewRange(first, final))
	}

	return ranges
}

type ValidationStrategy func(id int64) bool

func Part1Validation(id int64) bool {
	str := fmt.Sprintf("%d", id)
	n_digits := len(str)
	if n_digits%2 == 0 {
		parts := str[0 : n_digits/2]
		if parts == str[n_digits/2:] {
			return false
		}
	}
	return true
}

func Part2Validation(id int64) bool {
	// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
	str := fmt.Sprintf("%d", id)
	// 1. Take one digit and check if the whole string is made of this digit repeated
	// 2. Take two digits and check if the whole string is made of this two digits repeated
	// ...
	n_digits := len(str)
	for size := 1; size <= n_digits/2; size++ {
		if n_digits%size != 0 {
			continue
		}
		pattern := str[0:size]
		repeated := true
		for i := 0; i < n_digits; i += size {
			if str[i:i+size] != pattern {
				repeated = false
				break
			}
		}
		if repeated {
			return false
		}
	}
	return true
}

func VerifyRanges(ranges []Range, strategy ValidationStrategy) []int64 {
	var invalidIDs []int64
	for _, r := range ranges {
		for id := r.first; id <= r.final; id++ {
			if !strategy(id) {
				invalidIDs = append(invalidIDs, id)
			}
		}
	}
	return invalidIDs
}

func Part1(input string) int64 {
	var sum int64
	ranges := ParseInput(input)
	invalidIDs := VerifyRanges(ranges, Part1Validation)
	for _, id := range invalidIDs {
		sum += id
	}
	return sum
}

func Part2(input string) int64 {
	var sum int64
	ranges := ParseInput(input)
	invalidIDs := VerifyRanges(ranges, Part2Validation)
	for _, id := range invalidIDs {
		sum += id
	}
	return sum
}
