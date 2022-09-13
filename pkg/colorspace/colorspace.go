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

// SpectrumToXYZ calculates the CIE 1931 X, Y and Z coordinates for a light
// source with the given spectral distribution. The distribution will be
// evaluated from 380nm to 780nm at 5nm intervals and should return the
// emittance at that wavelength. The precise units it returns do not matter,
// as the chromaticity coordinates are scaled to respect the identity
//
//	X + Y + Z = 1
func SpectrumToXYZ(spec spectrum.Distribution) [3]float64 {
	X := 0.0
	Y := 0.0
	Z := 0.0

	for i, power := range spectrum.Discretize(spec) {
		X += power * CIE_X[i]
		Y += power * CIE_Y[i]
		Z += power * CIE_Z[i]
	}
	XYZ := X + Y + Z

	return [3]float64{X / XYZ, Y / XYZ, Z / XYZ}
}
