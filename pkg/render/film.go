package render

import (
	"image"
	"image/color"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

// Film gathers radiance samples for each pixel in the final image. In general
// there will be many paths sampled per
type Film struct {
	Width  int
	Height int

	pixels  []colorspace.Point
	samples []int
}

func NewFilm(width, height int) *Film {
	return &Film{
		Width:   width,
		Height:  height,
		pixels:  make([]colorspace.Point, width*height),
		samples: make([]int, width*height),
	}
}

func (f *Film) Add(x, y int, d spectrum.Distribution) {
	idx := (y * f.Width) + x
	xyz := colorspace.CIE1931.Convert(d)

	f.pixels[idx][0] += xyz[0]
	f.pixels[idx][1] += xyz[1]
	f.pixels[idx][2] += xyz[2]
	f.samples[idx]++
}

func (f *Film) Image(cs colorspace.RGB) *image.RGBA {
	img := image.NewRGBA(image.Rect(0, 0, f.Width, f.Height))
	for y := 0; y < f.Height; y++ {
		for x := 0; x < f.Width; x++ {
			idx := (y * f.Width) + x

			n := 1 / float64(f.samples[idx])
			xyz := f.pixels[idx].Scale(n)

			rgb := cs.ConvertXYZ(xyz)
			img.Set(x, y, color.RGBA{
				R: uint8(rgb[0] * 255),
				G: uint8(rgb[1] * 255),
				B: uint8(rgb[2] * 255),
				A: 255,
			})
		}
	}
	return img
}
