/**
 * File: main.go
 * Written by: Stephen M. Reaves
 * Created on: Mon, 06 Dec 2021
 */

package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

const (
	filename = "../input.txt"
)

type movement struct {
	direction string
	scale     int
}

var movements []movement

func readFile(r io.Reader) error {
	fmt.Println("FOO")
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		v := strings.Split(scanner.Text(), " ")
		fmt.Println(v[1], v[0])
		scale, err := strconv.Atoi(v[1])
		if err != nil {
			return err
		}
		movements = append(movements, movement{
			direction: v[0],
			scale:     scale,
		})
	}
	return nil
}

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}
	err = readFile(reader)
	if err != nil {
		log.Fatalln(err)
	}
	forward, depth := 0, 0
	for _, v := range movements {
		switch v.direction {
		case "forward":
			forward += v.scale
		case "up":
			depth -= v.scale
		case "down":
			depth += v.scale
		}
	}
	fmt.Println("forward", forward)
	fmt.Println("depth", depth)
	fmt.Println("Total number", forward*depth)
}
