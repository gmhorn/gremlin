package geo

import (
	"fmt"
	"math"
)

// Origin is the unique 0-vector representing the origin of coordinate space.
var Origin = Vec3{0, 0, 0}

// Vec3 is a vector in R3. Basis is the standard R3 basis. Coordinate values
// are represented as 64-bit floats.
type Vec3 struct {
	X, Y, Z float64
}

func (a Vec3) Plus(b Vec3) Vec3 {
	return Vec3{a.X + b.X, a.Y + b.Y, a.Z + b.Z}
}

func (a Vec3) Scale(t float64) Vec3 {
	return Vec3{t * a.X, t * a.Y, t * a.Z}
}

func (a Vec3) Dot(b Vec3) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z
}

func (a Vec3) Cross(b Vec3) Vec3 {
	return Vec3{
		a.Y*b.Z - a.Z*b.Y,
		a.Z*b.X - a.X*b.Z,
		a.X*b.Y - a.Y*b.X,
	}
}

func (a Vec3) Unit() Vec3 {
	return a.Scale(1 / a.Len())
}

func (a Vec3) Len() float64 {
	return math.Sqrt(a.X*a.X + a.Y*a.Y + a.Z*a.Z)
}

func (a *Vec3) String() string {
	if a == nil {
		return ""
	}
	return fmt.Sprintf("%g,%g,%g", a.X, a.Y, a.Z)
}
