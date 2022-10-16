package spectrum

import "sort"

// Wavelength minimum, maximum, increment, and total number of values that
// Sampled spectra are defined at.
const (
	SampledMin  = 380
	SampledMax  = 780
	SampledStep = 5
	NumSamples  = ((SampledMax - SampledMin) / SampledStep) + 1
)

var sampledWavelengths = func() []float64 {
	s := make([]float64, 0, NumSamples)
	for w := SampledMin; w <= SampledMax; w += SampledStep {
		s = append(s, float64(w))
	}
	return s
}()

// Sampled is a distribution with values defined for a range of fixed, uniformly
// distributed wavelengths. Conceptually, it is a distribution that has been
// "sampled" at those wavelengths.
//
// Structurally, its a simple typedef for a fixed-length float64 array. This
// makes it very easy to create new instances, compared to a float64 slice:
//
//	dist := new(Sampled)
//
// The downside is that arrays, unlike slices, have value-like semantics when
// passed into functions. You almost always want to pass these by pointer to
// avoid expensive copying.
type Sampled [NumSamples]float64

// Sample returns a new Sampled distribution, obtained by evaluation the given
// distribution at fixed wavelengths. If the given distribution is already
// a *Sampled, then just returns that.
func Sample(dist Distribution) *Sampled {
	if s, ok := dist.(*Sampled); ok {
		return s
	}

	s := new(Sampled)
	for i, w := range sampledWavelengths {
		s[i] = dist.Lookup(w)
	}
	return s
}

// Lookup returns the closest sampled value greater than or equal to the given
// wavelength. Returns 0 if the wavelength is outside the range given by
//
//	[SampledMin, SampledMax]
//
// TODO: Should this even implement the Distribution interface?
func (s *Sampled) Lookup(wavelength float64) float64 {
	if wavelength < SampledMin || wavelength > SampledMax {
		return 0
	}

	idx := sort.SearchFloat64s(sampledWavelengths, wavelength)
	return s[idx]
}

func (s *Sampled) Plus(t *Sampled) *Sampled {
	r := new(Sampled)
	for i, v := range s {
		r[i] = v + t[i]
	}
	return r
}

func (s *Sampled) Scale(n float64) *Sampled {
	t := new(Sampled)
	for i, v := range s {
		t[i] = n * v
	}
	return t
}

func (s *Sampled) Lerp(t *Sampled, n float64) *Sampled {
	lerp := new(Sampled)
	m := 1 - n
	for i, v := range s {
		lerp[i] = v*n + t[i]*m
	}
	return lerp
}
