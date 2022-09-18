package render

import (
	"image/png"
	"os"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

var cold = spectrum.Discretize(spectrum.Blackbody(2000))
var hot = spectrum.Discretize(spectrum.Blackbody(12000))

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

	file, err := os.Create("test.png")
	if err != nil {
		return err
	}
	defer file.Close()

	return png.Encode(file, film.Image(colorspace.SRGB))
}

func radiance(r *geo.Ray) spectrum.Distribution {
	t := 0.5 * (r.Dir.Y + 1)
	temp := (1.0-t)*2000 + t*12000
	return spectrum.Blackbody(temp)
}
