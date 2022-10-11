package main

import (
	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/geo"
)

func main() {
	film := camera.NewFilm(800, 600)
	cam := camera.NewPerspective(film.AspectRatio, 75.0)
	cam.MoveTo(geo.V(-3, 3, 1)).PointAt(geo.V(0, 0, -1))
}
