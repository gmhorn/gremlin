package geo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPlus(t *testing.T) {
	a := Vector{1, 2, 3}
	b := Vector{4, 5, 6}
	actual := a.Plus(b)

	assert.Equal(t, Vector{5, 7, 9}, actual)
}
