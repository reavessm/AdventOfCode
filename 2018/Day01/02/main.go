// Find first duplicate frequencies (not frequency changes)
package main

import (
  "fmt"
  "os"
  "log"
  "bufio"
  "strconv"
)

func main() {
  
  // Open File
  file, err := os.Open("freq.txt")
  
  // Check Errors
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  // Initialize variables
  var sum = 0
  var frequencies map[int]bool = make(map[int]bool) // Essentially a set
  frequencies[sum] = true // initialize with starting value

  for true {
    file, err := os.Open("freq.txt")
  
    if err != nil {
      log.Fatal(err)
    }
  
    defer file.Close()
    scanner := bufio.NewScanner(file)

    for scanner.Scan() {
      var line string = scanner.Text()
      i,_ := strconv.Atoi(line)
      sum += i

      if _, ok := frequencies[sum]; ok {
        fmt.Println(sum)
        return
      } else {
        frequencies[sum] = true
      }
    }
  }
}
