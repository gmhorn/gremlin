package spectrum

import (
	"sort"
)

const (
	discWavelengthMin  = 380
	discWavelengthMax  = 780
	discWavelengthStep = 5
)

// DiscreteSize is the number of wavelengths in a Discrete Distribution. That
// is, it's the size of the underlying array.
const DiscreteSize = (discWavelengthMax-discWavelengthMin)/discWavelengthStep + 1

// DiscreteWavelengths holds the fixed wavelength values every Discrete
// distribution is measured at. It is intended to make it easy to iterate over
// Discrete instances.
//
//	sun := Blackbody(5700)
//	spec := Discrete{}
//	for idx, wavelength := range DiscreteWavelengths {
//	  spec[idx] = sun(wavelength)
//	}
var DiscreteWavelengths = _discreteWavelengths()

// Discrete is a distribution with values defined at uniformly distributed
// discrete wavelengths.
//
// The specific wavelengths are fixed and given by the DiscreteWavelengths
// variable. This lets us operate on Discrete instances in a sensible way, such
// as summing and merging them.
type Discrete [DiscreteSize]float64

func (d *Discrete) Lookup(wavelength float64) float64 {
	if wavelength <= discWavelengthMin {
		return d[0]
	}

	idx := sort.Search(len(DiscreteWavelengths), func(i int) bool {
		return DiscreteWavelengths[i] > wavelength
	})
	return d[idx-1]
}

// Lerp returns a new Discrete distribution that is a linear interpolation.
func (d *Discrete) Lerp(other *Discrete, t float64) *Discrete {
	lerp := Discrete{}
	s := 1 - t
	for i := range d {
		lerp[i] = d[i]*s + other[i]*t
	}
	return &lerp
}

// Discretize returns the Discrete spectrum obtained by evaluting the given
// Distribution at the fixed DiscreteWavelengths values.
func Discretize(dist Distribution) *Discrete {
	// If its already a Discrete instance, then we're done.
	if discrete, ok := dist.(*Discrete); ok {
		return discrete
	}

	discrete := new(Discrete)
	for i, wavelength := range DiscreteWavelengths {
		discrete[i] = dist.Lookup(wavelength)
	}
	return discrete
}

func _discreteWavelengths() Discrete {
	d := Discrete{}
	for i := 0; i < len(d); i++ {
		d[i] = float64(discWavelengthMin + (i * discWavelengthStep))
	}
	return d
}
