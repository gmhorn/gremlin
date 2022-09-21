package geo

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

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
	assert.Equal(t, &Matrix{
		{210, 267, 236, 271},
		{93, 149, 104, 149},
		{171, 146, 172, 268},
		{105, 169, 128, 169},
	}, a.Mult(b))
}

func TestMatrixTranspose(t *testing.T) {
	m := Matrix{
		{10, 11, 12, 13},
		{14, 15, 16, 17},
		{18, 19, 20, 21},
		{22, 23, 24, 25}}

	assert.Equal(t, &Matrix{
		{10, 14, 18, 22},
		{11, 15, 19, 23},
		{12, 16, 20, 24},
		{13, 17, 21, 25}}, m.T())
}

func TestMatrixInv(t *testing.T) {
	a := &Matrix{
		{3, 4, 6, 8},
		{1, 2, 7, 2},
		{8, 9, 1, 3},
		{7, 7, 6, 2},
	}

	b := a.Inv()

	c := a.Mult(b)

	fmt.Println(c)
}
