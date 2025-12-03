package day1

import (
	"strconv"
	"strings"
)

type Instruction struct {
	Turn  string
	Steps int
}

type Safe struct {
	dial               int
	pointedToZeroCount int
}

func NewSafe() *Safe {
	return &Safe{dial: 50, pointedToZeroCount: 0}
}

func atoi(s string) int {
	result, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return result
}

func ParseInput(input string) []Instruction {
	var instructions []Instruction
	for line := range strings.SplitSeq(input, "\n") {
		switch line[0] {
		case 'L':
			instructions = append(instructions, Instruction{"L", atoi(line[1:])})
		case 'R':
			instructions = append(instructions, Instruction{"R", atoi(line[1:])})
		default:
			continue
		}
	}
	return instructions
}

func (s *Safe) Turn(direction string, steps int) {
	switch direction {
	case "L":
		s.dial = (s.dial - steps + 100) % 100
	case "R":
		s.dial = (s.dial + steps) % 100
	}
}

func (s *Safe) Dial() int {
	return s.dial
}

func (s *Safe) Reset() {
	s.dial = 50
}

func (s *Safe) Execute(instructions []Instruction) int {
	s.Reset()
	zeroCount := 0
	for _, instr := range instructions {
		s.Turn(instr.Turn, instr.Steps)
		switch instr.Turn {
		case "L":
			for i := 1; i <= instr.Steps; i++ {
				if (s.dial+instr.Steps-i+100)%100 == 0 {
					zeroCount++
				}
			}
		case "R":
			for i := 1; i <= instr.Steps; i++ {
				if (s.dial-instr.Steps+i+100)%100 == 0 {
					zeroCount++
				}
			}
		}
	}
	return zeroCount
}
