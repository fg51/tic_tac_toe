package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("hello")
	boards := createRecords()
	for _, i := range boards {
		fmt.Println(showBoard(i))
	}
}

func createRecords() [][3][3]int {
	return [][3][3]int{
		{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}},
		{{0, 2, 0}, {0, 1, 0}, {0, 0, 0}},
		{{0, 2, 0}, {0, 1, 0}, {1, 0, 0}},
		{{0, 2, 2}, {0, 1, 0}, {1, 0, 0}},
		{{1, 2, 2}, {0, 1, 0}, {1, 0, 0}},
		{{1, 2, 2}, {2, 1, 0}, {1, 0, 0}},
		{{1, 2, 2}, {2, 1, 0}, {1, 0, 1}},
	}
}

func showBoard(board [3][3]int) string {
	const format = "| %s | %s | %s |"

	xs := [3]string{}
	for j, row := range board {
		ys := [3]string{}
		for i, v := range row {
			ys[i] = fromIntToStr(v)
		}
		xs[j] = fmt.Sprintf(format, ys[0], ys[1], ys[2])
	}

	const separator = "+---+---+---+"
	return strings.Join([]string{
		"",
		separator,
		xs[0],
		separator,
		xs[1],
		separator,
		xs[2],
		separator,
		"",
	}, "\n")
}

func fromIntToStr(x int) string {
	switch x {
	case 0:
		return " "
	case 1:
		return "O"
	case 2:
		return "X"
	default:
		return " "
	}
}
