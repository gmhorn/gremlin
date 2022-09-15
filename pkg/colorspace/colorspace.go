// Package colorspace provides tools for rendering physically-based light
// spectra into component systems suitable for actual display (such as sRGB).
//
// This code heavily borrows from the following sources:
// https://www.fourmilab.ch/documents/specrend/
// https://scipython.com/blog/converting-a-spectrum-to-a-colour/
// http://www.brucelindbloom.com/
package colorspace

import "github.com/gmhorn/gremlin/pkg/spectrum"

// Colorspace converts a spectral distribution of light intensity to a
// tristimulus Pixel value.
type Colorspace interface {
	Convert(spectrum.Distribution) [3]float64
}

// ColorspaceFunc is a convenience typedef for defining a Colorspace from a
// function.
type ColorspaceFunc func(spectrum.Distribution) [3]float64

func (cf ColorspaceFunc) Convert(spec spectrum.Distribution) [3]float64 {
	return cf(spec)
}

// Pixel represents tristimulus coordinate values in a Colorspace. It's kind of
// an abuse of the term "pixel" -- calling it "Coordinates" may be more
// accurate. But since our goal is ultimately to paint pixels, the name is good
// enough.
type Pixel [3]float64
