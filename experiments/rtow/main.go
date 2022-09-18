package main

import (
	"image"
	"image/color"
	"image/png"
	"math"
	"os"

	"github.com/gmhorn/gremlin/pkg/geo"
)

const fileName = "rtow.png"

const imageWidth = 400
const imageHeight = 300
const aspectRatio = float64(imageWidth) / float64(imageHeight)

var Red = Color{1.0, 0.0, 0.0}
var White = Color{1.0, 1.0, 1.0}
var Blue = Color{0.5, 0.7, 1.0}

func rayColor(ray *geo.Ray) Color {
	// If hit sphere, paint normal (ish...)
	t := hitSphere(geo.Vec{0, 0, -1}, 0.5, ray)
	if t > 0 {
		N, _ := ray.At(t).Minus(geo.Vec{0, 0, -1}).Unit()
		return Color{N[0] + 1, N[1] + 1, N[2] + 1}.Mult(0.5)
	}

	// Else paint background
	t = 0.5 * (ray.Dir[1] + 1.0)
	return Blue.Lerp(White, t)
}

func hitSphere(center geo.Vec, radius float64, ray *geo.Ray) float64 {
	oc := ray.Origin.Minus(center)
	a := ray.Dir.Dot(geo.Vec(ray.Dir))
	b := 2.0 * oc.Dot(geo.Vec(ray.Dir))
	c := oc.Dot(oc) - radius*radius

	disc := b*b - 4*a*c
	if disc < 0 {
		return -1.0
	}
	return (-b - math.Sqrt(disc)) / (2.0 * a)
}

func main() {
	// Image
	img := image.NewRGBA(image.Rect(0, 0, imageWidth, imageHeight))

	// Camera
	viewportHeight := 2.0
	viewportWidth := aspectRatio * viewportHeight
	focalLength := 1.0

	origin := geo.Origin
	horizontal := geo.XAxis.Scale(viewportWidth)
	vertical := geo.YAxis.Scale(viewportHeight)
	lowerLeft := origin.Minus(horizontal.Scale(0.5))
	lowerLeft = lowerLeft.Minus(vertical.Scale(0.5))
	lowerLeft = lowerLeft.Minus(geo.Vec{0, 0, focalLength})

	// Render
	for y := 0; y < imageHeight; y++ {
		for x := 0; x < imageWidth; x++ {
			u := float64(x) / (imageWidth - 1)
			v := float64(y) / (imageHeight - 1)

			// calculate pixel pos in global coord space
			scrn := lowerLeft.Plus(horizontal.Scale(u))
			scrn = scrn.Plus(vertical.Scale(v))
			scrn = scrn.Minus(origin)

			// Normalize it
			dir, _ := scrn.Unit()
			ray := &geo.Ray{Origin: origin, Dir: dir}

			// Calculate color
			c := rayColor(ray)

			// Write to image
			img.Set(x, y, c.ToRGBA())
		}
	}

	// Write out
	file, err := os.Create(fileName)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	err = png.Encode(file, img)
	if err != nil {
		panic(err)
	}
}

type Color [3]float64

func (c Color) ToRGBA() *color.RGBA {
	return &color.RGBA{
		R: uint8(255.999 * c[0]),
		G: uint8(255.999 * c[1]),
		B: uint8(255.999 * c[2]),
		A: 255,
	}
}

func (c Color) Mult(t float64) Color {
	return Color{t * c[0], t * c[1], t * c[2]}
}

func (c Color) Lerp(d Color, t float64) Color {
	s := 1 - t
	return Color{
		s*c[0] + t*d[0],
		s*c[1] + t*d[1],
		s*c[2] + t*d[2],
	}
}
