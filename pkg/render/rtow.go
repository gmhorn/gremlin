package render

import (
	"fmt"
	"image/png"
	"math"
	"os"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

var background = spectrum.Blackbody(12000)

// var cold = spectrum.Blackbody(5000)
// var hot = spectrum.Blackbody(12000)

func RTOW() error {
	// Image
	film := NewFilm(400, 300)
	aspectRatio := 400.0 / 300.0

	// Camera
	viewportHeight := 2.0
	viewportWidth := aspectRatio * viewportHeight
	focalLength := 1.0

	horiz := geo.Vector{viewportWidth, 0, 0}
	vert := geo.Vector{0, viewportHeight, 0}
	lowerLeft := geo.Origin.Minus(horiz.Scale(0.5))
	lowerLeft = lowerLeft.Minus(vert.Scale(0.5))
	lowerLeft = lowerLeft.Minus(geo.Vector{0, 0, focalLength})

	// Render

	for y := 0; y < film.Height; y++ {
		for x := 0; x < film.Width; x++ {
			u := float64(x) / float64(film.Width)
			v := float64(y) / float64(film.Height)

			d := lowerLeft.Plus(horiz.Scale(u))
			d = d.Plus(vert.Scale(v))
			d = d.Minus(geo.Origin)
			dir, _ := d.Unit()

			film.Add(x, y, radiance(&geo.Ray{geo.Origin, dir}))
		}
	}

	fmt.Println("Temp min", tempMin)
	fmt.Println("Temp max", tempMax)

	file, err := os.Create("test.png")
	if err != nil {
		return err
	}
	defer file.Close()

	return png.Encode(file, film.Image(colorspace.SRGB))
}

var tempMin float64 = 3000
var tempMax float64 = 1000

func radiance(r *geo.Ray) spectrum.Distribution {
	t := hitSphere(geo.Vector{0, 0, -1}, 0.5, r)
	if t < 0 {
		return background
	}

	temp := (1.0-t)*1000 + t*3000
	if temp < tempMin {
		tempMin = temp
	}
	if temp > tempMax {
		tempMax = temp
	}
	return spectrum.Blackbody(temp)
	// t := 0.5 * (r.Dir.Y + 1)
	// temp := (1.0-t)*2000 + t*12000
	// return spectrum.Blackbody(temp)
}

func hitSphere(center geo.Vector, radius float64, r *geo.Ray) float64 {
	oc := r.Origin.Minus(center)
	a := r.Dir.Dot(geo.Vector(r.Dir))
	b := 2.0 * oc.Dot(geo.Vector(r.Dir))
	c := oc.Dot(oc) - radius*radius
	disc := b*b - 4*a*c
	if disc < 0 {
		return -1
	}
	return (-b - math.Sqrt(disc)) / (2.0 * a)
}
