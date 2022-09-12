package render

import "github.com/gmhorn/gremlin/pkg/spectrum"

// Film gathers radiance samples for each pixel in the final image. In general
// there will be many paths sampled per
type Film struct {
	Width  uint
	Height uint

	pixels  []spectrum.Discrete
	samples []uint
}

func NewFilm(width, height uint) *Film {
	return &Film{
		Width:   width,
		Height:  height,
		pixels:  make([]spectrum.Discrete, width*height),
		samples: make([]uint, width*height),
	}
}
