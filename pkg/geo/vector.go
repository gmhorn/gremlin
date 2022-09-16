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

func (a Vector) Plus(b Vector) Vector {
	return Vector{a.X + b.X, a.Y + b.Y, a.Z + b.Z}
}

func (a Vector) Minus(b Vector) Vector {
	return Vector{a.X - b.X, a.Y - b.Y, a.Z - b.Z}
}

func (a Vector) Scale(t float64) Vector {
	return Vector{t * a.X, t * a.Y, t * a.Z}
}

func (a Vector) Dot(b Vector) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z
}

func (a Vector) Cross(b Vector) Vector {
	return Vector{
		a.Y*b.Z - a.Z*b.Y,
		a.Z*b.X - a.X*b.Z,
		a.X*b.Y - a.Y*b.X,
	}
}

func (a Vector) Unit() Vector {
	return a.Scale(1 / a.Len())
}

func (a Vector) Len() float64 {
	return math.Sqrt(a.X*a.X + a.Y*a.Y + a.Z*a.Z)
}

func (a *Vector) String() string {
	if a == nil {
		return ""
	}
	return fmt.Sprintf("%g,%g,%g", a.X, a.Y, a.Z)
}
