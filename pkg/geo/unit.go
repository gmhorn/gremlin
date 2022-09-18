package geo

var (
	XAxis = Unit{1, 0, 0}
	YAxis = Unit{0, 1, 0}
	ZAxis = Unit{0, 0, 1}
)

// Unit is a unit vector (Vector of length 1).
type Unit Vec

// Scale scales the Unit vector by a scalar to return a new Vector.
func (u Unit) Scale(t float64) Vec {
	return Vec(u).Scale(t)
}

// Cross returns the cross product of this Unit with b.
func (u Unit) Cross(v Unit) (Unit, bool) {
	return Vec(u).Cross(Vec(v)).Unit()
}

func (u Unit) Dot(v Vec) float64 {
	return Vec(u).Dot(v)
}
