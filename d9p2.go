package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
	"slices"
)

 
func main() {
	buf, err := os.ReadFile("input")

	if err != nil {
		fmt.Println("failed to read file", err)
		return
	}

	points := [][]int{}

	for line := range strings.SplitSeq(string(buf), "\n") {
		point := strings.Split(line, ",")
		if len(point) < 2 {
			continue
		}
		a, _ := strconv.Atoi(point[0])
		b, _ := strconv.Atoi(point[1])
		points = append(points, []int{a, b})
	}

	n := len(points)
	edges := [][]int{}

	for i := 0; i < n; i++ {
		a := points[i];
		b := points[(i + 1) % n];

		sortedIs := []int{a[0], b[0]}
		sortedJs := []int{a[1], b[1]}
		slices.Sort(sortedIs)
		slices.Sort(sortedJs)
		
		edges = append(edges, []int{
			sortedIs[0],
			sortedIs[1],
			sortedJs[0],
			sortedJs[1],
			a[0],
			a[1],
			b[0],
			b[1],
		})
	}

	ans := 0

	for i := 0; i < n - 1; i++ {
		for j := i + 1; j < n; j++ {
			a := points[i]
			b := points[j]

			minI := min(a[0], b[0])
			minJ := min(a[1], b[1])
			maxI := max(a[0], b[0])
			maxJ := max(a[1], b[1])

			ok := true

			for _, edge := range edges {
				edgeMinI := edge[0]
				edgeMaxI := edge[1]
				edgeMinJ := edge[2]
				edgeMaxJ := edge[3]

				if a[0] == 7 && a[1] == 1 && b[0] == 2 && b[1] == 3 {
					fmt.Println(a, b, minI < edgeMaxI && maxI > edgeMinI && minJ < edgeMaxJ && maxJ > edgeMinJ)
				}
				if minI < edgeMaxI && maxI > edgeMinI && minJ < edgeMaxJ && maxJ > edgeMinJ {
					fmt.Println(points[i], points[j], edge[4:])
					ok = false
					break
				}
			}
			if !ok {
				continue
			}
			ans = max(ans, (maxI - minI + 1) * (maxJ - minJ + 1))
		}
	}

	fmt.Println(ans)
}
