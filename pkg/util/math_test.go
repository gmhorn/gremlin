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
