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
type Colorspace struct {
	Red, Green, Blue, White Illuminant
	Gamma                   func(float64) float64
}

// Standard color spaces
var (
	SRGB = Colorspace{
		Red:   Illuminant{0.64, 0.33},
		Green: Illuminant{0.3, 0.6},
		Blue:  Illuminant{0.15, 0.06},
		White: IlluminantD65}
)
