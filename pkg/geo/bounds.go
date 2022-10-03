package geo

import "math"

// Bounds is an axis-aligned bounding box ("AABB"). It is defined by its minimum
// and maximum points.
type Bounds [2]Vec

// NewBounds constructs a new AABB from the two points given.
func NewBounds(p1, p2 Vec) *Bounds {
	return &Bounds{vecMin(p1, p2), vecMax(p1, p2)}
}

// Intersect tests if the ray intersects the bounds. If it does, it returns the
// two t values in ascending order and the value true. Otherwise it returns
// false and garbage t values. Always check the returned boolean.
//
// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
func (b *Bounds) Intersect(ray *Ray) (t0, t1 float64, found bool) {
	return
}

// return the vector that is the component-wise minimum of the two vectors
func vecMin(a, b Vec) Vec {
	return Vec{
		math.Min(a[0], b[0]),
		math.Min(a[1], b[1]),
		math.Min(a[2], b[2]),
	}
}

// return the vector that is the component-wise maximum of the two vectors
func vecMax(a, b Vec) Vec {
	return Vec{
		math.Max(a[0], b[0]),
		math.Max(a[1], b[1]),
		math.Max(a[2], b[2]),
	}
}
