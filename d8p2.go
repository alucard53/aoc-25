package main

import (
	"cmp"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func getDist(a []float64, b []float64) float64 {
	return math.Sqrt(math.Pow(a[0] - b[0], 2) + 
								   math.Pow(a[1] - b[1], 2) +
									 math.Pow(a[2] - b[2], 2))
}

type Edge struct {
	src  int
	dest int
	dist float64
}

type DisjointSet struct {
	parent []int
	rank []int
}

func(ds *DisjointSet) find(a int) int {
	parent := ds.parent
	family := []int{}
	for parent[a] != a {
		family = append(family, a)
		a = parent[a]
	}
	for _, member := range family {
		parent[member] = a
	}
	return a
}

func(ds *DisjointSet) union(a int, b int) bool {
	parentA := ds.find(a)
	parentB := ds.find(b)

	if parentA == parentB {
		return false
	}

	if ds.rank[parentA] < ds.rank[parentB] {
		parentA, parentB = parentB, parentA
	} 

	ds.rank[parentA] += ds.rank[parentB]
	ds.rank[parentB] = 0 
	ds.parent[parentB] = parentA

	return ds.rank[parentA] == len(ds.rank)
}

func main() {
	buf, err := os.ReadFile("input")

	if err != nil {
		fmt.Println("got an error while reading the file", err)
		return
	}

	input := strings.Trim(string(buf), "\n")

	points := [][]float64{}
	for line := range strings.SplitSeq(input, "\n") {
		point := []float64{}
		for coordStr := range strings.SplitSeq(line, ",") {
			coord, _ := strconv.ParseFloat(coordStr, 64)
			point = append(point, coord)
		}
		points = append(points, point)
	}

	n := len(points)
	parent := make([]int, n)
	rank := make([]int, n)
	for i := range n {
		parent[i] = i
		rank[i] = 1
	}
	ds := DisjointSet{
		parent: parent,
		rank: rank,
	}
	edges := []*Edge{}
	for i := range len(points) {
		for j := i + 1; j < n; j++ {
			edges = append(edges, &Edge{
				src:  i,
				dest: j,
				dist: getDist(points[i], points[j]),
			})
		}
	}

	slices.SortFunc(edges, func(a *Edge, b *Edge) int {
		return cmp.Compare(a.dist, b.dist)
	})
	
	ans := 0

	for i := 0; i < len(edges); i++ {
		edge := edges[i]
		src := edge.src
		dest := edge.dest
		if ds.union(src, dest) {
			ans = int(points[src][0] * points[dest][0])
			break
		}
	}
	fmt.Println(ans)
}
