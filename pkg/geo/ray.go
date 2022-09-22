package geo

// Ray is a geometric ray.
type Ray struct {
	Origin Vec
	Dir    Unit
}

// NewRay is a convenience constructor for the Ray struct. Mostly because
//
//	r := &Ray{Origin: origin, Dir: dir}
//
// is much more tedious than
//
//	r := NewRay(origin, dir)
func NewRay(origin Vec, dir Unit) *Ray {
	return &Ray{origin, dir}
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vec {
	return r.Origin.Plus(r.Dir.Scale(t))
}
