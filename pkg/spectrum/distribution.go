package spectrum

import "math"

// Distribution represents a spectral distribution - a quantity that is a
// function of wavelength. Examples are reflectance, refractive index, radiance,
// etc. Typically these are implemented either analytically, or via lookup
// table and interpolation.
//
// The Lookup method evaluates the Distribution at a given wavelength.
// Wavelength is expected to be given in nanometers; the visible spectrum is
// between 380nm and 780nm. The returned value and its units are Distribution-
// dependent.
type Distribution interface {
	Lookup(wavelength float64) float64
}

// DistributionFunc is a convenience typedef to make it easy to define a
// Distribution from a function.
type DistributionFunc func(float64) float64

// Lookup just uses the DistributionFunc itself to evaluate the argument.
func (df DistributionFunc) Lookup(wavelength float64) float64 {
	return df(wavelength)
}

// Flat creates a constant-valued Distribution.
func Flat(value float64) Distribution {
	return DistributionFunc(func(_ float64) float64 {
		return value
	})
}

// Peak creates a narrow, normal distribution centered at the given wavelength.
func Peak(center, variance float64) Distribution {
	return DistributionFunc(func(wavelength float64) float64 {
		return math.Exp(-math.Pow(wavelength-center, 2) / (2 * variance))
	})
}

// // Sum returns a new Distribution that is the sum of the two distributions.
// func Sum(a, b Distribution) Distribution {
// 	return DistributionFunc(func(wavelength float64) float64 {
// 		return a.Lookup(wavelength) + b.Lookup(wavelength)
// 	})
// }

// // Scaled returns a new Distribution that is a scaled version of the given
// // distribution.
// func Scaled(d Distribution, n float64) Distribution {
// 	return DistributionFunc(func(wavelength float64) float64 {
// 		return n * d.Lookup(wavelength)
// 	})
// }
