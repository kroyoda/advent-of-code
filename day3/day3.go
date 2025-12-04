package day3

import (
	"strconv"

	"github.com/kroyoda/aoc/utils"
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

func ParseInput(input string) []Bank {
	var banks []Bank

	for _, line := range utils.SplitLines(input) {
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
	var lastIndex int

	for i := nBatteries; i > 0; i-- {
		subset := b.Values()[lastIndex : b.Size()-(i-1)]
		index, value := findMax(subset)

		maxJoltage = maxJoltage*10 + value
		lastIndex += index + 1
	}

	return maxJoltage
}

func Part1(input string) int64 {
	var sum int64

	for _, bank := range ParseInput(input) {
		sum += bank.FindMaxJoltage(2)
	}
	return sum
}

func Part2(input string) int64 {
	var sum int64

	for _, bank := range ParseInput(input) {
		sum += bank.FindMaxJoltage(12)
	}
	return sum
}
