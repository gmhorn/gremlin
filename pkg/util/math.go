package util

import "math"

// SolveQuadratic finds the real roots, if they exist, of the quadratic
// equation
//
//	ax^2 + bx + c
//
// If roots are found, they are returned in ascending order.
func SolveQuadratic(a, b, c float64) (float64, float64, bool) {
	disc := b*b - 4*a*c
	if disc < 0 {
		return -1, -1, false
	}

	if disc == 0 {
		return -b / (2 * a), -b / (2 * a), true
	}

	q := -0.5 * (b + Sign(b)*math.Sqrt(disc))
	r0, r1 := q/a, c/q

	if r1 < r0 {
		r0, r1 = r1, r0
	}

	return r0, r1, true
}

// Sign returns -1.0 if n is negative, 1.0 if n is positive, and 0 if n is
// identically equal to 0
func Sign(n float64) float64 {
	switch {
	case n < 0:
		return -1.0
	case n > 0:
		return 1.0
	default:
		return 0.0
	}
}
