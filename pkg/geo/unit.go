package geo

// Unit is a unit vector (Vector of length 1).
type Unit Vector

// Scale scales the Unit vector by a scalar to return a new Vector.
func (u Unit) Scale(t float64) Vector {
	return Vector(u).Scale(t)
}
