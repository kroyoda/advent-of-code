package day3

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Battery struct {
	Joltage int64
	Index   int
}

type Bank []int64

func (b Bank) Size() int {
	return len(b)
}

func (b Bank) Batteries() []Battery {
	batteries := make([]Battery, len(b))
	for i, joltage := range b {
		batteries[i] = Battery{
			Joltage: joltage,
			Index:   i,
		}
	}
	return batteries
}

func (b Bank) Values() []int64 {
	values := make([]int64, len(b))
	copy(values, b)
	return values
}

func (b Bank) ValuesAtLeastSize(n int) []int64 {
	result := make([]int64, b.Size()-n)
	copy(result, b.Values()[:b.Size()-n])
	return result
}

func (b Bank) ValuesSubset(start, end int) []int64 {
	result := make([]int64, end-start)
	copy(result, b.Values()[start:end])
	return result
}

func NewBankFromString(s string) Bank {
	bank := make(Bank, len(s))
	for i, ch := range s {
		result, err := strconv.ParseInt(string(ch), 10, 64)
		if err != nil {
			panic(err)
		}
		bank[i] = result
	}
	return bank
}

func splitLines(s string) []string {
	s = strings.ReplaceAll(s, "\r\n", "\n")
	s = strings.ReplaceAll(s, "\r", "\n")
	lines := strings.Split(s, "\n")

	var result []string
	for _, line := range lines {
		if line == "" {
			continue
		}
		result = append(result, strings.TrimSpace(line))
	}
	return result
}

func ParseInput(input string) []Bank {
	var banks []Bank

	for _, line := range splitLines(input) {
		if line == "" {
			continue
		}
		banks = append(banks, NewBankFromString(line))
	}
	return banks
}

func findMax(list []int64) (index int, value int64) {
	for i, v := range list {
		if v > value {
			index = i
			value = v
		}
	}
	return index, value
}

func (b *Bank) FindMaxJoltage(nBatteries int) int64 {
	var maxJoltage int64
	var latestIndex int
	fmt.Printf("\nFinding max joltage for %d batteries\n", nBatteries)

	subset := b.Values()[latestIndex : b.Size()-nBatteries+1]

	for i := nBatteries; i > 0; i-- {
		fmt.Printf("\tFinding battery %d in subset: %v\n", nBatteries-i, subset)

		idx, value := findMax(subset)
		fmt.Printf("\t\tFound battery with joltage %d at index %d\n", value, idx)

		latestIndex += idx + 1
		fmt.Printf("\t\tLatest index is now %d\n", latestIndex)
		subset = b.Values()[latestIndex:max(b.Size(), b.Size()-i)]
		maxJoltage += int64(math.Pow10(i-1)) * value
	}
	return maxJoltage
}

func Part1(input string) int64 {
	banks := ParseInput(input)

	var total int64
	for _, bank := range banks {
		var maxJoltage int64

		batteries := bank.Batteries()
		for _, firstBattery := range batteries {
			for _, secondBattery := range batteries[firstBattery.Index:] {
				if firstBattery.Index == secondBattery.Index {
					continue
				}
				joltage := firstBattery.Joltage*10 + secondBattery.Joltage
				if joltage > maxJoltage {
					maxJoltage = joltage
				}
			}
		}

		total += maxJoltage
	}
	return total
}

func Part2(input string) int64 {
	return 0
}
