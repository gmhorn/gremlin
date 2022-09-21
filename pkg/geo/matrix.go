package geo

import "math"

var Identity = &Mtx{
	{1, 0, 0, 0},
	{0, 1, 0, 0},
	{0, 0, 1, 0},
	{0, 0, 0, 1},
}

// Matrix is a row-major, 4x4 "real-valued" (float64-valued) matrix.
// Right-handed coordinate system is assumed.
//
// Row-major keeps things simple, at the cost of possible cache misses during
// matrix multiplication. So mathematically a matrix is defined as:
//
//	a00 a01 a02 a03
//	a10 a11 a12 a13
//	a20 a21 a22 a23
//	a30 a31 a32 a33
//
// And Matrix indexing works similar:
//
//	var a Matrix
//	fmt.Println("a12:", a[1][2])
type Mtx [4][4]float64

// Clone returns a copy of this matrix.
func (a *Mtx) Clone() *Mtx {
	b := &Mtx{}
	copy(b[0][:], a[0][:])
	copy(b[1][:], a[1][:])
	copy(b[2][:], a[2][:])
	copy(b[3][:], a[3][:])
	return b
}

// Mult returns a new matrix that is the value of a*b.
func (a *Mtx) Mult(b *Mtx) *Mtx {
	// because we're row-major, this is straight from the mathematical
	// definition
	c := &Mtx{}
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			for k := 0; k < 4; k++ {
				c[i][j] += (a[i][k] * b[k][j])
			}
		}
	}
	return c
}

// MultPoint does a "point-like" vector multiplcation where v is extended to a
// homogeneous representation with 1 in the fourth coordinate. Essentially it
// performs a multiplication like:
//
//	a00 a01 a02 a03     v.x
//	a10 a11 a12 a12  x  v.y
//	a20 a21 a22 a21     v.z
//	a30 a31 a32 a33     1.0
//
// This is appropriate when the vector represents a point and we want to
// preserve any translation effects.
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#HomogeneousCoordinates
// https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/transforming-points-and-vectors
func (a *Mtx) MultPoint(v Vec) Vec {
	return Vec{
		a[0][0]*v[0] + a[0][1]*v[1] + a[0][2]*v[2] + a[0][3],
		a[1][0]*v[0] + a[1][1]*v[1] + a[1][2]*v[2] + a[1][3],
		a[2][0]*v[0] + a[2][1]*v[1] + a[2][2]*v[2] + a[2][3],
	}
}

// MultPoint does a "vector-like" vector multiplcation where v is extended to a
// homogeneous representation with 0 in the fourth coordinate. Essentially it
// performs a multiplication like:
//
//	a00 a01 a02 a03     v.x
//	a10 a11 a12 a12  x  v.y
//	a20 a21 a22 a21     v.z
//	a30 a31 a32 a33     0.0
//
// This is appropriate for vectors where we do not want to preserve any
// translation effects.
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#HomogeneousCoordinates
// https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/transforming-points-and-vectors
func (a *Mtx) MultVec(v Vec) Vec {
	return Vec{
		a[0][0]*v[0] + a[0][1]*v[1] + a[0][2]*v[2],
		a[1][0]*v[0] + a[1][1]*v[1] + a[1][2]*v[2],
		a[2][0]*v[0] + a[2][1]*v[1] + a[2][2]*v[2],
	}
}

// MultUnit is a convenience method. Its precisely equilent to
//
//	MultVec(Vec(u))
func (a *Mtx) MultUnit(u Unit) Vec {
	return a.MultVec(Vec(u))
}

// T returns a new matrix that is the transpose of this matrix.
func (a *Mtx) T() *Mtx {
	t := a.Clone()
	for i := 0; i < 4; i++ {
		for j := i + 1; j < 4; j++ {
			t[i][j], t[j][i] = t[j][i], t[i][j]
		}
	}
	return t
}

// Inv returns a new matrix that is the inverse of this matrix.
//
// Uses simple Gauss-Jordan elimination with partial pivoting.
// https://en.wikipedia.org/wiki/Gaussian_elimination
// https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry
func (a *Mtx) Inv() *Mtx {
	// Create augmented matrix
	m := [4][8]float64{}
	for i := 0; i < 4; i++ {
		copy(m[i][0:4], a[i][:])
		copy(m[i][4:], Identity[i][:])
	}

	// Forward substitute
	h, k := 0, 0
	for h < 4 && k < 8 {
		// Find k-th pivot
		pivot := findPivot(h, k, &m)
		// If no pivot in column, move to next column
		if m[pivot][k] == 0 {
			k++
			continue
		}
		// If pivot row is not current row, swap rows
		if pivot != h {
			m[h], m[pivot] = m[pivot], m[h]
		}

		// For all rows below the pivot...
		for i := h + 1; i < 4; i++ {
			f := m[i][k] / m[h][k]
			// Fill rest of column below pivot with 0
			m[i][k] = 0
			// Reduce all remaining elements in row
			for j := k + 1; j < 8; j++ {
				m[i][j] -= f * m[h][j]
			}
		}
		// increment pivot row and column
		h++
		k++
	}

	// Back substitute
	for i := 3; i >= 0; i-- {
		f := m[i][i]
		for j := 0; j < 8; j++ {
			m[i][j] /= f
		}

		for j := 0; j < i; j++ {
			f := m[j][i]

			for k := 0; k < 8; k++ {
				m[j][k] -= f * m[i][k]
			}
		}
	}

	aInv := &Mtx{}
	copy(aInv[0][:], m[0][4:])
	copy(aInv[1][:], m[1][4:])
	copy(aInv[2][:], m[2][4:])
	copy(aInv[3][:], m[3][4:])
	return aInv
}

func findPivot(h, k int, m *[4][8]float64) int {
	max := math.Abs(m[h][k])
	pivot := h
	for i := h + 1; i < 4; i++ {
		if math.Abs(m[i][k]) > max {
			max = math.Abs(m[i][k])
			pivot = i
		}
	}
	return pivot
}
