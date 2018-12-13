package main // The main package ;)

import (
  // If your import section is longer than your code, then you didn't code
  "fmt"
  "os"
  "log"
  "bufio"
  "regexp"
  "strconv"
)

func main() {
  // Open file.  This should be a symbolic link to the actual data
  file, err := os.Open("run.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  // Define some "global" stuff
  var e            string = "."   // represents empty space
  var dub          string = "x"   // represents overlapping space
  var dubCounter   int    = 0
  var max_x, max_y int    = 1000, 1000
  var finalIndex   string
  
  // 2D map, map of maps
  colmap := make(map[int]map[int]string)
  
  // Keep track if index has been overlapped, or is overlapping
  eligible := make(map[string]bool)

  // Prepopulate grid with null characters
  for i := 0; i < max_y; i++ {
    rowmap := make(map[int]string)
    for j := 0; j < max_x; j++ {
      rowmap[j] = e
    }
    colmap[i] = rowmap
  }

  // Read file
  for scanner.Scan() {
    // Read line
    var line string = scanner.Text()

    // Define RegExes
    i, _ := regexp.CompilePOSIX(`^#([0-9]+)[[:space:]]`)
    l, _ := regexp.CompilePOSIX(`@[[:space:]]([0-9]+),`)
    t, _ := regexp.CompilePOSIX(`,([0-9]+):[[:space:]]`)
    w, _ := regexp.CompilePOSIX(`:[[:space:]]([0-9]+)x`)
    h, _ := regexp.CompilePOSIX(`x([0-9]+).*$`)

    // Find pattern matches, (stuff inside parens)
    top    := t.FindStringSubmatch(line)[1]
    left   := l.FindStringSubmatch(line)[1]
    width  := w.FindStringSubmatch(line)[1]
    index  := i.FindStringSubmatch(line)[1]
    height := h.FindStringSubmatch(line)[1]

    // Convert strings to ints
    leftNum,_   := strconv.Atoi(left)
    topNum,_    := strconv.Atoi(top)
    widthNum,_  := strconv.Atoi(width)
    heightNum,_ := strconv.Atoi(height)

    bottomNum := topNum  + heightNum
    rightNum  := leftNum + widthNum

    //fmt.Println()
    //fmt.Println("**********")
    //fmt.Println("I: " + index)
    //fmt.Println("L: " + left)
    //fmt.Println("W: " + width)
    //fmt.Println("T: " + top)
    //fmt.Println("H: " + height)
    //fmt.Println(rightNum)
    //fmt.Println(bottomNum)
    //fmt.Println("**********")

    eligible[index] = true

    // Insert and check for overlaps
    for i := 0; i < max_y; i++ {
      for j := 0; j < max_x; j++ {
        if j >= leftNum && j < rightNum && i >= topNum && i < bottomNum {
          // if there's another index there, then this is an overlap
          if colmap[i][j] != e { 
            if colmap[i][j] != dub {
              // if there is an overlap, we need to fix the bottom and top layers
              eligible[index] = false
              eligible[colmap[i][j]] = false
            }
            colmap[i][j] = dub
            //fmt.Print(dub)
          } else {
            colmap[i][j] = index
            //fmt.Print(index)
          }
        } else {
          //fmt.Print(e)
        }
      }
      //fmt.Println()
    }
  }

  //fmt.Println()
  //fmt.Println("Starting map range")

  // Counts overlaps, could be done early and save a loop
  for i := 0; i <= len(colmap); i++ {
    for j := 0; j <= len(colmap[i]); j++ {
      //fmt.Print(colmap[i][j] + " ")
      if colmap[i][j] == dub {
        dubCounter++
      }
      if eligible[colmap[i][j]] {
        finalIndex = colmap[i][j]
      }
    }
    //fmt.Println()
  }

  // This is the only thing that needs to be printed, AKA the answer
  fmt.Print("Overlapping Squares: ")
  fmt.Println(dubCounter)

  fmt.Println("Only non-overlapping index: " + finalIndex)

}
