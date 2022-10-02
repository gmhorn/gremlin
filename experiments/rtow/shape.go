package main

import (
	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/util"
)

// Record of a ray hit. Pt is the location of the intersection and Norm is the
// normal. T is the distance to the intersection. Interior is true if Norm faces
// inside the object.
type Hit struct {
	Pt       geo.Vec
	Norm     geo.Unit
	T        float64
	Interior bool
}

// Interface for things that can be hit by rays
type Hittable interface {
	Hit(r *geo.Ray, tMin, tMax float64) (*Hit, bool)
}

// Aggregate is a list of Hittables, that itself supports the Hittable interface
type Aggregate []Hittable

func (a Aggregate) Hit(r *geo.Ray, tMin, tMax float64) (*Hit, bool) {
	var hit *Hit
	var hitAnything bool

	for _, obj := range a {
		if h, hitObj := obj.Hit(r, tMin, tMax); hitObj {
			hitAnything = true
			tMax = h.T
			hit = h
		}
	}

	return hit, hitAnything
}

type Sphere struct {
	Center geo.Vec
	Radius float64
}

func (s *Sphere) Hit(r *geo.Ray, tMin, tMax float64) (*Hit, bool) {
	L := r.Origin.Minus(s.Center)

	a := 1.0 // a = ||r.Dir||^2 == 1
	b := 2 * L.Dot(geo.Vec(r.Dir))
	c := L.Dot(L) - s.Radius*s.Radius

	t0, t1, found := util.SolveQuadratic(a, b, c)
	if !found {
		return nil, false
	}

	root := t0
	if root < tMin || root > tMax {
		root = t1
		if root < tMin || root > tMax {
			return nil, false
		}
	}

	// Calculate the intersection point and outward normal.
	point := r.At(root)
	norm, _ := point.Minus(s.Center).Unit()

	// Determine if ray is coming from inside the sphere
	interior := r.Dir.Enters(norm)
	// If it is, we need to flip the normal its towards the center of the sphere
	if interior {
		norm = norm.Reverse()
	}

	return &Hit{
		Pt:       point,
		Norm:     norm,
		T:        root,
		Interior: interior,
	}, true
}
