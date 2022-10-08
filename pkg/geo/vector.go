package geo

import (
	"fmt"
	"math"
)

// Origin vector.
var Origin = Vec{0, 0, 0}

// Vec is "real-valued" (float64-valued) vector in R3.
//
// Initially implemented as a simply typedef of [3]float64, but for some reason
// Go can't optimize it nearly as well. Benchmarking shows this implementation
// is an order of magnitude faster.
type Vec struct {
	X, Y, Z float64
}

// V is a convenience constructor for a vector.
func V(x, y, z float64) Vec {
	return Vec{x, y, z}
}

// Reflected returns a new vector that is the reflection of the incident vector
// about the normal.
//
//	v - 2(v*n)*n
func Reflected(incident Vec, normal Unit) Vec {
	return incident.Minus(normal.Scale(2 * incident.Dot(Vec(normal))))
}

// VecMin returns a new vector that is the component-wise minimum. Useful for
// constructing bounding volumes.
func VecMin(a, b Vec) Vec {
	return Vec{
		X: math.Min(a.X, b.X),
		Y: math.Min(a.Y, b.Y),
		Z: math.Min(a.Z, b.Z),
	}
}

// VecMax returns a new vector that is the component-wise maximum. Useful for
// constructing bounding volumes.
func VecMax(a, b Vec) Vec {
	return Vec{
		X: math.Max(a.X, b.X),
		Y: math.Max(a.Y, b.Y),
		Z: math.Max(a.Z, b.Z),
	}
}

// Plus returns the vector a + b.
func (a Vec) Plus(b Vec) Vec {
	return Vec{a.X + b.X, a.Y + b.Y, a.Z + b.Z}
}

// Minus returns the vector a - b.
func (a Vec) Minus(b Vec) Vec {
	return Vec{a.X - b.X, a.Y - b.Y, a.Z - b.Z}
}

// Scale returns a copy of this vector scaled by t.
func (a Vec) Scale(t float64) Vec {
	return Vec{t * a.X, t * a.Y, t * a.Z}
}

// Reverse returns a copy of this vector with all components flipped.
func (a Vec) Reverse() Vec {
	return Vec{-a.X, -a.Y, -a.Z}
}

// Dot returns the dot product of this vector with b.
func (a Vec) Dot(b Vec) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z
}

// Cross returns the cross product of this vector with b.
func (a Vec) Cross(b Vec) Vec {
	return Vec{
		a.Y*b.Z - a.Z*b.Y,
		a.Z*b.X - a.X*b.Z,
		a.X*b.Y - a.Y*b.X,
	}
}

// Unit return the normalized vector. It won't check that you tried to normalize
// a 0-vector; use HasInfs on the result if you need to check.
func (a Vec) Unit() Unit {
	n := 1.0 / a.Len()
	return Unit{n * a.X, n * a.Y, n * a.Z}
}

// Len returns the length of this vector.
func (a Vec) Len() float64 {
	return math.Sqrt(a.X*a.X + a.Y*a.Y + a.Z*a.Z)
}

// LenSquared is a convenience for returning the squared-length. This is
// identitically equivalent to dotting the vector with itself, i.e.
//
//	a.LenSquared() == a.Dot(a)
//
// But is slightly more efficient, since it avoids a copy.
func (a Vec) LenSquared() float64 {
	return a.X*a.X + a.Y*a.Y + a.Z*a.Z
}

// HasNaNs returns true if any component of this vector is an IEEE 754
// NaN.
func (a Vec) HasNaNs() bool {
	return math.IsNaN(a.X) || math.IsNaN(a.Y) || math.IsNaN(a.Z)
}

// HasInfs checks if any of the vector's components are positive or negative
// infinity.
func (a Vec) HasInfs() bool {
	return math.IsInf(a.X, 0) || math.IsInf(a.Y, 0) || math.IsInf(a.Z, 0)
}

// NearZero returns true if a vector is "pretty close" to zero.
func (a Vec) NearZero() bool {
	return math.Abs(a.X) < epsilon && math.Abs(a.Y) < epsilon && math.Abs(a.Z) < epsilon
}

// String returns a string representation of this vector.
func (a Vec) String() string {
	return fmt.Sprintf("Vec(%5f, %5f, %5f)", a.X, a.Y, a.Z)
}
