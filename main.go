package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"os"
)

const image_w = 256
const image_h = 256

func main() {
	fmt.Printf("Image width: %d, height: %d\n", image_w, image_h)
	doImage()
}

func printImage() {
	for j := 0; j < image_h; j++ {
		for i := 0; i < image_w; i++ {
			r := float64(i) / (image_w - 1)
			g := float64(j) / (image_h - 1)
			b := 0.25

			ir := uint8(255.999 * r)
			ig := uint8(255.999 * g)
			ib := uint8(255.999 * b)

			fmt.Printf("%d %d %d\n", ir, ig, ib)
		}
	}
}

func doImage() {
	topLeft := image.Point{0, 0}
	bottomRight := image.Point{image_w, image_h}
	img := image.NewRGBA(image.Rectangle{topLeft, bottomRight})

	for x := 0; x < image_w; x++ {
		for y := 0; y < image_h; y++ {
			r := float64(x) / (image_w - 1)
			g := float64(y) / (image_h - 1)
			b := 0.25

			ir := uint8(255.999 * r)
			ig := uint8(255.999 * g)
			ib := uint8(255.999 * b)

			c := color.RGBA{ir, ig, ib, 0xff}
			img.Set(x, y, c)
		}
	}

	f, _ := os.Create("out.png")
	png.Encode(f, img)
}
