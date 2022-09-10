package geo

// Unit is a unit vector (Vec3 of length 1).
type Unit Vec3

// Scale scales the Unit vector by a scalar to return a new Vec3.
func (u Unit) Scale(t float64) Vec3 {
	return Vec3(u).Scale(t)
}
