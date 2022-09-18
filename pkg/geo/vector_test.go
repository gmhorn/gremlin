package geo

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPlus(t *testing.T) {
	a := Vector{1, 2, 3}
	b := Vector{4, 5, 6}
	actual := a.Plus(b)

	assert.Equal(t, Vector{5, 7, 9}, actual)
}

func TestNaN(t *testing.T) {
	var a, b float64
	a = 123.4
	b = 0
	c := a / b
	t.Log("IsNaN?", math.IsNaN(c))
	t.Log("IsInf?", math.IsInf(c, 0))
}
