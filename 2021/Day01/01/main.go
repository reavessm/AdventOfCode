/**
 * File: main.go
 * Written by: Stephen M. Reaves
 * Created on: Wed, 01 Dec 2021
 */

package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
)

const (
	filename = "../depths.txt"
)

func readInts(r io.Reader) ([]int, error) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanWords)

	var results []int

	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return results, err
		}
		results = append(results, x)
	}
	return results, nil
}

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}

	ints, err := readInts(reader)
	if err != nil {
		log.Fatalln(err)
	}

	prev := int(^uint(0) >> 1) // MAXINT
	count := 0
	for _, v := range ints {
		if v > prev {
			count++
		}
		prev = v
	}

	fmt.Println("Total increases is:", count)
}
