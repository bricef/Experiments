package main

import (
	"fmt"

	jsx "github.com/zbysir/gojsx"
)

func main() {
	mdr, err := jsx.NewJsx(jsx.Option{Debug: true})
	if err != nil {
		fmt.Printf("jsx error: %v\n", err)
		return
	}
	// code, err := os.ReadFile("templates/simple.mdx")
	// if err != nil {
	// 	fmt.Printf("read file error: %v\n", err)
	// 	return
	// }
	s, err := mdr.Render("./templates/simple.mdx", map[string]interface{}{"a": 1})
	// s, err := mdr.ExecCode(code, jsx.WithFileName("templates/simple.mdx"))
	fmt.Printf("s: %v\n", s)

	if err != nil {
		fmt.Printf("render error: %v\n", err)
		return
	}

}
