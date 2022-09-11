package spectrum

// Sellmeier gives the refractive index for a given wavelength thru a medium.
type Sellmeier struct {
	// B coefficients. Unitless.
	B [3]float64
	// C coefficients. Units of um^2 (square micrometers)
	C [3]float64
}

// Lookup returns the refractive index for a given wavelength. Wavelength should
// be given in units of nanometers
func (s Sellmeier) Lookup(wavelength float64) float64 {
	// convert wavelength to micrometers
	wavelength *= 1e-3
	wSquared := wavelength * wavelength

	n := 1.0
	for i := 0; i < 3; i++ {
		n += (s.B[i] * wSquared) / (wSquared - s.C[i])
	}
	return n
}
