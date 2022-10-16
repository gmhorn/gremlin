package spectrum

import "math"

// Blackbody is the spectrum around a black-body of a given temperature. Units
// are Kelvin.
type Blackbody float64

// https://en.wikipedia.org/wiki/Planck%27s_law#First_and_second_radiation_constants
const c1 = 3.74177e-16
const c2 = 1.43879e-2

// Lookup returns the spectral radiant existance (power per unit area per unit
// wavelength). Wavelength should be given in units of nanometers
// https://en.wikipedia.org/wiki/Planckian_locus
func (temp Blackbody) Lookup(wavelength float64) float64 {
	// Convert the wavelength to meters.
	wavelength *= 1e-9

	// Apply Plank's Law.
	powerTerm := c1 * math.Pow(wavelength, -5.0)
	return powerTerm / (math.Exp(c2/(wavelength*float64(temp))) - 1.0)
}
