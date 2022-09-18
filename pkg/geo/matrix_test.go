package geo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLookAt(t *testing.T) {
	from := Vec{1, 1, 1}
	to := Origin
	actual := LookAt(from, to)

	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[0][0], actual[0][1], actual[0][2], actual[0][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[1][0], actual[1][1], actual[1][2], actual[1][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[2][0], actual[2][1], actual[2][2], actual[2][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[3][0], actual[3][1], actual[3][2], actual[3][3])
}

func TestMatrixMult(t *testing.T) {
	a := &Matrix{
		{5, 7, 9, 10},
		{2, 3, 3, 8},
		{8, 10, 2, 3},
		{3, 3, 4, 8}}
	b := &Matrix{
		{3, 10, 12, 18},
		{12, 1, 4, 9},
		{9, 10, 12, 2},
		{3, 12, 4, 10}}
	assert.Equal(t, Matrix{
		{210, 267, 236, 271},
		{93, 149, 104, 149},
		{171, 146, 172, 268},
		{105, 169, 128, 169},
	}, *(a.Mult(b)))
}
