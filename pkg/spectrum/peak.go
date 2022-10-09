package spectrum

import "math"

// Peak is a narrow normal distribution centered at a given peak wavelength.
type Peak float64

func (p Peak) Lookup(wavelength float64) float64 {
	return math.Exp(-0.002 * math.Pow(wavelength-float64(p), 2))
}
