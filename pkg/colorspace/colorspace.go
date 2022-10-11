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
// tristimulus value.
type Colorspace interface {
	Convert(spectrum.Distribution) Point
}

// ColorspaceFunc is a convenience typedef for defining a Colorspace from a
// function.
type ColorspaceFunc func(spectrum.Distribution) Point

func (cf ColorspaceFunc) Convert(dist spectrum.Distribution) Point {
	return cf(dist)
}

// Point represents a (tristimulus) point in a Colorspace.
type Point [3]float64

// Max returns the maximum component value.
func (p Point) Max() float64 {
	return math.Max(p[0], math.Max(p[1], p[2]))
}

// Min returns the minumum component value.
func (p Point) Min() float64 {
	return math.Min(p[0], math.Min(p[1], p[2]))
}

// Scale "scales" a point by multiplying all components by v.
func (p Point) Scale(v float64) Point {
	return Point{p[0] * v, p[1] * v, p[2] * v}
}

// Shift "shifts" a point by adding v to all components.
func (p Point) Shift(v float64) Point {
	return Point{p[0] + v, p[1] + v, p[2] + v}
}

// Zero returns true if the components are all 0
func (p Point) Zero() bool {
	return p[0] == 0 && p[1] == 0 && p[2] == 0
}
