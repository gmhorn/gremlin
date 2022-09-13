package spectrum

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDiscrete_Lookup(t *testing.T) {
	d := Discrete{}
	for idx := range d {
		d[idx] = float64(idx)
	}

	v := DiscreteWavelengths
	fmt.Println(v[0])

	tests := []struct {
		name       string
		wavelength int
		expected   int
	}{{
		name:       "below min",
		wavelength: discWavelengthMin - 1,
		expected:   0,
	}, {
		name:       "at min",
		wavelength: discWavelengthMin,
		expected:   0,
	}, {
		name:       "above min",
		wavelength: discWavelengthMin + 1,
		expected:   0,
	}, {
		name:       "at next step",
		wavelength: discWavelengthMin + discWavelengthStep,
		expected:   1,
	}, {
		name:       "above next step",
		wavelength: discWavelengthMin + discWavelengthStep + 1,
		expected:   1,
	}, {
		name:       "at max",
		wavelength: discWavelengthMax,
		expected:   DiscreteSize - 1,
	}, {
		name:       "above max",
		wavelength: discWavelengthMax + 1,
		expected:   DiscreteSize - 1,
	}}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, float64(tt.expected), d.Lookup(float64(tt.wavelength)))
		})
	}
}
