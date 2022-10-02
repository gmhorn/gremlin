package camera

import (
	"math"

	"github.com/gmhorn/gremlin/pkg/geo"
)

// Perspective is one of the most basic camera models. It simulates a camera
// with vanishingly small aperture and no lense effects (e.g. all points in
// space are in focus).
//
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
type Perspective struct {
	aspectRatio float64
	tanHalfFOV  float64

	eye, target geo.Vec
	camToWorld  *geo.Mtx
}

// NewPerspective generates a new perspective camera. It is initialized at the
// global origin, facing in the negative-z direction ("into the page").
func NewPerspective(aspectRatio, fov float64) *Perspective {
	fov = (math.Pi * fov) / 180 // degree to radian
	c := &Perspective{
		aspectRatio: aspectRatio,
		tanHalfFOV:  math.Tan(fov * 0.5),
		eye:         geo.Origin,
		target:      geo.Vec{0, 0, -1},
	}

	c.recalculateLookMatrix()
	return c
}

// MoveTo shifts the camera to the given location.
func (c *Perspective) MoveTo(location geo.Vec) *Perspective {
	c.eye = location
	c.recalculateLookMatrix()
	return c
}

// PointAt repoints the camera to the given location.
func (c *Perspective) PointAt(location geo.Vec) *Perspective {
	c.target = location
	c.recalculateLookMatrix()
	return c
}

// Ray generates a ray from the normalized device coordinates (NDC) u and v.
//
// The NDC (u, v) of a specific pixel (x, y) is a function of the overall film
// size and sampling method used. For a film W pixels wide and H pixels high
//
//	u, v := x/W, y/H
//
// are the NDC coordinates of the upper-left corner of the pixel and
//
//	u, v := (x+0.5)/W, (y+0.5)/H
//
// are the NDC coordinates of the middle of the pixel.
//
// It's common to want to sample randomly within a pixel. In that case, use
//
//	u, v := (x+rand.Float64())/W, (y+rand(Float64())/H
//
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
func (c *Perspective) Ray(u, v float64) *geo.Ray {
	// In camera space, the camera is centered a the origin and facing down
	// the negative-z axis ("into the page"). The screen is centered one
	// unit down the z-axis at (0, 0, -1)
	//
	// With this, we can construct the ray vector fairly simply. The point on
	// the screen given by (u, v) is calculated below...
	p := geo.Vec{
		(2*u - 1) * c.aspectRatio * c.tanHalfFOV,
		(1 - 2*v) * c.tanHalfFOV,
		-1,
	}

	// ...and the direction is given by (p-camera_origin) == p-{0, 0, 0} == p
	//
	// All that remains is to convert that direction to world space.
	dir := c.camToWorld.MultVec(p)

	return geo.NewRay(c.eye, dir)
}

func (c *Perspective) recalculateLookMatrix() {
	c.camToWorld = geo.LookAt(c.eye, c.target, geo.YAxis)
}
