package shape

import (
	"github.com/gmhorn/gremlin/pkg/geo"
)

var Calls int

type Triangle struct {
	p1, p2, p3   geo.Vec
	edge1, edge2 geo.Vec
	normal       geo.Unit
	centroid     geo.Vec
}

func NewTriangle(p1, p2, p3 geo.Vec) *Triangle {
	tri := &Triangle{
		p1:    p1,
		p2:    p2,
		p3:    p3,
		edge1: p2.Minus(p1),
		edge2: p3.Minus(p2),
	}

	tri.normal = tri.edge1.Cross(tri.edge2).Unit()
	tri.centroid = (p1.Plus(p2).Plus(p3)).Scale(1.0 / 3.0)

	return tri
}

// Intersect calculates the ray-triangle intersection using Moller-Trumbore.
//
// https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/
func (tri *Triangle) Intersect(ray *geo.Ray) float64 {
	Calls++

	h := ray.Dir.Cross(tri.edge2)
	a := h.Dot(tri.edge1)
	if a > -0.0001 && a < 0.0001 {
		return -1 // ray parallel to triangle
	}

	f := 1 / a
	s := ray.Origin.Minus(tri.p1)
	u := f * s.Dot(h)
	if u < 0 || u > 1 {
		return -1
	}

	q := s.Cross(tri.edge1)
	v := f * q.Dot(ray.Dir)
	if v < 0 || u+v > 1 {
		return -1
	}

	return f * q.Dot(tri.edge2)
}
