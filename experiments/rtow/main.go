package main

import (
	"image"
	"image/color"
	"image/png"
	"math"
	"os"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/geo"
)

const fileName = "rtow.png"

const imageWidth = 400
const imageHeight = 300
const aspectRatio = float64(imageWidth) / float64(imageHeight)
const fov = 90.0

var Red = Color{1.0, 0.0, 0.0}
var White = Color{1.0, 1.0, 1.0}
var Blue = Color{0.5, 0.7, 1.0}

func rayColor(ray *geo.Ray, world Hittable) Color {
	// If hit something, pain normal (ish...)
	if hit, success := world.Hit(ray, 0, math.MaxFloat64); success {
		c := Color{hit.Norm[0] + 1, hit.Norm[1] + 1, hit.Norm[2] + 1}
		return c.Mult(0.5)
	}

	// Else paint background
	t := 0.5 * (ray.Dir[1] + 1.0)
	return Blue.Lerp(White, t)
}

func main() {
	// Image
	img := image.NewRGBA(image.Rect(0, 0, imageWidth, imageHeight))

	// Camera
	cam := camera.NewPerspective(aspectRatio, fov)
	cam.MoveTo(geo.Vec{0, 0, 100})

	//World
	var world Aggregate
	world = append(world, &Sphere{
		Center: geo.Vec{0, 0, -1},
		Radius: 0.5,
	})
	world = append(world, &Sphere{
		Center: geo.Vec{0, -100.5, -1},
		Radius: 100,
	})

	// Render
	for y := 0; y < imageHeight; y++ {
		for x := 0; x < imageWidth; x++ {
			u := (float64(x) + 0.5) / imageWidth
			v := (float64(y) + 0.5) / imageHeight

			// Create ray
			ray := cam.Ray(u, v)

			// Calculate color
			c := rayColor(ray, world)

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
