// Package colorspace provides tools for rendering physically-based light
// spectra into component systems suitable for actual display (such as sRGB).
//
// This code is more-or-less a direct port of the C program in John Walker's
// "Colour Rendering of Spectra" page, here:
// https://www.fourmilab.ch/documents/specrend/
//
// This SciPy blog post also provides some helpful explanation:
// https://scipython.com/blog/converting-a-spectrum-to-a-colour/
package colorspace

import "github.com/gmhorn/gremlin/pkg/spectrum"

// Colorspace converts a spectral distribution of light intensity to tristimulus
// values.
type Colorspace interface {
	Convert(spectrum.Distribution) [3]float64
}

// ColorspaceFunc is a convenience typedef for defining a Colorspace from a
// function.
type ColorspaceFunc func(spectrum.Distribution) [3]float64

func (cf ColorspaceFunc) Convert(spec spectrum.Distribution) [3]float64 {
	return cf(spec)
}

// Illuminant are the normalized chromaticity coordinates of an illuminant
// white point.
// https://en.wikipedia.org/wiki/Standard_illuminant
type Illuminant struct {
	X, Y float64
}

// White points of standard illuminants.
var (
	IlluminantD65 = Illuminant{0.31271, 0.32902}
	IlluminantC   = Illuminant{0.31006, 0.31616}
	IlluminantE   = Illuminant{0.33333, 0.33333}
)

// Colorspace is a mathetical color space based on the RGB color model.
// https://en.wikipedia.org/wiki/RGB_color_spaces
type Model struct {
	Red, Green, Blue, White Illuminant
	Gamma                   func(float64) float64
}

// Standard color spaces
// var (
// 	SRGB = Model{
// 		Red:   Illuminant{0.64, 0.33},
// 		Green: Illuminant{0.3, 0.6},
// 		Blue:  Illuminant{0.15, 0.06},
// 		White: IlluminantD65}
// )
