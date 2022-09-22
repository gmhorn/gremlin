package geo

import "fmt"

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
	return Vec{t * u[0], t * u[1], t * u[2]}
}

// Reverse returns a copy of this unit vector with all components flipped.
func (u Unit) Reverse() Unit {
	return Unit{-u[0], -u[1], -u[2]}
}

// Dot returns the dot product of this unit vector with v.
func (u Unit) Dot(v Unit) float64 {
	return u[0]*v[0] + u[1]*v[1] + u[2]*v[2]
}

// Cross returns the cross product of this unit vector with v.
func (u Unit) Cross(v Unit) (Unit, bool) {
	// Can't assume always a unit because u.Cross(u) is 0-vector.
	return Vec(u).Cross(Vec(v)).Unit()
}

// String returns a string representation of this unit vector.
func (u *Unit) String() string {
	return fmt.Sprintf("%g,%g,%g", u[0], u[1], u[2])
}
