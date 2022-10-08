package shape

import "github.com/gmhorn/gremlin/pkg/geo"

type Intersection struct {
	Shape Shape
	T     float64
}

type Shape interface {

	// Intersect returns the closest intersection of the ray with this primitive.
	// A negative value means it does not intersect the primitive.
	Intersect(ray *geo.Ray) float64
	Normal(point geo.Vec) geo.Unit
}
