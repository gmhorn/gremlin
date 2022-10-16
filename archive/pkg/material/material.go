package material

import "github.com/gmhorn/gremlin/pkg/geo"

type Material interface {

	// Sample returns an exitant light ray, wo, at from and incident, wi, and
	// normal, n.
	//
	// Note that the returned ray will be slightly displaced along the normal to
	// avoid self-intersection from floating-point inaccuracies.
	Sample(point, wi geo.Vec, n geo.Unit, wavelength float64) (wo geo.Ray)
}
