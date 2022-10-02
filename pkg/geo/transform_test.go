package geo

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestShift(t *testing.T) {
	m := Shift(Vec{10, 20, 30})

	assert.Equal(t, Vec{1, 2, 3}, m.MultVec(Vec{1, 2, 3}))
	assert.Equal(t, Vec{11, 22, 33}, m.MultPoint(Vec{1, 2, 3}))
}

func TestScale(t *testing.T) {
	m := Scale(Vec{10, 20, 30})

	assert.Equal(t, Vec{10, 20, 30}, m.MultVec(Vec{1, 1, 1}))
	assert.Equal(t, Vec{10, 20, 30}, m.MultPoint(Vec{1, 1, 1}))
}

func TestLookAt(t *testing.T) {
	eye := Vec{10, 10, 10}
	target := Origin
	m := LookAt(eye, target, YAxis)

	r := NewRay(Origin, Unit{0, 0, -1})

	assert.Equal(t, eye, m.MultPoint(r.Origin))

	c := 1.0 / math.Sqrt(3.0)
	assertVecEqual(t, Vec{-c, -c, -c}, m.MultUnit(r.Dir), 0.00001)
}
