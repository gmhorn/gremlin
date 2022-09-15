// Package colorspace provides tools for rendering physically-based light
// spectra into component systems suitable for actual display (such as sRGB).
//
// This code heavily borrows from the following sources:
// https://www.fourmilab.ch/documents/specrend/
// https://scipython.com/blog/converting-a-spectrum-to-a-colour/
// http://www.brucelindbloom.com/
package colorspace

import (
	"math"

	"github.com/gmhorn/gremlin/pkg/spectrum"
)

// Colorspace converts a spectral distribution of light intensity to a
// tristimulus Pixel value.
type Colorspace interface {
	Convert(spectrum.Distribution) Pixel
}

// ColorspaceFunc is a convenience typedef for defining a Colorspace from a
// function.
type ColorspaceFunc func(spectrum.Distribution) Pixel

func (cf ColorspaceFunc) Convert(spec spectrum.Distribution) Pixel {
	return cf(spec)
}

// Pixel represents tristimulus coordinate values in a Colorspace. It's kind of
// an abuse of the term "pixel" -- calling it "Coordinates" may be more
// accurate. But since our goal is ultimately to paint pixels, the name is good
// enough.
type Pixel [3]float64

// Max returns the minumum component value.
func (p Pixel) Max() float64 {
	return math.Max(p[0], math.Max(p[1], p[2]))
}

// Min returns the minumum component value.
func (p Pixel) Min() float64 {
	return math.Min(p[0], math.Min(p[1], p[2]))
}

// CAdd returns a new Pixel obtained by adding v to all component values.
func (p Pixel) CAdd(v float64) Pixel {
	return Pixel{p[0] + v, p[1] + v, p[2] + v}
}

// CDiv returns a new Pixel obtained by dividing all component values by v.
func (p Pixel) CDiv(v float64) Pixel {
	return Pixel{p[0] / v, p[1] / v, p[2] / v}
}
