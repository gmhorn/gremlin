package colorspace

import (
	"math"

	"github.com/gmhorn/gremlin/pkg/spectrum"
)

// RGB is Colorspace whose chromaticity values are the familiar red, green, blue
// additive primaries.
//
// See:
// https://en.wikipedia.org/wiki/RGB_color_model
// https://en.wikipedia.org/wiki/RGB_color_spaces
type RGB struct {
	m     [3][3]float64
	gamma func(float64) float64
}

// Convert returns the red, green, blue chromaticity values for the given
// spectrum. Returned values are in the range [0, 1]. Final conversion to
// integer values (e.g. 0 to 255) can then be done by multiplying by 2^(bits)
// and rounding.
//
// Internally, this works by first converting the spectrum to (CIE 1931) XYZ,
// then calling ConvertXYZ.
func (cs *RGB) Convert(spec spectrum.Distribution) [3]float64 {
	xyz := CIE1931.Convert(spec)
	return cs.ConvertXYZ(xyz)
}

// ConvertXYZ converts CIE 1931 X, Y, Z chromaticities to final red, green, blue
// chromaticities. Like in Convert, values are in the range [0, 1].
//
// Internally this works by first multiplying by a linear transformation, then
// gamma correcting. If the color is outside gamut, it is desaturated by adding
// white (equal parts r, g and b) to bring it into gamut. Finally, if any
// component values are out of range, it is clamped into range by uniformly
// scaling the components.
//
// This code is more-or-less a straight port of John Walker's "SpectrumToXYZ"
// function from his "Colour Rendering of Spectra" page:
//
//	https://www.fourmilab.ch/documents/specrend/
//	https://www.fourmilab.ch/documents/specrend/specrend.c
func (cs *RGB) ConvertXYZ(xyz [3]float64) [3]float64 {
	rgb := [3]float64{}
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			rgb[i] += cs.m[i][j] * xyz[j]
		}

		rgb[i] = cs.gamma(rgb[i])
	}

	// if out of gamut, desaturate
	min := math.Min(rgb[0], math.Min(rgb[1], rgb[2]))
	if min < 0 {
		for i := range rgb {
			rgb[i] += min
		}
	}

	// clamp max value
	max := math.Max(rgb[0], math.Max(rgb[1], rgb[2]))
	if max > 1 {
		for i, v := range rgb {
			rgb[i] = v / max
		}
	}

	return rgb
}

// SRGB is a standard color space widely useful for display on monitors. Note
// that its name is properly rendered "sRGB" but Go naming conventions require
// the initial "s" to be capitalized.
// https://en.wikipedia.org/wiki/SRGB
//
// Values taken from Bruce Lindbloom's page:
// http://www.brucelindbloom.com/
var SRGB = RGB{
	m: [3][3]float64{
		{+3.2404542, -1.5371385, -0.4985314},
		{-0.9692660, +1.8760108, +0.0415560},
		{+0.0556434, -0.2040259, +1.0572252},
	},
	gamma: func(v float64) float64 {
		if v <= 0.0031308 {
			return 12.92 * v
		}
		return 1.055*math.Pow(v, 0.41667) - 0.055
	},
}

// Illuminant are the normalized chromaticity coordinates of an illuminant
// white point.
// https://en.wikipedia.org/wiki/Standard_illuminant
// type Illuminant struct {
// 	X, Y float64
// }

// White points of standard illuminants.
// var IlluminantD65 = Illuminant{0.31271, 0.32902}
// var IlluminantC   = Illuminant{0.31006, 0.31616}
// var IlluminantE   = Illuminant{0.33333, 0.33333}

// Model represents an instance of an RGB color model.
// https://en.wikipedia.org/wiki/RGB_color_spaces
// type Model struct {
// 	Red, Green, Blue, White Illuminant
// 	Gamma                   func(float64) float64
// }

// Standard color spaces
// var (
// 	ModelSRGB = Model{
// 		Red:   Illuminant{0.64, 0.33},
// 		Green: Illuminant{0.3, 0.6},
// 		Blue:  Illuminant{0.15, 0.06},
// 		White: IlluminantD65}
// )
