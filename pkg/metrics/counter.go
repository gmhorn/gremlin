package metrics

import (
	"math"
	"sync/atomic"
)

var RayIntersectionTestsSucceeded Count64
var RayIntersectionTestsFailed Count64

// Count64 is an unsigned integer metric which only increments
type Count64 uint64

// Inc increments the metric
func (c *Count64) Inc() {
	atomic.AddUint64((*uint64)(c), 1)
}

// Get retrieves the metric value
func (c *Count64) Get() uint64 {
	return atomic.LoadUint64((*uint64)(c))
}

// Quantity64 represents a floating-point metric quantity.
type Quantity64 uint64

// Inc increments the metric by the given value.
//
// Pretty much taken 100% from Prometheus's Gauge implementation
func (q *Quantity64) Inc(v float64) {
	for {
		oldBits := atomic.LoadUint64((*uint64)(q))
		newBits := math.Float64bits(math.Float64frombits(oldBits) + v)
		if atomic.CompareAndSwapUint64((*uint64)(q), oldBits, newBits) {
			return
		}
	}
}

// Get retrieves the metric value
func (q *Quantity64) Get() float64 {
	return math.Float64frombits(atomic.LoadUint64((*uint64)(q)))
}
