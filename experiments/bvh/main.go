package main

import (
	"image"
	"image/color"
	"image/png"
	"math"
	"math/rand"
	"os"

	"github.com/gmhorn/gremlin/pkg/geo"
)

// bias is minumum distance unit
const bias = 1e-4
const fileName = "bvh.png"

const imageWidth = 640
const imageHeight = 640

var Black = Color{0, 0, 0}
var White = Color{1, 1, 1}

type Tri struct {
	Vert     [3]geo.Vec
	Centroid geo.Vec
}

func main() {
	// Image
	img := image.NewRGBA(image.Rect(0, 0, imageWidth, imageHeight))

	// Camera
	camPos := geo.Vec{0, 0, -18}
	p0 := geo.Vec{-1, 1, -15}
	p1 := geo.Vec{1, 1, -15}
	p2 := geo.Vec{-1, -1, -15}

	// World
	tris := InitTris(64)

	// Render
	for y := 0; y < imageHeight; y++ {
		for x := 0; x < imageWidth; x++ {

			// calculate pixel pos in global coord space
			pixelPos := p0.Plus(p1.Minus(p0).Scale(float64(x) / float64(imageWidth)))
			pixelPos = pixelPos.Plus(p2.Minus(p0).Scale(float64(y) / float64(imageHeight)))

			dir, _ := pixelPos.Minus(camPos).Normalize()
			ray := geo.NewRay(camPos, dir)

			// calculate closest intersection
			tMin := 1e30
			for _, tri := range tris {
				tInt := IntersectTri(ray, tri, tMin)
				tMin = math.Min(tInt, tMin)
			}

			// Color based on hit or not
			c := Black
			if tMin < 1e30 {
				c = White
			}

			// Color final image
			img.Set(x, imageHeight-y, c.ToRGBA())
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

func InitTris(count int) []*Tri {
	tris := make([]*Tri, 0)

	for i := 0; i < count; i++ {
		tri := &Tri{}
		r0 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}
		r1 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}
		r2 := geo.Vec{rand.Float64(), rand.Float64(), rand.Float64()}
		tri.Vert[0] = r0.Scale(9.0).Minus(geo.Vec{5, 5, 5})
		tri.Vert[1] = tri.Vert[0].Plus(r1)
		tri.Vert[2] = tri.Vert[0].Plus(r2)
		tris = append(tris, tri)
	}

	return tris
}

func IntersectTri(ray *geo.Ray, tri *Tri, max float64) float64 {
	edge1 := tri.Vert[1].Minus(tri.Vert[0])
	edge2 := tri.Vert[2].Minus(tri.Vert[0])

	h := ray.Dir.Cross(edge2)
	a := edge1.Dot(h)

	if a > -bias && a < bias {
		return max
	}

	f := 1.0 / a
	s := ray.Origin.Minus(tri.Vert[0])
	u := f * s.Dot(h)
	if u < 0 || u > 1 {
		return max
	}

	q := s.Cross(edge1)
	v := f * ray.Dir.Dot(q)

	if v < 0 || u+v > 1 {
		return max
	}

	dist := f * edge2.Dot(q)
	if dist <= bias || dist >= max {
		return max
	}

	return dist
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
