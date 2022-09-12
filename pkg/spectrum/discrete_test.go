package spectrum

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDiscrete_Lookup(t *testing.T) {
	d := Discrete{}
	for idx := range d {
		d[idx] = float64(idx)
	}

	assert.Equal(t, 0.0, d.Lookup(200))
	assert.Equal(t, 0.0, d.Lookup(400))
	assert.Equal(t, 1.0, d.Lookup(405))
	assert.Equal(t, 1.0, d.Lookup(406))
	assert.Equal(t, 59.0, d.Lookup(700))
	assert.Equal(t, 59.0, d.Lookup(900))
}
