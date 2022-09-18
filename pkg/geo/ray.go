package geo

// Ray is a geometric ray, with origin given by a Vec and direction given
// by a Unit.
type Ray struct {
	Origin Vec
	Dir    Unit
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vec {
	return r.Origin.Plus(r.Dir.Scale(t))
}
