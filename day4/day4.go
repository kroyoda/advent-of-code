package day4

import "github.com/kroyoda/aoc/utils"

type Cell int

const (
	Empty Cell = iota
	Occupied
	Marked
)

type Grid struct {
	cells  []Cell
	width  int
	height int
}

func (g *Grid) Equals(other Grid) bool {
	if g.width != other.width || g.height != other.height {
		return false
	}
	for i := range g.cells {
		if g.cells[i] != other.cells[i] {
			return false
		}
	}
	return true
}

func (g *Grid) Get(r, c int) Cell {
	return g.cells[r*g.width+c]
}

func (g *Grid) Count(cellType Cell) int {
	count := 0
	for _, cell := range g.cells {
		if cell == cellType {
			count++
		}
	}
	return count
}

func (g *Grid) GetNeighbors(r, c int) []Cell {
	var neighbors []Cell
	directions := []struct{ dr, dc int }{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1} /*{0,0},*/, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}

	for _, d := range directions {
		nr, nc := r+d.dr, c+d.dc
		if nr >= 0 && nr < g.height && nc >= 0 && nc < g.width {
			neighbors = append(neighbors, g.Get(nr, nc))
		} else {
			neighbors = append(neighbors, Empty)
		}
	}
	return neighbors
}

func (g *Grid) MarkAccessibles() *Grid {
	for r := 0; r < g.height; r++ {
		for c := 0; c < g.width; c++ {
			if g.Get(r, c) == Empty {
				continue
			}
			neighbors := g.GetNeighbors(r, c)
			occupiedCount := 0
			for _, n := range neighbors {
				if n != Empty {
					occupiedCount++
				}
			}
			if occupiedCount < 4 {
				g.cells[r*g.width+c] = Marked
			}
		}
	}
	return g
}

func (g *Grid) RemoveMarked() *Grid {
	for i, cell := range g.cells {
		if cell == Marked {
			g.cells[i] = Empty
		}
	}
	return g
}

func NewGrid(cells []Cell, width, height int) Grid {
	return Grid{cells: cells, width: width, height: height}
}

func ParseInput(input string) Grid {
	lines := utils.SplitLines(input)

	height := len(lines)
	width := len(lines[0])
	cells := make([]Cell, width*height)

	for r, line := range lines {
		for c, ch := range line {
			switch ch {
			case '@':
				cells[r*width+c] = Occupied
			case '.':
				cells[r*width+c] = Empty
			}
		}
	}
	return NewGrid(cells, width, height)
}

func Part1(input string) int {
	grid := ParseInput(input)
	return grid.MarkAccessibles().Count(Marked)
}

func Part2(input string) int {
	grid := ParseInput(input)
	var totalMarked int
	for {
		marked := grid.MarkAccessibles().Count(Marked)
		if marked == 0 {
			break
		}
		grid.RemoveMarked()
		totalMarked += marked
	}
	return totalMarked
}
