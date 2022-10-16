package geo

import (
	"fmt"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

var benchResultVec Vec
var benchResultFloat64 float64

func BenchmarkV(b *testing.B) {
	for i := 0; i < b.N; i++ {
		benchResultVec = V(float64(i), 2, 3)
	}
}

func TestVec_Plus(t *testing.T) {
	a := Vec{1, 2, 3}
	b := Vec{4, 5, 6}
	actual := a.Plus(b)

	assert.Equal(t, Vec{5, 7, 9}, actual)
}

func BenchmarkVec_Plus(b *testing.B) {
	u := Vec{1, 2, 3}
	for i := 0; i < b.N; i++ {
		benchResultVec = u.Plus(Vec{1, 2, float64(i)})
	}
}

func TestVec_Dot(t *testing.T) {
	tests := []struct {
		a, b     Vec
		expected float64
	}{{
		a:        Vec{1, 0, 0},
		b:        Vec{0, 1, 0},
		expected: 0,
	}, {
		a:        Vec{1, 0, 0},
		b:        Vec{2, 1, 0},
		expected: 2,
	}}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			actual := tt.a.Dot(tt.b)
			assert.InDelta(t, tt.expected, actual, 0.0001)
		})
	}
}

func BenchmarkVec_Dot(b *testing.B) {
	u := Vec{1, 2, 3}
	for i := 0; i < b.N; i++ {
		benchResultFloat64 = u.Dot(Vec{1, 2, float64(i)})
	}
}

func TestVec_Cross(t *testing.T) {
	tests := []struct {
		a, b, expected Vec
	}{{
		a:        Vec{1, 0, 0},
		b:        Vec{0, 1, 0},
		expected: Vec{0, 0, 1},
	}, {
		a:        Vec{0, 1, 0},
		b:        Vec{0, 0, 1},
		expected: Vec{1, 0, 0},
	}, {
		a:        Vec{0, 0, 1},
		b:        Vec{1, 0, 0},
		expected: Vec{0, 1, 0},
	}}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			actual := tt.a.Cross(tt.b)
			assertVecEqual(t, tt.expected, actual, 0.0001)
		})
	}
}

func BenchmarkVec_Cross(b *testing.B) {
	u := Vec{1, 2, 3}
	for i := 0; i < b.N; i++ {
		benchResultVec = u.Cross(Vec{1, 2, float64(i)})
	}
}

func TestNaN(t *testing.T) {
	var a, b float64
	a = 123.4
	b = 0
	c := a / b
	t.Log("IsNaN?", math.IsNaN(c))
	t.Log("IsInf?", math.IsInf(c, 0))
}

func assertVecEqual(t *testing.T, expected, actual Vec, epsilon float64) {
	dist := expected.Minus(actual).Len()
	assert.LessOrEqualf(t, dist, epsilon,
		"Expected close to %s, got %s (distance %g)", expected, actual, dist)
}
