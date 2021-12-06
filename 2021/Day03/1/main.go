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
	"sync"
)

const (
	filename = "../input.txt"
)

var (
	firstChar  []string
	secondChar []string
	thirdChar  []string
	fourthChar []string
)

func readFile(r io.Reader) error {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		v := scanner.Text()
		firstChar = append(firstChar, v[0:1])
		secondChar = append(secondChar, v[1:2])
		thirdChar = append(thirdChar, v[2:3])
		fourthChar = append(fourthChar, v[3:4])
	}
	return nil
}

func getBinary() []uint64 {
	binary := []string{
		"1001",
		"1101",
		"11101",
	}
	var hex []uint64
	for _, v := range binary {
		h, err := strconv.ParseUint(v, 2, 10)
		if err != nil {
			log.Fatalln(err)
		}
		hex = append(hex, h)
	}
	return hex
}

func b2i(b bool) int8 {
	if b {
		return 1
	}
	return 0
}

func mostCommon() {
	binary := []string{
		"1001",
		"1101",
		"11101",
	}
	fmt.Printf("%s\n", string(binary[1][2]))
	first, second, third, fourth := 0, 0, 0, 0
	for _, v := range binary {
		firstBit := v[0:1]
		if firstBit == "0" {
			first--
		} else {
			first++
		}

		secondBit := v[1:2]
		if secondBit == "0" {
			second--
		} else {
			second++
		}

		thirdBit := v[2:3]
		if thirdBit == "0" {
			third--
		} else {
			third++
		}

		fourthBit := v[3:4]
		if fourthBit == "0" {
			fourth--
		} else {
			fourth++
		}
	}
	v := fmt.Sprintf("%d%d%d%d", b2i(first >= 1), b2i(second >= 1), b2i(third >= 1), b2i(fourth >= 1))
	h, err := strconv.ParseUint(v, 2, 4)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(v)
	fmt.Println(h)
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

func main() {
	reader, err := os.Open(filename)
	if err != nil {
		log.Fatalln(err)
	}
	err = readFile(reader)
	if err != nil {
		log.Fatalln(err)
	}
	wg := new(sync.WaitGroup)
	var firstResult string
	for i := 0; i < 1; i++ {
		wg.Add(1)
		go findMostCommon(wg, firstChar, &firstResult)
	}

	wg.Wait()
	fmt.Println(firstResult)
}
