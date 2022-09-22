package geo

import (
	"fmt"
	"math"
)

// Origin vector.
var Origin = Vec{0, 0, 0}

// Vec is "real-valued" (float64-valued) vector in R3. I went back and forth
// between this typedef and a struct with X, Y, Z members; ultimately having
// the array typedef allows for cleaner calling code. Specifically,
//
//	v := Vec{1.2, 0, 7.5}
//
// gives an ugly warning for the struct-style. The tradeoff is slightly uglier
// implementation code.
type Vec [3]float64

// Plus returns the vector a + b.
func (a Vec) Plus(b Vec) Vec {
	return Vec{a[0] + b[0], a[1] + b[1], a[2] + b[2]}
}

// Minus returns the vector a - b.
func (a Vec) Minus(b Vec) Vec {
	return Vec{a[0] - b[0], a[1] - b[1], a[2] - b[2]}
}

// Scale returns a copy of this vector scaled by t.
func (a Vec) Scale(t float64) Vec {
	return Vec{t * a[0], t * a[1], t * a[2]}
}

// Reverse returns a copy of this vector with all components flipped.
func (a Vec) Reverse() Vec {
	return Vec{-a[0], -a[1], -a[2]}
}

// Dot returns the dot product of this vector with b.
func (a Vec) Dot(b Vec) float64 {
	return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

// Cross returns the cross product of this vector with b.
func (a Vec) Cross(b Vec) Vec {
	return Vec{
		a[1]*b[2] - a[2]*b[1],
		a[2]*b[0] - a[0]*b[2],
		a[0]*b[1] - a[1]*b[0],
	}
}

// Unit normalizes this vector.
func (a Vec) Normalize() (Vec, bool) {
	n := 1.0 / a.Len()
	return Vec{n * a[0], n * a[1], n * a[2]}, math.IsInf(n, 0)
}

// Unit return the normalized vector.
func (a Vec) Unit() (Unit, bool) {
	n := 1.0 / a.Len()
	return Unit{n * a[0], n * a[1], n * a[2]}, math.IsInf(n, 0)
}

// Len returns the length of this vector.
func (a Vec) Len() float64 {
	return math.Sqrt(a[0]*a[0] + a[1]*a[1] + a[2]*a[2])
}

// HasNaNs returns true if any component of this vector is an IEEE 754
// NaN.
func (a Vec) HasNaNs() bool {
	return math.IsNaN(a[0]) || math.IsNaN(a[1]) || math.IsNaN(a[2])
}

// String returns a string representation of this vector.
func (a *Vec) String() string {
	return fmt.Sprintf("%g,%g,%g", a[0], a[1], a[2])
}
