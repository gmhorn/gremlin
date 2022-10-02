package primitive

import (
	"log"

	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/util"
)

type Sphere struct {
	Center geo.Vec
	Radius float64
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
func (s *Sphere) Intersect(ray *geo.Ray) float64 {
	L := ray.Origin.Minus(s.Center)

	a := ray.Dir.Dot(ray.Dir)
	b := 2 * L.Dot(geo.Vec(ray.Dir))
	c := L.Dot(L) - s.Radius*s.Radius

	t0, t1, found := util.SolveQuadratic(a, b, c)
	if !found {
		return -1.0
	}

	if t0 < 0 {
		return t1
	}
	return t0
}

func (s *Sphere) Normal(point geo.Vec) geo.Unit {
	n, valid := point.Minus(s.Center).Unit()
	if !valid {
		log.Printf("invalid sphere normal!")
		return geo.YAxis
	}
	return n
}
