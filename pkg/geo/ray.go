package geo

// Ray is a geometric ray.
//
// Origin is a vector defining the point the ray originates from. Dir is the
// vector (not necessarily normalized!) that defines the ray's direction.

// Ray structs also contain non-public members that are mostly used for
// accelerating intersection tests with Bounds struct. As result:
// - Never construct these structs directly. Always use NewRay
// - Never modify the public members of these structs. Consider them read-only
type Ray struct {
	Origin Vec
	Dir    Vec

	invDir Vec
	sign   [3]int
}

// NewRay creates a new Ray at the given origin and direction
func NewRay(origin, dir Vec) *Ray {
	if dir.NearZero() {
		panic("Cannot create Ray with 0-direction")
	}

	ray := &Ray{
		Origin: origin,
		Dir:    dir,
	}

	// calculate reciprocals and signs
	// sign = (int) (recip < 0) but since Go doesn't have casting from
	// bool to in, have to do it in an explicit if-block
	for i, d := range dir {
		ray.invDir[i] = 1 / d
		if ray.invDir[i] < 0 {
			ray.sign[i] = 1
		}
	}

	return ray
}

// At returns a Vec3 that gives the position along the Ray at distance t.
func (r *Ray) At(t float64) Vec {
	return r.Origin.Plus(r.Dir.Scale(t))
}
