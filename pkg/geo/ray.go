package geo

// Ray is a geometric ray, with origin given by a Vec3 and direction given
// by a Unit vector.
type Ray struct {
	Origin Vec3
	Dir    Unit
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vec3 {
	return r.Origin.Plus(r.Dir.Scale(t))
}
