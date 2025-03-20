package main

import "fmt"

func main() {
    var currentAge int64
    fmt.Println("What is your age?")
    _, err := fmt.Scanln(&currentAge)
    if err != nil {
        fmt.Println("Invalid input. Please enter a number.")
        currentAge = 0
        _, err := fmt.Scanln(&currentAge)
        if err != nil {
            fmt.Println("Sorry, that was your last try, you may not enter.")
            return
        }
    }


    var of_age bool = age_checker(currentAge)
    if of_age{
        fmt.Printf("thank you, your is %v and this is 18 or above, thank you", currentAge)
    } else {
        fmt.Println("Sorry, your age is under 18 and you cannot enter.")
    }

}

func age_checker(age int64) bool{
    return age >= 17
}