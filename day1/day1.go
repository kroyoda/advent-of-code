package day1

import (
	"fmt"

	"github.com/kroyoda/aoc/utils"
)

type Direction string

const (
	Left  Direction = "L"
	Right Direction = "R"
)

type Instruction struct {
	Turn  Direction
	Steps int
}

type Dial struct {
	points int
}

func NewDial() *Dial {
	return &Dial{points: 50}
}

func (d *Dial) Execute(inst Instruction) {
	switch inst.Turn {
	case Left:
		d.points = (d.points - inst.Steps + 100) % 100
	case Right:
		d.points = (d.points + inst.Steps) % 100
	default:
	}
}

func (d *Dial) Points() int {
	return d.points
}

func ParseInput(input string) []Instruction {
	lines := utils.SplitLines(input)
	instructions := make([]Instruction, 0, len(lines))
	for _, line := range lines {
		var turn Direction
		switch line[0] {
		case 'L':
			turn = Left
		case 'R':
			turn = Right
		default:
			continue
		}
		var steps int
		_, err := fmt.Sscanf(line[1:], "%d", &steps)
		if err != nil {
			panic(fmt.Sprintf("invalid steps: %s", line[1:]))
		}
		instructions = append(instructions, Instruction{
			Turn:  turn,
			Steps: steps,
		})
	}
	return instructions
}

func Part1(input string) int {
	instructions := ParseInput(input)
	dial := NewDial()

	result := 0
	for _, inst := range instructions {
		dial.Execute(inst)
		if dial.Points() == 0 {
			result++
		}
	}
	return result
}

func Part2(input string) int {
	instructions := ParseInput(input)
	dial := NewDial()

	result := 0
	for _, inst := range instructions {
		switch inst.Turn {
		case Left:
			for i := 0; i < inst.Steps; i++ {
				dial.Execute(Instruction{Turn: Left, Steps: 1})
				if dial.Points() == 0 {
					result++
				}
			}
		case Right:
			for i := 0; i < inst.Steps; i++ {
				dial.Execute(Instruction{Turn: Right, Steps: 1})
				if dial.Points() == 0 {
					result++
				}
			}
		}
	}
	return result
}
