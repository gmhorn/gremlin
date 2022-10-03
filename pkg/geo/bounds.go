package geo

import "math"

// Bounds is an axis-aligned bounding box ("AABB").
type Bounds struct {
	min, max Vec
}

func NewBounds(p1, p2 Vec) *Bounds {
	return &Bounds{
		min: vecMin(p1, p2),
		max: vecMax(p1, p2),
	}
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
