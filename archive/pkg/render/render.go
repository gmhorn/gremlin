package render

import (
	"math"
	"math/rand"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/shape"
	"github.com/gmhorn/gremlin/pkg/spectrum"
	"github.com/gmhorn/gremlin/pkg/util"
)

const tileSize = 64
const samples = 32

func Fixed(film *camera.Film, cam *camera.Perspective, scene []shape.Shape) error {
	// Split up film into tiles
	tiles := util.Partition(len(film.Pixels), tileSize)
	results := make(chan *camera.FilmTile)

	for _, tile := range tiles {
		go func(offset, size int) {
			pixels := make([]camera.Pixel, size)
			rnd := rand.New(rand.NewSource(rand.Int63()))

			for i := range pixels {
				for s := 0; s < samples; s++ {
					ray := cam.Ray(film.RandomNDC(i+offset, rnd))
					dist := rayColor(ray, scene)
					pixels[i].AddColor(colorspace.CIE1931.Convert(dist))
				}
			}

			results <- &camera.FilmTile{Pixels: pixels, Offset: offset}

		}(tile.Offset, tile.Size)
	}

	for range tiles {
		film.Merge(<-results)
	}

	return nil
}

func rayColor(ray *geo.Ray, scene []shape.Shape) spectrum.Distribution {
	var tInt = math.Inf(1)
	var sInt shape.Shape

	for _, shape := range scene {
		t := shape.Intersect(ray)
		if t > 0 && t < tInt {
			tInt = t
			sInt = shape
		}
	}

	if !math.IsInf(tInt, 0) {
		pt := ray.At(tInt)
		norm := sInt.Normal(pt)

		r := spectrum.Red.Scale(norm.X + 1)
		g := spectrum.Green.Scale(norm.Y + 1)
		b := spectrum.Blue.Scale(norm.Z + 1)
		return r.Plus(g.Plus(b)).Scale(0.5)
	}

	t := 0.5 * (ray.Dir.Unit().Y + 1.0)
	return spectrum.Blue.Lerp(&spectrum.ACESIllumD60, t)
}
