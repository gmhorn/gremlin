package geo

import (
	"fmt"
	"testing"
)

var benchResultRay *Ray

func TestRay_At(t *testing.T) {
	tests := []struct {
		ray      *Ray
		t        float64
		expected Vec
	}{{
		ray:      NewRay(Origin, Vec(YAxis)),
		t:        2.5,
		expected: YAxis.Scale(2.5),
	}}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			actual := tt.ray.At(tt.t)
			assertVecEqual(t, tt.expected, actual, 0.0001)
		})
	}
}

func BenchmarkNewRay(b *testing.B) {
	for i := 0; i < b.N; i++ {
		benchResultRay = NewRay(Origin, V(1, 2, float64(i)))
	}
}

func BenchmarkRay_At(b *testing.B) {
	ray := NewRay(Origin, V(1, 2, 3))
	for i := 0; i < b.N; i++ {
		benchResultVec = ray.At(float64(i))
	}
}
