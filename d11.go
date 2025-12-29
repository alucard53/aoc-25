package main

import (
	"fmt"
	"os"
	"strings"
)

type void struct {}
type Info struct {
	paths   int
	hasDac  bool
	hasFft  bool
}

func dfs(
	curr string,
	graph map[string][]string,
	dp map[string]int,
	vis map[string]void,
	dest string,
) int {
	if paths, visited := dp[curr]; visited {
		return paths
	}

	vis[curr] = void{}

	paths := 0
	
	for _, next := range graph[curr] {
		if _, ok := vis[next]; ok {
			continue
		}
		if next == dest {
			paths++
		} else {
			paths += dfs(next, graph, dp, vis, dest)
		}
	}

	delete(vis, curr)
	dp[curr] = paths
	return paths
}

func getPaths(
	src,
	dest string,
	graph map[string][]string,
) int {
	return dfs(src, graph, make(map[string]int), make(map[string]void), dest)
}

func main() {
	buf, err := os.ReadFile("input")

	if err != nil {
		fmt.Println("failed to read file", err)
		return
	}

	input := strings.Trim(string(buf), "\n")
	graph := make(map[string][]string)
	edges := 0

	for line := range strings.SplitSeq(input, "\n") {
		items := strings.Split(line, ": ")
		src := items[0]
		dests := strings.Split(items[1], " ")

		if graph[src] == nil {
			graph[src] = []string{}
		}

		graph[src] = append(graph[src], dests...)
		edges += len(dests)
	}
	
	p1 := getPaths("you", "out", graph)
	p2 := getPaths("svr", "fft", graph) *
  			getPaths("fft", "dac", graph) *
  			getPaths("dac", "out", graph)

	fmt.Println(p1, p2)
}
