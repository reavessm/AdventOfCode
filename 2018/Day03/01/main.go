package main

import (
  "fmt"
  "os"
  "log"
  "bufio"
  "regexp"
  "strconv"
)

func main() {
  file, err := os.Open("run.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  var e            string = "."   // represents empty space
  var dub          string = "x"   // represents overlapping space
  var dubCounter   int    = 0
  var max_x, max_y int    = 1000, 1000
  
  colmap := make(map[int]map[int]string)

    for i := 0; i < max_y; i++ {
      rowmap := make(map[int]string)
      for j := 0; j < max_x; j++ {
        rowmap[j] = e
      }
      colmap[i] = rowmap
    }

  for scanner.Scan() {
    var line string = scanner.Text()

    fmt.Println(line)
    i, _ := regexp.CompilePOSIX(`^#([1-9]+)[[:space:]]`)
    l, _ := regexp.CompilePOSIX(`@[[:space:]]([1-9]+),`)
    t, _ := regexp.CompilePOSIX(`,([1-9]+):[[:space:]]`)
    w, _ := regexp.CompilePOSIX(`:[[:space:]]([1-9]+)x`)
    h, _ := regexp.CompilePOSIX(`x([1-9]+).*$`)

    top    := t.FindAllStringSubmatch(line, -1)[0][1]
    left   := l.FindAllStringSubmatch(line, -1)[0][1]
    width  := w.FindAllStringSubmatch(line, -1)[0][1]
    index  := i.FindAllStringSubmatch(line, -1)[0][1]
    height := h.FindAllStringSubmatch(line, -1)[0][1]

    leftNum,_   := strconv.Atoi(left)
    topNum,_    := strconv.Atoi(top)
    widthNum,_  := strconv.Atoi(width)
    heightNum,_ := strconv.Atoi(height)

    rightNum  := leftNum + widthNum
    bottomNum := topNum + heightNum

    fmt.Println()
    fmt.Println("**********")
    fmt.Println("I: " + index)
    fmt.Println("L: " + left)
    fmt.Println("W: " + width)
    fmt.Println("T: " + top)
    fmt.Println("H: " + height)
    fmt.Println(rightNum)
    fmt.Println(bottomNum)
    fmt.Println("**********")

    for i := 0; i < max_y; i++ {
      for j := 0; j < max_x; j++ {
        if j >= leftNum && j < rightNum && i >= topNum && i < bottomNum {
          if colmap[i][j] != e && colmap[i][j] != dub {
            colmap[i][j] = dub
            fmt.Print(dub)
          } else {
            colmap[i][j] = index
            fmt.Print(index)
          }
        } else {
          fmt.Print(e)
          //colmap[i][j] = e
        }
      }
      fmt.Println()
    }
  }

  fmt.Println()
  fmt.Println("Starting map range")

  for i := 0; i <= len(colmap); i++ {
    for j := 0; j <= len(colmap[i]); j++ {
      fmt.Print(colmap[i][j])
      if colmap[i][j] == dub {
        dubCounter++
      }
    }
    fmt.Println()
  }

  fmt.Print("Overlapping Squares: ")
  fmt.Println(dubCounter)

}
