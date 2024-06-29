package main

import (
	"fmt"
	"math"
	"math/rand"
)

func add(x int, y int) int {
	return x + y
}

func subtract(x, y int) int { //they share type
	return x - y
}

// A function can return any number of outputs
func swap(x, y float64) (float64, float64) {
	return y, x
}

// you can create named variables and return them automatically (but avoid it for longer functions)
func split(sum float64) (x, y float64) {
	x = sum * 4 / 9
	y = sum - x
	return
}

func main() {
	fmt.Println("A random number is", rand.Intn(10))
	fmt.Printf("Now a more squared %d number: %g.\n", 7, math.Sqrt(7))
	// call now `Pi`, exported names start with capital latters!
	fmt.Println("I want PI please:", math.Pi)
	fmt.Printf("Add %d and %d to get %d\n", 5, 6, add(5, 6)) // add \n at the end or Prinln to avoid a "%" at the end of the program
	fmt.Printf("Subtract %d and %d to get %d\n", 5, 6, subtract(5, 6))

	a, b := swap(math.Pi, math.Ln2)
	fmt.Printf("Swapping %g and %g to get %g and %g\n", math.Pi, math.Ln2, a, b)

	sum := rand.Float64()
	x, y := split(sum)
	fmt.Printf("%g is %g + %g\n", sum, x, y)
}
