package primitive

import "github.com/gmhorn/gremlin/pkg/geo"

type Intersection struct {
	Primitive Primitive
	T         float64
}

type Primitive interface {

	// Intersect returns the closest intersection of the ray with this primitive.
	// A negative value means it does not intersect the primitive.
	Intersect(ray *geo.Ray) float64
	Normal(point geo.Vec) geo.Unit
}
