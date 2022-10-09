package colorspace

import (
	"fmt"
	"testing"

	"github.com/gmhorn/gremlin/pkg/spectrum"
	"github.com/stretchr/testify/assert"
)

func TestCIE1931_Convert(t *testing.T) {
	tests := map[float64][3]float64{
		2000: {0.5267, 0.4133, 0.0600},
		2500: {0.4770, 0.4137, 0.1093},
		3000: {0.4369, 0.4041, 0.1590},
		3500: {0.4053, 0.3907, 0.2040},
		4000: {0.3805, 0.3768, 0.2428},
		4500: {0.3608, 0.3636, 0.2756},
		5000: {0.3451, 0.3516, 0.3032},
		5500: {0.3325, 0.3411, 0.3265},
		6000: {0.3221, 0.3318, 0.3461},
		6500: {0.3135, 0.3237, 0.3628},
		7000: {0.3064, 0.3166, 0.3770},
		7500: {0.3004, 0.3103, 0.3893},
		8000: {0.2952, 0.3048, 0.4000},
		8500: {0.2908, 0.3000, 0.4093},
		9000: {0.2869, 0.2956, 0.4174},
		9500: {0.2836, 0.2918, 0.4246},
	}

	for temp, expected := range tests {
		t.Run(fmt.Sprintf("%gK Blackbody", temp), func(t *testing.T) {
			actual := CIE1931.Convert(spectrum.Blackbody(temp))

			assert.InEpsilon(t, expected[0], actual[0], 1e-3)
			assert.InEpsilon(t, expected[1], actual[1], 1e-3)
			assert.InEpsilon(t, expected[2], actual[2], 1e-3)
		})
	}
}

var spectra = []spectrum.Distribution{
	spectrum.Discretize(spectrum.Blackbody(2000)),
	spectrum.Discretize(spectrum.Blackbody(2500)),
	spectrum.Discretize(spectrum.Blackbody(3000)),
	spectrum.Discretize(spectrum.Blackbody(3500)),
	spectrum.Discretize(spectrum.Blackbody(4000)),
	spectrum.Discretize(spectrum.Blackbody(4500)),
	spectrum.Discretize(spectrum.Blackbody(5000)),
	spectrum.Discretize(spectrum.Blackbody(5500)),
	spectrum.Discretize(spectrum.Blackbody(6000)),
	spectrum.Discretize(spectrum.Blackbody(6500)),
}
var numSpectra = len(spectra)
var result Point

func BenchmarkCIE1931_Convert(b *testing.B) {
	for i := 0; i < b.N; i++ {
		result = CIE1931.Convert(spectra[i%numSpectra])
	}
}
