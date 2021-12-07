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
)

const (
	filename = "../input.txt"
)

func readFile(r io.Reader) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		v := scanner.Text()

		fmt.Println(v)
	}
}

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}

	readFile(reader)
}
