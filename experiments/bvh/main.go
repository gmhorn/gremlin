package main

import (
	"flag"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math"
	"math/rand"
	"os"
	"runtime/pprof"
	"time"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/shape"
)

var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")

const filename = "bvh.png"

const imageWidth = 640
const imageHeight = 650
const aspectRatio = float64(imageWidth) / float64(imageHeight)
const fov = 45.0

var Black = Color{0, 0, 0}
var White = Color{1, 1, 1}

func rayColor(ray *geo.Ray, tris []*shape.Triangle) Color {
	tMin := math.Inf(1)

	for _, tri := range tris {
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
	// Profiling
	flag.Parse()
	if *cpuprofile != "" {
		p, err := os.Create(*cpuprofile)
		if err != nil {
			panic(err)
		}
		pprof.StartCPUProfile(p)
		defer pprof.StopCPUProfile()
	}

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

	fmt.Printf("Rendered in %s\n", dur)
	fmt.Printf("Total intersections:      %d\n", shape.Calls)
	fmt.Printf("Dur: %g\n", dur.Seconds()/float64(shape.Calls))

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

func initTris(count int) []*shape.Triangle {
	start := time.Now()

	tris := make([]*shape.Triangle, 0)

	for i := 0; i < count; i++ {
		r0 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}
		r1 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}
		r2 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}

		p0 := r0.Scale(9).Minus(geo.Vec{5, 5, 5})
		p1 := p0.Plus(r1)
		p2 := p0.Plus(r2)

		tris = append(tris, shape.NewTriangle(p0, p1, p2))
	}

	fmt.Printf("Created %d triangles in %s\n", count, time.Since(start))
	return tris
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
