package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {

	dir, err := os.Getwd()
	if err != nil {
		log.Println(err)
	}
	inputPath := dir + "/day_1/input.txt"
	fmt.Println(inputPath)

	file, err := os.Open(inputPath)
	check(err)

	scanner := bufio.NewScanner(file)

	caloriesForCurrentElf := 0

	mostCalories := math.MinInt64

	for scanner.Scan() {
		currentLine := scanner.Text()

		if currentLine != "" {
			calories, err := strconv.Atoi(currentLine)
			check(err)
			caloriesForCurrentElf += calories
		} else {
			if caloriesForCurrentElf > mostCalories {
				mostCalories = caloriesForCurrentElf
			}
			caloriesForCurrentElf = 0
		}
	}
	println(mostCalories)
}
