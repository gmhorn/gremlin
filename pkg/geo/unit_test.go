package geo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUnit_Cross(t *testing.T) {
	u := XAxis
	v, _ := V(1, 1, 0).Unit()
	w := Vec(u).Cross(Vec(v))
	l := w.Len()

	assert.Equal(t, 1.0, l)
}
