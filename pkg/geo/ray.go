package geo

// Ray is a geometric ray, with origin given by a Vector and direction given
// by a Unit.
type Ray struct {
	Origin Vector
	Dir    Unit
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vector {
	return r.Origin.Plus(r.Dir.Scale(t))
}
