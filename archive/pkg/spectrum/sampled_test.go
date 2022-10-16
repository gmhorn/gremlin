package spectrum

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var benchResultSampled *Sampled

func TestSampled_Lookup(t *testing.T) {
	tests := []struct {
		name       string
		wavelength float64
		expected   float64
	}{{
		name:       "below min",
		wavelength: SampledMin - 1,
		expected:   0,
	}, {
		name:       "at min",
		wavelength: SampledMin,
		expected:   SampledMin,
	}, {
		name:       "at defined value",
		wavelength: SampledMin + 4*SampledStep,
		expected:   SampledMin + 4*SampledStep,
	}, {
		name:       "between values",
		wavelength: SampledMin + 4.2*SampledStep,
		expected:   SampledMin + 5*SampledStep,
	}, {
		name:       "at max",
		wavelength: SampledMax,
		expected:   SampledMax,
	}, {
		name:       "above max",
		wavelength: SampledMax + 1,
		expected:   0,
	}}

	dist := new(Sampled)
	copy(dist[:], sampledWavelengths)

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.InDelta(t, tt.expected, dist.Lookup(tt.wavelength), 0.001)
		})
	}
}

func BenchmarkSample_AlreadySampled(b *testing.B) {
	dist := Sample(Blackbody(4500))
	for i := 0; i < b.N; i++ {
		benchResultSampled = Sample(dist)
	}
}

func BenchmarkSampled_Scale(b *testing.B) {
	dist := Sample(Blackbody(4500))
	for i := 0; i < b.N; i++ {
		benchResultSampled = dist.Scale(float64(i))
	}
}

func BenchmarkSampled_Plus(b *testing.B) {
	x := Sample(Blackbody(4500))
	y := Sample(Flat(0.1))

	for i := 0; i < b.N; i++ {
		benchResultSampled = x.Plus(y)
	}
}
