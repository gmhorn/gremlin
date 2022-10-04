package geo

import "fmt"

// Axis vectors
var (
	XAxis = Unit{1, 0, 0}
	YAxis = Unit{0, 1, 0}
	ZAxis = Unit{0, 0, 1}
)

// Unit is a unit vector (vector of length 1).
//
// Go doesn't have method overloading, implicit type conversion, or good support
// for "constraint-based" types (e.g. Unit is a type of Vec with Vec.Len() == 1
// identically). However, in a lot of code its very important to distinguish
// between when we're using an arbitrary vector vs a normalized one. So we'll
// maintain this separate type and force the code to be explicit. The price is
// needing to redefine Vec operations for the Unit struct, and a some extra type
// coercions. But it pays off overall.
type Unit Vec

// Scale returns this unit vector scaled by t.
func (u Unit) Scale(t float64) Vec {
	return Vec{t * u.X, t * u.Y, t * u.Z}
}

// Reverse returns a copy of this unit vector with all components flipped.
func (u Unit) Reverse() Unit {
	return Unit{-u.X, -u.Y, -u.Z}
}

// Dot returns the dot product of this unit vector with v.
func (u Unit) Dot(v Unit) float64 {
	return u.X*v.X + u.Y*v.Y + u.Z*v.Z
}

// Cross returns the cross product of this unit vector with v. Note that in
// general this will not itself be a unit vector.
func (u Unit) Cross(v Unit) Vec {
	return Vec(u).Cross(Vec(v))
}

// HasInf returns true if any of this unit vector's components are positive or
// negative infinity. Useful if you've called Unit() on a vector you're not sure
// is secretly a 0-vector.
func (u Unit) HasInfs() bool {
	return Vec(u).HasInfs()
}

// Enters returns whether this unit vector is entering the plane represented
// by the normal.
func (u Unit) Enters(normal Unit) bool {
	return normal.Dot(u) < 0
}

// String returns a string representation of this unit vector.
func (u *Unit) String() string {
	return fmt.Sprintf("Unit(%5f, %5f, %5f)", u.X, u.Y, u.Z)
}
