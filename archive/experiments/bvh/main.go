package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math"
	"os"
	"time"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/shape"
)

const filename = "bvh.png"

const imageWidth = 640
const imageHeight = 650
const aspectRatio = float64(imageWidth) / float64(imageHeight)
const fov = 45.0

var Black = Color{0, 0, 0}
var White = Color{1, 1, 1}

var intersectionTests int

func rayColor(ray *geo.Ray, tris []*shape.Triangle) Color {
	tMin := math.Inf(1)

	for _, tri := range tris {
		intersectionTests++

		if t := tri.Intersect(ray); t >= 0 {
			tMin = math.Min(tMin, t)
		}
	}

	if tMin < math.Inf(1) {
		return White
	}
	return Black
}

func main() {
	// Image
	img := image.NewRGBA(image.Rect(0, 0, imageWidth, imageHeight))

	// Camera
	cam := camera.NewPerspective(aspectRatio, fov)
	cam.MoveTo(geo.V(0, 0, -18))
	cam.PointAt(geo.Origin)

	// Triangles
	tris := initTris(64)

	// Render
	start := time.Now()
	for y := 0; y < imageHeight; y++ {
		for x := 0; x < imageWidth; x++ {
			u := (float64(x) + 0.5) / imageWidth
			v := (float64(y) + 0.5) / imageHeight

			// Create ray
			ray := cam.Ray(u, v)

			// Calculate color
			c := rayColor(ray, tris)

			// Write to image
			img.Set(x, y, c.ToRGBA())
		}
	}
	dur := time.Since(start)

	// Print metrics
	fmt.Printf("Total render time:  %s\n", dur)
	fmt.Printf("Intersection Tests: %d\n", intersectionTests)
	fmt.Printf("Average cost:       %5f ns/test\n", float64(dur.Nanoseconds())/float64(intersectionTests))

	// Write out
	file, err := os.Create(filename)
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
