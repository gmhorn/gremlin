package main

import (
	"image/png"
	"os"
	"runtime/pprof"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/render"
	"github.com/gmhorn/gremlin/pkg/shape"
)

func main() {
	profFile, err := os.Create("main.prof")
	if err != nil {
		panic(err)
	}
	pprof.StartCPUProfile(profFile)
	defer pprof.StopCPUProfile()

	film := camera.NewFilm(1920, 1080)
	cam := camera.NewPerspective(film.AspectRatio, 75.0)
	cam.MoveTo(geo.V(-3, 3, 1)).PointAt(geo.V(0, 0, -1))

	scene := []shape.Shape{
		&shape.Sphere{
			Center: geo.V(-0.5, 0, -1),
			Radius: 0.5,
		},
		&shape.Sphere{
			Center: geo.V(0, -100.5, -1),
			Radius: 100,
		},
	}

	if err := render.Fixed(film, cam, scene); err != nil {
		panic(err)
	}

	file, err := os.Create("main.png")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	err = png.Encode(file, film.Image(colorspace.SRGB))
	if err != nil {
		panic(err)
	}
}
