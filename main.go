package main

import "fmt"

const image_w = 256
const image_h = 256

func main() {
	fmt.Printf("Image width: %d, height: %d\n", image_w, image_h)

	for j := 0; j < image_h; j++ {
		for i := 0; i < image_w; i++ {
			r := float64(i) / (image_w - 1)
			g := float64(j) / (image_h - 1)
			b := 0.25

			ir := int64(255.999 * r)
			ig := int64(255.999 * g)
			ib := int64(255.999 * b)

			fmt.Printf("%d %d %d\n", ir, ig, ib)
		}
	}
}
