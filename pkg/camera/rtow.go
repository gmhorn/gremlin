package camera

import (
	"math"

	"github.com/gmhorn/gremlin/pkg/geo"
)

// RTOW is a straight port of the camera from "Ray Tracing in One Weekend".
type RTOW struct {
	origin     geo.Vector
	lowerLeft  geo.Vector
	horizontal geo.Vector
	vertical   geo.Vector
}

// NewRTOW creates a new "Ray Tracing in One Weekend" type camera
func NewRTOW(from, to geo.Vector, up geo.Unit, fov, aspectRatio float64) *RTOW {
	theta := (fov * math.Pi) / 180 // degree-to-radian
	h := math.Tan(theta / 2)
	viewportH := 2 * h
	viewportW := aspectRatio * viewportH

	w, _ := from.Minus(to).Unit()
	u, _ := up.Cross(w)
	v, _ := w.Cross(u)

	origin := from
	horizontal := u.Scale(viewportW)
	vertical := v.Scale(viewportH)

	lowerLeft := origin.Minus(horizontal.Scale(0.5))
	lowerLeft = lowerLeft.Minus(vertical.Scale(0.5))
	lowerLeft = lowerLeft.Minus(geo.Vector(w))

	return &RTOW{
		origin:     origin,
		lowerLeft:  lowerLeft,
		horizontal: horizontal,
		vertical:   vertical,
	}
}

func (c *RTOW) Ray(u, v float64) *geo.Ray {
	scrn := c.lowerLeft
	scrn = scrn.Plus(c.horizontal.Scale(u))
	scrn = scrn.Plus(c.vertical.Scale(v))
	scrn = scrn.Minus(c.origin)

	dir, _ := scrn.Unit()

	return &geo.Ray{Origin: c.origin, Dir: dir}
}
