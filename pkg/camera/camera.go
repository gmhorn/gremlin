package camera

import (
	"github.com/gmhorn/gremlin/pkg/geo"
)

type Camera interface {
	Ray() *geo.Ray
}
