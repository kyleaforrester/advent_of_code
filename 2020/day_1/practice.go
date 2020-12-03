package main

import "fmt"
import "bufio"
import "os"
import "strconv"

func main() {
    items := make([]int, 0, 100)
    sum := 0
    scanner := bufio.NewScanner(os.Stdin)
    for scanner.Scan() {
        newItem, err := strconv.Atoi(scanner.Text())
        if err != nil {
            fmt.Println(err)
        }
        items = append(items, newItem)
    }

    for i := 0; i < len(items); i++ {
        sum += fuel_req(items[i])
    }

    fmt.Println(sum)
}

func fuel_req(mass int) int {
    remaining := mass
    sum := 0
    for ; remaining > 0; {
        temp := remaining / 3 - 2
        if temp <= 0 {
            temp = 0
            remaining = 0
        } else {
            remaining -= temp
            sum += temp
        }
    }

    return sum
}
