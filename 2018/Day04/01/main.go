package main // The main package ;)

import (
  // If your import section is longer than your code, then you didn't code
  "fmt"
  "os"
  "log"
  "bufio"
  "regexp"
  "strconv"
  "math/rand"
)

type Line struct {
  year   int
  month  int
  day    int
  hour   int
  minute int
  guard  string
  wakes  bool
  sleeps bool
}

func quicksort(a []int) []int {
  if len(a) < 2 {
    return a
  }

  left, right := 0, len(a)-1

  pivot := rand.Int() % len(a)
  //pivot := len(a)/2

  a[pivot], a[right] = a[right], a[pivot]

  for i, _ := range a {
    if a[i] < a[right] {
      a[left], a[i] = a[i], a[left]
      left++
    }
  }

  a[left], a[right] = a[right], a[left]

  quicksort(a[:left])
  quicksort(a[left+1:])
  
  return a
}

func main() {
  // Open file.  This should be a symbolic link to the actual data
  file, err := os.Open("run.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)

  indexMap := make(map[int]Line)
  var slice []int

  // Read file
  for scanner.Scan() {
    // Read line
    var line string = scanner.Text()

    // Define RegExes
    y,_ := regexp.CompilePOSIX(`^\[([0-9]+)-`)
    M,_ := regexp.CompilePOSIX(`-([0-9]+)-`)
    d,_ := regexp.CompilePOSIX(`([0-9]+)[[:space:]]`)
    h,_ := regexp.CompilePOSIX(`[[:space:]]([0-9]+):`)
    m,_ := regexp.CompilePOSIX(`:([0-9]+)\]`)
    //s,_ := regexp.CompilePOSIX(`\][[:space:]](.*)$`)
    g,_ := regexp.CompilePOSIX(`\][[:space:]](.*)begins.*`) // Gaurd Name
    a,_ := regexp.CompilePOSIX(`\][[:space:]](falls).*`) // boolean, falls asleep
    w,_ := regexp.CompilePOSIX(`\][[:space:]](wakes).*`) // boolean, wakes up

    //fmt.Println(line)
    //fmt.Println(g.FindStringSubmatch(line))

    // Find pattern matches, (stuff inside parens)
    year   := y.FindStringSubmatch(line)[1]
    month  := M.FindStringSubmatch(line)[1]
    day    := d.FindStringSubmatch(line)[1]
    hour   := h.FindStringSubmatch(line)[1]
    min    := m.FindStringSubmatch(line)[1]
    //stuff  := s.FindStringSubmatch(line)[1]
    wakes  := w.FindStringSubmatch(line) != nil
    asleep := a.FindStringSubmatch(line) != nil
    guard  := "No Guard"

    if g.FindStringSubmatch(line) != nil {
      guard = g.FindStringSubmatch(line)[1]
    } 

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

    //fmt.Println(lineData)

    index := yearNum*100000000 + monthNum*1000000 + dayNum*10000 + hourNum*100 + minNum
    indexMap[index] = lineData

    slice = append(slice, index)

    //fmt.Println(index)
    //fmt.Println()

  } // end for file

  //fmt.Println(slice)

  quicksort(slice)

  //fmt.Println(slice)

  tmpGuard := "No Guard"

  sleepMap := make(map[string]int)
  totalMap  := make(map[string]int)
  minuteCount := make(map[string]map[int]int)
  var startTime int

  for i := range slice {
  //tmpMap := make(map[int]int)
    //fmt.Println(indexMap[slice[i]])
    
    if indexMap[slice[i]].guard != "No Guard" {
      totalMap[tmpGuard] += slice[i] - startTime
      startTime = slice[i]
      tmpGuard = indexMap[slice[i]].guard
    } else {
      // If they are asleep and were previously awake
      // Redundant, but ignores garbage data
      if indexMap[slice[i]].wakes && indexMap[slice[i-1]].sleeps {
        sleepMap[tmpGuard] += slice[i] - slice[i-1]
        //fmt.Println(sleepMap[tmpGuard])
      }
      if indexMap[slice[i]].sleeps {
        //tmpMap[indexMap[slice[i]].minute] = 1
        //minuteCount[tmpGuard][indexMap[slice[i]].minute] += tmpMap[indexMap[slice[i]].minute]
        //minuteCount[tmpGuard][indexMap[slice[i]].minute] = 1
        if minuteCount[tmpGuard] == nil {
          minuteCount[tmpGuard] = make(map[int]int)
        }
        minuteCount[tmpGuard][indexMap[slice[i]].minute]++
      }

    }

  }

  //fmt.Println(sleepMap)
  //fmt.Println(totalMap)

  var chosenGuard string

  for k, v := range sleepMap {
    if float32(v) / float32((totalMap[k]+1)) > float32(sleepMap[chosenGuard]) / float32((totalMap[chosenGuard]+1)) {
      chosenGuard = k
    }
  }

  fmt.Println(chosenGuard)

  var mostMin int
  var mostMinCount int

  for k, v := range minuteCount {
    if k == chosenGuard {
      for ik, iv := range v {
        if iv > mostMinCount {
          mostMinCount = iv
          mostMin = ik
        }
      }
    }
  }

  fmt.Println(mostMin)

    a,_ := regexp.CompilePOSIX(`#([0-9]+)[[:space:]]*`)
    answer := a.FindStringSubmatch(chosenGuard)[1]
    fmt.Println(answer)
    answerNum,_ := strconv.Atoi(answer)
    answerNum *= mostMin

    fmt.Println(answerNum)
}
