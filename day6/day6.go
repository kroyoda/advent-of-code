package day6

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/kroyoda/aoc/utils"
)

type Grid struct {
	values    []int64
	operators []rune
	width     int
}

func NewGrid(values []int64, operators []rune, width int) Grid {
	return Grid{
		values:    values,
		operators: operators,
		width:     width,
	}
}

func (g Grid) Equal(other Grid) bool {
	if g.width != other.width {
		return false
	}
	if len(g.values) != len(other.values) {
		return false
	}
	for i := range g.values {
		if g.values[i] != other.values[i] {
			return false
		}
	}
	if len(g.operators) != len(other.operators) {
		return false
	}
	for i := range g.operators {
		if g.operators[i] != other.operators[i] {
			return false
		}
	}
	return true
}

func (g Grid) MakeOperations() []int64 {
	var results []int64
	numRows := len(g.values) / g.width

	for col := 0; col < g.width; col++ {
		var colValues []int64
		for row := range numRows {
			colValues = append(colValues, g.values[row*g.width+col])
		}
		operator := g.operators[col]
		var result int64
		switch operator {
		case '+':
			for _, val := range colValues {
				result += val
			}
		case '*':
			result = 1
			for _, val := range colValues {
				result *= val
			}
		}
		results = append(results, result)
	}
	return results
}

func Part1(input string) int {
	var sum int
	var values []int64
	var operators []rune
	var width int

	lines := utils.SplitLines(input)
	lastLine := lines[len(lines)-1]

	lastLine = strings.ReplaceAll(lastLine, " ", "")
	width = len(lastLine)

	for lastLineChar := range lastLine {
		operators = append(operators, rune(lastLine[lastLineChar]))
	}
	for _, line := range lines[:len(lines)-1] {
		numStrs := strings.FieldsSeq(line)
		for numStr := range numStrs {
			var val int64
			fmt.Sscanf(numStr, "%d", &val)
			values = append(values, val)
		}
	}

	grid := NewGrid(values, operators, width)
	results := grid.MakeOperations()
	for _, res := range results {
		sum += int(res)
	}
	return sum
}

type Operation rune

const (
	Addition       Operation = '+'
	Multiplication Operation = '*'
)

type Problem struct {
	startIndex, endIndex int
	operation            Operation
	values               []int64
}

func (p *Problem) AddValue(val int64) {
	if p.values == nil {
		p.values = make([]int64, 0, 1)
	}
	p.values = append(p.values, val)
}

func (p Problem) Values() []int64 {
	return p.values
}

func (p Problem) Solve() int64 {
	var result int64
	switch p.operation {
	case Addition:
		for _, val := range p.values {
			result += val
		}
	case Multiplication:
		result = 1
		for _, val := range p.values {
			result *= val
		}
	}
	return result
}

func Part2(input string) int64 {
	input = strings.ReplaceAll(input, "\r\n", "\n")
	input = strings.ReplaceAll(input, "\r", "\n")

	lines := make([]string, 0)
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		lines = append(lines, line)
	}

	operationsLine := lines[len(lines)-1]

	problems := make(map[int]Problem, 0)
	var currentProblem *Problem

	for i, char := range operationsLine {
		switch char {
		case '+', '*':
			if currentProblem != nil {
				currentProblem.endIndex = i - 1
				problems[currentProblem.startIndex] = *currentProblem
			}
			currentProblem = &Problem{
				startIndex: i,
				operation:  Operation(char),
			}
		}
		if i == len(operationsLine)-1 && currentProblem != nil {
			currentProblem.endIndex = i
			problems[currentProblem.startIndex] = *currentProblem
		}
	}

	// The lines of the input before the last line contain the values
	// but those values should be read by column and not by row.
	columns := make([]string, len(lines[0]))
	for _, line := range lines[:len(lines)-1] {
		for i, char := range line {
			columns[i] += string(char)
		}
	}

	for colIndex, colValues := range columns {
		if strings.TrimSpace(colValues) == "" {
			continue
		}

		value, err := strconv.ParseInt(strings.TrimSpace(colValues), 10, 64)
		if err != nil {
			log.Fatalf("Cannot parse value %s: %s", colValues, err.Error())
		}

		for startIndex, problem := range problems {
			if colIndex >= problem.startIndex && colIndex <= problem.endIndex {
				prob := problems[startIndex]
				prob.AddValue(value)
				problems[startIndex] = prob
			}
		}
	}

	var result int64
	for _, problem := range problems {
		result += problem.Solve()
	}
	return result
}
