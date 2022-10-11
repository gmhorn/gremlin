package spectrum

import "sort"

const (
	sampledMin  = 380
	sampledMax  = 780
	sampledStep = 5
	numSamples  = ((sampledMax - sampledMin) / sampledStep) + 1
)

var sampledWavelengths = func() []float64 {
	s := make([]float64, 0, numSamples)
	for w := sampledMin; w <= sampledMax; w += sampledStep {
		s = append(s, float64(w))
	}
	return s
}()

type Sampled [numSamples]float64

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

func (s *Sampled) Lookup(wavelength float64) float64 {
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
