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
  file, err := os.Open("main.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  //var max_x, max_y int = 100, 100
  var max_x, max_y int = 20, 20
  var e string = "." // represents empty space
  var dub string = "x" // represents overlapping space
  var index string
  var topNum, leftNum, rightNum, bottomNum, widthNum, heightNum int
  
  //rowmap := make(map[int]string)
  colmap := make(map[int]map[int]string)

  for scanner.Scan() {
    var line string = scanner.Text()

    i, _ := regexp.Compile(`^#([1-9]+)\s`)
    l, _ := regexp.Compile(`@\s([1-9]+),`)
    t, _ := regexp.Compile(`,([1-9]+):\s`)
    w, _ := regexp.Compile(`:\s([1-9]+)x`)
    h, _ := regexp.Compile(`x([1-9]+).*$`)

    top    := t.FindAllStringSubmatch(line, -1)[0][1]
    left   := l.FindAllStringSubmatch(line, -1)[0][1]
    width  := w.FindAllStringSubmatch(line, -1)[0][1]
    height := h.FindAllStringSubmatch(line, -1)[0][1]
    
    index   = i.FindAllStringSubmatch(line, -1)[0][1]

    leftNum,_   = strconv.Atoi(left)
    topNum,_    = strconv.Atoi(top)
    widthNum,_  = strconv.Atoi(width)
    heightNum,_ = strconv.Atoi(height)

    rightNum  = leftNum + widthNum
    bottomNum = topNum + heightNum

    fmt.Println(index)
    fmt.Println(left)
    fmt.Println(width)
    fmt.Println(top)
    fmt.Println(height)

    for i := 0; i < max_y; i++ {
      rowmap := make(map[int]string)
      for j := 0; j < max_x; j++ {
        rowmap[j] = e
      }
      colmap[i] = rowmap
    }

    for i := 0; i < max_y; i++ {
      for j := 0; j < max_x; j++ {
        if j >= leftNum && j < rightNum && i >= topNum && i < bottomNum {
          if colmap[i][j] != e {
            colmap[i][j] = dub
          } else {
            colmap[i][j] = index
            fmt.Print(index)
          }
        } else {
          fmt.Print(e)
          colmap[i][j] = e
        }
      }
      fmt.Println()
    }
  }

  fmt.Println("Starting map range")

  for i := 0; i <= len(colmap); i++ {
    for j := 0; j <= len(colmap[i]); j++ {
      fmt.Print(colmap[i][j])
    }
    fmt.Println()
  }

}
