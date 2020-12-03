package main

import "fmt"
import "bufio"
import "os"
import "strconv"

func main() {
    items := make([]int, 0, 100)
    scanner := bufio.NewScanner(os.Stdin)
    for scanner.Scan() {
        newItem, err := strconv.Atoi(scanner.Text())
        if err != nil {
            fmt.Println(err)
        }
        items = append(items, newItem)
    }

    comps, succ := find_complements(items, 3, 2020)

    if !succ {
        fmt.Println("No numbers exist!")
    }

    product := 1
    for i := 0; i < len(comps); i++ {
        product *= comps[i]
    }

    fmt.Println(product)
}

func test_func() []int {
    return []int{1,2,3}
}

func find_complements(list []int, count int, sum int) ([]int, bool) {
    if count == 0 && sum == 0 {
        return []int{}, true
    }
    if count == 0 && sum != 0 {
        return []int{}, false
    }

    for i := 0; i < len(list) - (count - 1); i++ {
        comps, succ := find_complements(list[i+1:], count - 1, sum - list[i])
        if succ {
            return append(comps, list[i]), true
        }
    }

    return []int{}, false
}

