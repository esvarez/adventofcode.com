package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	fmt.Println("Hey advent of code 2022!")
	input := readFile("input.txt")
	queue := &Queue{}
	sum := 0
	for _, v := range input {
		if v == "" {
			if sum > queue.Min {
				queue.Enqueue(sum)
			}
			sum = 0
			continue
		}
		i, _ := strconv.Atoi(v)
		sum += i
	}
	fmt.Println(queue.Sum())
}

func readFile(fle string) []string {
	file, err := os.Open(fle)

	if err != nil {
		log.Fatalf("failed to open")

	}
	scanner := bufio.NewScanner(file)

	scanner.Split(bufio.ScanLines)
	var text []string

	for scanner.Scan() {
		text = append(text, scanner.Text())
	}

	file.Close()
	return text
}
