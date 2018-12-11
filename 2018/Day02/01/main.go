package main

import (
  "fmt"
  "os"
  "log"
  "bufio"
)

func main() {
  file, err := os.Open("boxIds.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  var dubs, trips int = 0, 0

  for scanner.Scan() {
    var sightings = make(map[string]int)
    var dubsBool, tripsBool bool = false, false

    var line string = scanner.Text()

    for _, c := range line {
      sightings[string(c)]++
    }

    for _, v := range sightings {
      if v == 3 {
        tripsBool = true
      } else if v == 2 {
        dubsBool = true
      }
    }
    
    if dubsBool {
      dubs++
    }

    if tripsBool {
      trips++
    }
  }

  var checkSum int = dubs * trips

  fmt.Println(checkSum)
}
