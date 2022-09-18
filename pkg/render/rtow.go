package render

import (
	"fmt"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/geo"
)

func RTOW() error {
	film := Film{Width: 400, Height: 300}
	cam := camera.NewRTOW(geo.Origin, geo.Vector{0, 0, -1}, geo.YAxis, 50.0, 16.0/9.0)

	for y := 0; y < film.Height; y++ {
		for x := 0; x < film.Width; x++ {
			u := float64(x) / float64(film.Width)
			v := float64(y) / float64(film.Height)
			ray := cam.Ray(u, v)
			fmt.Print(ray)
		}
	}
	return nil
}
