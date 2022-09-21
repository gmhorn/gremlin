package geo

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMtx_Mult(t *testing.T) {
	a := &Mtx{
		{5, 7, 9, 10},
		{2, 3, 3, 8},
		{8, 10, 2, 3},
		{3, 3, 4, 8}}
	b := &Mtx{
		{3, 10, 12, 18},
		{12, 1, 4, 9},
		{9, 10, 12, 2},
		{3, 12, 4, 10}}
	assert.Equal(t, &Mtx{
		{210, 267, 236, 271},
		{93, 149, 104, 149},
		{171, 146, 172, 268},
		{105, 169, 128, 169},
	}, a.Mult(b))
}

func TestMtx_Transpose(t *testing.T) {
	m := Mtx{
		{10, 11, 12, 13},
		{14, 15, 16, 17},
		{18, 19, 20, 21},
		{22, 23, 24, 25}}

	assert.Equal(t, &Mtx{
		{10, 14, 18, 22},
		{11, 15, 19, 23},
		{12, 16, 20, 24},
		{13, 17, 21, 25}}, m.T())
}

func TestMtx_Inv(t *testing.T) {
	a := &Mtx{
		{3, 4, 6, 8},
		{1, 2, 7, 2},
		{8, 9, 1, 3},
		{7, 7, 6, 2},
	}

	b := a.Inv()

	c := a.Mult(b)

	fmt.Println(c)
}
