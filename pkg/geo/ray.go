package geo

// Ray is a geometric ray.
//
// Origin is a vector defining the point the ray originates from. Dir is the
// vector (not necessarily normalized!) that defines the ray's direction. InvDir
// is the vector formed by the component-wise reciprocals of the ray's
// direction vector. The majority of the time, rays will be used as input to
// Bounds.Intersect() and having these pre-computed speeds things up.
//
// The upshot is, hand-constucting Ray structs directly, or directly
// manipulating its components, can lead to trouble! Make sure to either update
// both Dir and InvDir at the same time, or just use the NewRay function.
type Ray struct {
	Origin, Dir, InvDir Vec
}

// NewRay creates a new Ray at the given origin and direction
func NewRay(origin, dir Vec) *Ray {
	if dir.NearZero() {
		panic("Cannot create Ray with 0-direction")
	}
	return &Ray{
		Origin: origin,
		Dir:    dir,
		InvDir: Vec{1 / dir[0], 1 / dir[1], 1 / dir[2]},
	}
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vec {
	return r.Origin.Plus(r.Dir.Scale(t))
}
