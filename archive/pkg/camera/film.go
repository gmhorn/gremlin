package camera

import (
	"image"
	"image/color"
	"math/rand"

	"github.com/gmhorn/gremlin/archive/pkg/colorspace"
)

// Pixel is an individual film pixel. Its Color field stores the running sum of
// the spectral sample contributions to the final pixel color, and the Samples
// field stores the number of samples. The final pixel color can be easily
// determined by taking the average
//
//	pixel.Color / pixel.Samples
//
// A natural alternative would be to have each pixel store a running sum of the
// full spectral distributions (say spectrum.Discrete). However, this results
// a much larger memory usage. Further, as long as a linear color space is used
// (e.g. colorspace.CIE1932) then both
//
//   - converting from a spectrum to a colorspace point, and
//   - averaging colorspace points
//
// are linear and distribute over each other. So no accuracy is lost.
//
// https://computergraphics.stackexchange.com/a/11000
type Pixel struct {
	Color   colorspace.Point
	Samples uint64
}

func (p *Pixel) AddColor(c colorspace.Point) {
	p.Color[0] += c[0]
	p.Color[1] += c[1]
	p.Color[2] += c[2]
	p.Samples++
}

// Film is a rectagular grid of pixels.
//
// It stores the pixels in a linear slice, since the most frequent operations
// are interation over ranges of pixels. The PixelAt and RasterCoords functions
// help translate between linear pixel space and raster space.
//
// For any given Film, it's raster space runs from (0, 0) in the upper-left to
// (W, H) in the lower right (i.e. the y-axis "points down").
//
// Also remember, we're keeping a slice of Pixel values, not pixel pointers.
// Don't try to modify them in a for-range loop:
//
//	var film *Film
//	var color colorspace.Point
//	for _, px := range film.Pixels {
//	  px.Color = color	// Change won't be reflected in slice!
//	  px.Samples = 1	// Ditto!
//	}
//
// The tradeoff here is we have to range over the index then use that to mutate
// the underlying slice. The benefit is much better data locality and cache
// performance.
type Film struct {
	Width, Height int
	AspectRatio   float64
	Pixels        []Pixel
}

// FilmTile is a slice of Pixels with a set Offset.
type FilmTile struct {
	Pixels []Pixel
	Offset int
}

// NewFilm creates a new film with the given width and height (in pixels).
// Panics if width or height is 0 or negative (since this is almost always
// constructed at the beginning of a program, so might as well fail fast).
func NewFilm(width, height int) *Film {
	if width < 1 || height < 1 {
		panic("Film must have positive width and height")
	}

	return &Film{
		Width:       width,
		Height:      height,
		AspectRatio: float64(width) / float64(height),
		Pixels:      make([]Pixel, width*height),
	}
}

// RasterCoords gives the x, y raster coordinates for a given pixel index.
func (f *Film) RasterCoords(pxIdx int) (x, y int) {
	x = pxIdx % f.Width
	y = pxIdx / f.Width
	return
}

func (f *Film) RandomNDC(pxIdx int, r *rand.Rand) (u, v float64) {
	x, y := f.RasterCoords(pxIdx)
	u = (float64(x) + r.Float64()) / float64(f.Width)
	v = (float64(y) + r.Float64()) / float64(f.Height)
	return
}

// PixelAt returns the Pixel and its index for the given raster coordinates.
//
// TODO: should we check bounds, and what to do if bounds check fails?
func (f *Film) PixelAt(x, y int) (int, *Pixel) {
	pxIdx := x*f.Width + y
	return pxIdx, &f.Pixels[pxIdx]
}

// Merge merges a slice of pixels into this film's pixel buffer at the given
// offset.
func (f *Film) Merge(tile *FilmTile) {
	for idx := range tile.Pixels {
		filmIdx := tile.Offset + idx
		f.Pixels[filmIdx].Color[0] = tile.Pixels[idx].Color[0]
		f.Pixels[filmIdx].Color[1] = tile.Pixels[idx].Color[1]
		f.Pixels[filmIdx].Color[2] = tile.Pixels[idx].Color[2]
		f.Pixels[filmIdx].Samples += tile.Pixels[idx].Samples
	}
}

func (f *Film) Image(cs colorspace.RGB) *image.RGBA {
	img := image.NewRGBA(image.Rect(0, 0, f.Width, f.Height))
	for i, px := range f.Pixels {
		x, y := f.RasterCoords(i)

		n := 1 / float64(px.Samples)
		xyz := px.Color.Scale(n)

		rgb := cs.ConvertXYZ(xyz)
		img.Set(x, y, color.RGBA{
			R: uint8(rgb[0] * 255),
			G: uint8(rgb[1] * 255),
			B: uint8(rgb[2] * 255),
			A: 255,
		})
	}
	return img
}
