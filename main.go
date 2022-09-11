package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"os"
	"unsafe"

	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

const image_w = 256
const image_h = 256

func main() {
	fmt.Printf("Image width: %d, height: %d\n", image_w, image_h)
	doImage()
	doFilm()
}

func compareSizes() {
	var structs [800 * 600]geo.Vec3
	for x := 0; x < 800; x++ {
		for y := 0; y < 600; y++ {
			structs[y*800+x] = geo.Origin
		}
	}

	var flat [800 * 600 * 3]float64
	for x := 0; x < 800*600*3; x++ {
		flat[x] = 123.45
	}

	fmt.Printf("structs: %v, flat: %v\n", unsafe.Sizeof(structs), unsafe.Sizeof(flat))
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

func doFilm() {
	f := spectrum.NewFilm(800, 600)
	for x := 0; x < f.Width; x++ {
		for y := 0; y < f.Height; y++ {
			e := spectrum.Energy{
				(float64(x) * 255) / (float64(f.Width) - 1),
				(float64(y) * 255) / (float64(f.Height) - 1),
				64,
			}
			f.Add(x, y, e)
		}
	}
	file, _ := os.Create("film.png")
	png.Encode(file, f.Image())
}
