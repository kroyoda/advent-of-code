package day7

import "github.com/kroyoda/aoc/utils"

type Manifold struct {
	source    int
	splitters []map[int]bool
}

func (m Manifold) Equal(other Manifold) bool {
	if m.source != other.source {
		return false
	}
	if len(m.splitters) != len(other.splitters) {
		return false
	}
	for i := range m.splitters {
		if len(m.splitters[i]) != len(other.splitters[i]) {
			return false
		}
		for k := range m.splitters[i] {
			if !other.splitters[i][k] {
				return false
			}
		}
	}
	return true
}

func NewManifold(source int, splitters ...[]int) Manifold {
	splitterMaps := []map[int]bool{}
	for _, splitterRow := range splitters {
		splitterMap := map[int]bool{}
		for _, x := range splitterRow {
			splitterMap[x] = true
		}
		splitterMaps = append(splitterMaps, splitterMap)
	}
	return Manifold{source: source, splitters: splitterMaps}
}

func ParseInput(input string) Manifold {
	lines := utils.SplitLines(input)
	splitters := []map[int]bool{}
	var source int

	for _, line := range lines {
		splitterRow := map[int]bool{}
		for x, char := range line {
			switch char {
			case '^':
				splitterRow[x] = true
			case 'S':
				source = x
			}
		}
		if len(splitterRow) > 0 {
			splitters = append(splitters, splitterRow)
		}
	}
	return Manifold{source: source, splitters: splitters}
}

func Part1(input string) (timeSplitted int) {
	m := ParseInput(input)
	beams := map[int]bool{m.source: true}

	for _, splitterRow := range m.splitters {
		newBeams := map[int]bool{}
		for beamX := range beams {
			if splitterRow[beamX] {
				newBeams[beamX-1] = true
				newBeams[beamX+1] = true
				timeSplitted++
			} else {
				newBeams[beamX] = true
			}
		}
		beams = newBeams
	}
	return timeSplitted
}

func Part2(input string) (possiblePaths int) {
	m := ParseInput(input)
	beams := map[int]int{m.source: 1}

	for _, splitterRow := range m.splitters {
		newBeams := map[int]int{}
		for beamX, pathCount := range beams {
			if splitterRow[beamX] {
				newBeams[beamX-1] += pathCount
				newBeams[beamX+1] += pathCount
			} else {
				newBeams[beamX] += pathCount
			}
		}
		beams = newBeams
	}

	for _, pathCount := range beams {
		possiblePaths += pathCount
	}

	return possiblePaths
}
