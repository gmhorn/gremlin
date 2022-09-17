package geo

import (
	"fmt"
	"math"
)

// Origin is the unique 0-vector representing the origin of coordinate space.
var Origin = Vector{0, 0, 0}

// Vector is "real-valued" (float64-valued) vector in R3. I went back and forth
// between just making this a typedef to [3]float64; ultimately the explicit X,
// Y, Z members seemed to make for better code ~*~aesthetics~*~.
type Vector struct{ X, Y, Z float64 }

// Plus returns the Vector a + b.
func (a Vector) Plus(b Vector) Vector {
	return Vector{a.X + b.X, a.Y + b.Y, a.Z + b.Z}
}

// Minus returns the Vector a - b.
func (a Vector) Minus(b Vector) Vector {
	return Vector{a.X - b.X, a.Y - b.Y, a.Z - b.Z}
}

// Scale returns a copy of this vector, scaled by t.
func (a Vector) Scale(t float64) Vector {
	return Vector{t * a.X, t * a.Y, t * a.Z}
}

// Dot returns the dot product of this vector with b.
func (a Vector) Dot(b Vector) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z
}

// Cross returns the cross product of this Vector with b.
func (a Vector) Cross(b Vector) Vector {
	return Vector{
		a.Y*b.Z - a.Z*b.Y,
		a.Z*b.X - a.X*b.Z,
		a.X*b.Y - a.Y*b.X,
	}
}

// Unit normalizes this Vector.
func (a Vector) Unit() (Unit, bool) {
	d := a.Len()
	return Unit(a.Scale(1 / d)), d > 0
}

// Len returns the length of this Vector.
func (a Vector) Len() float64 {
	return math.Sqrt(a.X*a.X + a.Y*a.Y + a.Z*a.Z)
}

// HasNaNs returns true if any component of this Vector is an IEEE 754
// NaN.
func (a Vector) HasNaNs() bool {
	return math.IsNaN(a.X) || math.IsNaN(a.Y) || math.IsNaN(a.Z)
}

// String returns a string representation of this Vector.
func (a *Vector) String() string {
	if a == nil {
		return ""
	}
	return fmt.Sprintf("%g,%g,%g", a.X, a.Y, a.Z)
}
