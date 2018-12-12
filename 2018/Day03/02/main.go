package main

import (
  "fmt"
  "os"
  "log"
  "bufio"
)

var max string = ""

func min(a int, b int) int {
  if a <= b {
    return a
  }
  return b
}

func setMax(_max string) {
  if len(_max) > len(max) {
    max = _max
  }
}

func main() {
  file, err := os.Open("boxIds.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)
  lines := make(map[int]string)
  count := 0

  for scanner.Scan() {
    lines[count] = scanner.Text()
    count++
  }

  for i := 0; i <= len(lines)-2; i++ {
    for j := i+1; j <= len(lines)-1; j++ {
      var maybe string = ""
      for k := 0; k <= min(len(lines[i]),len(lines[j]))-1; k++ {
        if lines[i][k] == lines[j][k] {
          maybe += string(lines[i][k])
        }
      }
      setMax(maybe)
    } // end j
  } // end i

  fmt.Println(max)
}
