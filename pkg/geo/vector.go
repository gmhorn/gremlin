package geo

import (
	"fmt"
	"math"
)

// Origin is the unique 0-vector representing the origin of coordinate space.
var Origin = Vec3{0, 0, 0}

// Vec3 is a vector in R3. Basis is the standard R3 basis. Coordinate values
// are represented as 64-bit floats.
type Vec3 [3]float64

func (a Vec3) Plus(b Vec3) Vec3 {
	return Vec3{a[0] + b[0], a[1] + b[1], a[2] + b[2]}
}

func (a Vec3) Minus(b Vec3) Vec3 {
	return Vec3{a[0] - b[0], a[1] - b[1], a[2] - b[2]}
}

func (a Vec3) Scale(t float64) Vec3 {
	return Vec3{t * a[0], t * a[1], t * a[2]}
}

func (a Vec3) Dot(b Vec3) float64 {
	return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

func (a Vec3) Cross(b Vec3) Vec3 {
	return Vec3{
		a[1]*b[2] - a[2]*b[1],
		a[2]*b[0] - a[0]*b[2],
		a[0]*b[1] - a[1]*b[0],
	}
}

func (a Vec3) Unit() Vec3 {
	return a.Scale(1 / a.Len())
}

func (a Vec3) Len() float64 {
	return math.Sqrt(a[0]*a[0] + a[1]*a[1] + a[2]*a[2])
}

func (a *Vec3) String() string {
	if a == nil {
		return ""
	}
	return fmt.Sprintf("%g,%g,%g", a[0], a[1], a[2])
}
