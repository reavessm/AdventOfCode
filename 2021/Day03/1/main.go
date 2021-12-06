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
	"sync"
)

const (
	filename = "../input.txt"
)

var (
	mostCommonArr  []string   // Arrary holding the most common bits in their correct positioning
	leastCommonArr []string   // Arrary holding the least common bits in their correct positioning
	charArr        [][]string // The index of the first dimensions slice corresponds to the character of they binary string while the index of the second dimension corresponds to the line.  This is essentially a matrix transpositioning of the input.
)

// readFile reads binary strings from a file and stores them in a slice of slice broken up by character index
func readFile(r io.Reader) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	// iterate each line
	for scanner.Scan() {
		v := scanner.Text()

		// initizialize charArr on first run
		if len(charArr) == 0 {
			charArr = make([][]string, len(v))
		}

		// iterate each character
		for k, token := range strings.Split(v, "") {
			charArr[k] = append(charArr[k], token)
		}
	}
}

func b2i(b bool) int8 {
	if b {
		return 1
	}
	return 0
}

func findMostCommon(wg *sync.WaitGroup, s []string, result *string) {
	defer wg.Done()
	bit := 0
	for _, v := range s {
		if v == "1" {
			bit++
		} else {
			bit--
		}
	}

	*result = fmt.Sprintf("%v", b2i(bit >= 1))
}

func findLeastCommon(wg *sync.WaitGroup, s []string, result *string) {
	defer wg.Done()
	bit := 0
	for _, v := range s {
		if v == "1" {
			bit--
		} else {
			bit++
		}
	}

	*result = fmt.Sprintf("%v", b2i(bit >= 1))
}

func formatAsBinary(s []string) string {
	var result string
	for _, v := range s {
		result += v
	}
	return result
}

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}

	readFile(reader)

	// Make result slice match input slice
	mostCommonArr = make([]string, len(charArr))
	leastCommonArr = make([]string, len(charArr))

	wg := new(sync.WaitGroup)
	for i := 0; i < len(charArr); i++ {
		wg.Add(2)
		go findMostCommon(wg, charArr[i], &mostCommonArr[i])
		go findLeastCommon(wg, charArr[i], &leastCommonArr[i])
	}

	wg.Wait()

	mostCommonBit := formatAsBinary(mostCommonArr)
	leastCommonBit := formatAsBinary(leastCommonArr)

	fmt.Println("Most common: ", mostCommonBit)
	fmt.Println("Least common:", leastCommonBit)

	mcbDecimal, err := strconv.ParseUint(mostCommonBit, 2, 12)
	if err != nil {
		log.Fatalln(err)
	}
	lcbDecimal, err := strconv.ParseUint(leastCommonBit, 2, 12)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("Most common as decimal: ", mcbDecimal)
	fmt.Println("Least common as decimal:", lcbDecimal)

	fmt.Println("Answer:", lcbDecimal*mcbDecimal)
}
