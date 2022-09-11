package spectrum

// Distribution represents a spectral distribution - a quantity that is a
// function of wavelength. Examples are reflectance, refractive index, radiance,
// etc.
//
// Typically these are implemented either analytically, or via lookup table and
// interpolation.
type Distribution interface {

	// Lookup evaluates the Distribution at a given wavelength. The wavelength
	// is in units of nanometers; visible spectrum is between 380 and 780. The
	// returned value and its units are Distribution-dependent.
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
	return DistributionFunc(func(f float64) float64 {
		return value
	})
}
