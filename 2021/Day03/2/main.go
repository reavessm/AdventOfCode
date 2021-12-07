/**
 * File: main.go
 * Written by: Stephen M. Reaves
 * Created on: Tue, 07 Dec 2021
 */

package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"sort"
	"strconv"
)

const (
	filename = "../input.txt"
)

var (
	binary []string
)

func readFile(r io.Reader) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		v := scanner.Text()

		binary = append(binary, v)
	}
}

func sortByIndex(s []string, i int) {
	sort.SliceStable(s, func(p, q int) bool {
		first, _ := strconv.Atoi(string(s[p][i]))
		second, _ := strconv.Atoi(string(s[q][i]))
		return first < second
	})
}

func findMostCommon(s []string, i int) []string {
	// Assume s is sorted
	if len(s) == 1 {
		return s
	}

	for k, v := range s {
		if string(v[i]) == "1" {
			if k > len(s)/2 {
				return s[:k]
			}
			return s[k:]
		}
	}
	return s
}

func findLeastCommon(s []string, i int) []string {
	// Assume s is sorted
	if len(s) == 1 {
		return s
	}

	for k, v := range s {
		if string(v[i]) == "1" {
			if k < len(s)/2 {
				return s[:k]
			}
			return s[k:]
		}
	}
	return s
}

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}

	readFile(reader)

	// Assuming all binary strings are the same length
	mostCommon := make([]string, len(binary))
	leastCommon := make([]string, len(binary))
	copy(mostCommon, binary)
	copy(leastCommon, binary)
	for k := range binary[0] {
		if len(leastCommon) > 1 {
			sortByIndex(leastCommon, k)
			leastCommon = findLeastCommon(leastCommon, k)
		}
		if len(mostCommon) > 1 {
			sortByIndex(mostCommon, k)
			mostCommon = findMostCommon(mostCommon, k)
		}
	}
	fmt.Println(mostCommon)
	fmt.Println(leastCommon)
	mcb, _ := strconv.ParseUint(string(mostCommon[0]), 2, len(binary[0]))
	lcb, _ := strconv.ParseUint(string(leastCommon[0]), 2, len(binary[0]))
	fmt.Println(mcb, lcb)
	fmt.Println(mcb * lcb)
}
