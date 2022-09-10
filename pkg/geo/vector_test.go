package geo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPlus(t *testing.T) {
	a := Vec3{1, 2, 3}
	b := Vec3{4, 5, 6}
	actual := a.Plus(b)

	assert.Equal(t, Vec3{5, 7, 9}, actual)
}
