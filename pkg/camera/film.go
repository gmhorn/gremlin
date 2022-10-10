package camera

import "github.com/gmhorn/gremlin/pkg/colorspace"

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

// Film is a rectagular grid of pixels.
//
// It stores the pixels in a linear slice, since the most frequent operations
// are interation over ranges of pixels. The PixelAt and RasterCoords functions
// help translate between linear pixel space and raster space.
//
// For any given Film, it's raster space runs from (0, 0) in the upper-left to
// (W, H) in the lower right (i.e. the y-axis "points down").
type Film struct {
	Width, Height int
	AspectRatio   float64
	Pixels        []Pixel
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

// PixelAt returns the Pixel and its index for the given raster coordinates.
//
// TODO: should we check bounds, and what to do if bounds check fails?
func (f *Film) PixelAt(x, y int) (int, *Pixel) {
	pxIdx := x*f.Width + y
	return pxIdx, &f.Pixels[pxIdx]
}
