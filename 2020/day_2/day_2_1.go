package main

import (
    "fmt"
    "bufio"
    "os"
    "strconv"
    "strings"
    "errors"
)

type Password struct {
    low int
    high int
    substr string
    pass string
}

func newPassword(s string) (*Password, error) {
    s_tok := strings.Split(s, " ")
    if len(s_tok) != 3 {
        return nil, errors.New(fmt.Sprint("Password not valid: ", s))
    }

    ranged := strings.Split(s_tok[0], "-")
    low, err := strconv.Atoi(ranged[0])
    if err != nil {
        return nil, err
    }
    high, err := strconv.Atoi(ranged[1])
    if err != nil {
        return nil, err
    }
    runes := []rune(s_tok[1])
    substr := string(runes[:len(runes) - 1])

    return &Password {
        low: low,
        high: high,
        substr: string(substr),
        pass: s_tok[2],
    }, nil
}

func (p Password) isValid() bool {
    c := strings.Count(p.pass, p.substr)

    return c >= p.low && c <= p.high
}

func main() {
    items := make([](*Password), 0, 100)
    scanner := bufio.NewScanner(os.Stdin)
    for scanner.Scan() {
        newItem, err := newPassword(scanner.Text())
        if err != nil {
            fmt.Println(err)
        }
        items = append(items, newItem)
    }

    sum := 0
    for i := 0; i < len(items); i++ {
        if items[i].isValid() {
            sum += 1
        }
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
