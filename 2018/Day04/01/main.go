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

type Line struct {
  day    int
  year   int
  hour   int
  month  int
  guard  string
  wakes  bool
  minute int
  sleeps bool
}

func main() {
  // Open file.  This should be a symbolic link to the actual data
  file, err := os.Open("run.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  // Read file
  for scanner.Scan() {
    // Read line
    var line string = scanner.Text()

    // Define RegExes
    y,_ := regexp.CompilePOSIX(`^\[([0-9]+)-`)
    M,_ := regexp.CompilePOSIX(`-([0-9]+)-`)
    d,_ := regexp.CompilePOSIX(`-([0-9]+)[[:space::]]`)
    h,_ := regexp.CompilePOSIX(`[[:space:]]([0-9]+):`)
    m,_ := regexp.CompilePOSIX(`:([0-9]+)\]`)
    s,_ := regexp.CompilePOSIX(`\][[:space:]](.*)$`)
    g,_ := regexp.CompilePOSIX(`\][[:space:]](.*)begins.*`) // Gaurd Name
    w,_ := regexp.CompilePOSIX(`\][[:space:]](falls).*`) // boolean, falls asleep
    a,_ := regexp.CompilePOSIX(`\][[:space:]](wakes).*`) // boolean, wakes up

    fmt.Println(d.FindAllStringSubmatch(line, -1)[0][0])

    // Find pattern matches, (stuff inside parens)
    year   := y.FindStringSubmatch(line)[1]
    month  := M.FindStringSubmatch(line)[1]
    day    := d.FindStringSubmatch(line)[1]
    hour   := h.FindStringSubmatch(line)[1]
    min    := m.FindStringSubmatch(line)[1]
    stuff  := s.FindStringSubmatch(line)[1]
    guard  := g.FindStringSubmatch(line)[1]
    wakes  := w.FindStringSubmatch(line) != nil
    asleep := a.FindStringSubmatch(line) != nil

    // Convert strings to ints
    yearNum,_   := strconv.Atoi(year)
    monthNum,_ := strconv.Atoi(month)
    dayNum,_ := strconv.Atoi(day)
    hourNum,_ := strconv.Atoi(hour)
    minNum,_ := strconv.Atoi(min)

    lineData := Line {
      day: dayNum,
      year: yearNum,
      hour: hourNum,
      month: monthNum,
      guard: guard,
      wakes: wakes,
      minute: minNum,
      sleeps: asleep,
    }

    fmt.Println(lineData)
    fmt.Println(stuff)
  }

}
