package render

import (
	"image"
	"image/color"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

// Pixel represents a pixel in the finalized image.
//
// Conceptually, the job of a raytracer is to determine what color each pixel
// should be. In turn, the color of a pixel is determined by the light arriving
// at the film plane at the pixel's location. The camera subsystem handles
// generating rays that pass through each pixel, and the majority of the rest of
// the code handles evaluating the radiance along each ray and converting it to
// a color value.
//
// For a given pixel, many rays will be sampled, and their contributions
// combined in order to create a final color value.
type Pixel struct {
	Color   colorspace.Point
	Samples uint64
}

// Safe to do because averaging colors is a linear color operation, and
// reduction of spectrum to tristimulus is a linear operation, so they
// distribute over each other
//
// https://computergraphics.stackexchange.com/a/11000
func (p *Pixel) AddSample(radiance spectrum.Distribution, cs colorspace.Colorspace) {
	c := cs.Convert(radiance)
	p.Color[0] += c[0]
	p.Color[1] += c[1]
	p.Color[2] += c[2]
	p.Samples++
}

type FilmTile struct {
}

// Film is responsible for storing pixel color values.
//
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

// Raster2NDC takes the Film-centric raster coordinates (px, py) in the range
// [0, Width) x [0, Height) to their Normalized Device Coordinates (nx, ny)
// in the range [0, 1] x [0, 1].
//
// See below for details:
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
func (f *Film) Raster2NDC(px, py int) (float64, float64) {
	nx := (float64(px) + 0.5) / float64(f.Width)
	ny := (float64(py) + 0.5) / float64(f.Height)
	return nx, ny
}
