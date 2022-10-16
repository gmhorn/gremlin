package util

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolveQuadratic(t *testing.T) {
	t.Run("two real roots", func(t *testing.T) {
		r1, r2, result := SolveQuadratic(1, -4, 3)
		assert.True(t, result)
		assert.Equal(t, r1, 1.0)
		assert.Equal(t, r2, 3.0)
	})

	t.Run("one real root", func(t *testing.T) {
		r1, r2, result := SolveQuadratic(1, -4, 4)
		assert.True(t, result)
		assert.Equal(t, r1, 2.0)
		assert.Equal(t, r2, 2.0)
	})

	t.Run("no real roots", func(t *testing.T) {
		_, _, result := SolveQuadratic(1, 0, 1)
		assert.False(t, result)
	})
}

func TestPartition(t *testing.T) {
	tests := []struct {
		name        string
		elems, size int
		expected    []Bin
	}{{
		"empty case",
		0, 16,
		[]Bin{},
	}, {
		"single evenly divides",
		4, 4,
		[]Bin{{0, 4}},
	}, {
		"single does not divide",
		3, 4,
		[]Bin{{0, 3}},
	}, {
		"multiple evenly divides",
		16, 4,
		[]Bin{{0, 4}, {4, 4}, {8, 4}, {12, 4}},
	}, {
		"mutliple does not divide",
		15, 4,
		[]Bin{{0, 4}, {4, 4}, {8, 4}, {12, 3}},
	}}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.expected, Partition(tt.elems, tt.size))
		})
	}
}
