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

// Dot returns the dot product of two units. Also happens to equal the cosine of
// the angle between them.
func (u Unit) Dot(v Unit) float64 {
	return u[0]*v[0] + u[1]*v[1] + u[2]*v[2]
}

// Enters returns whether this Unit is entering the plane represented by the
// normal.
func (u Unit) Enters(normal Unit) bool {
	return normal.Dot(u) < 0
}

// Reverse reverses the direction of a Unit vector.
func (u Unit) Reverse() Unit {
	return Unit{-u[0], -u[1], -u[2]}
}
