package geo

var Identity = &Matrix{
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
//	fmt.Println("a12", a[1][2])
type Matrix [4][4]float64

// Mult returns a new matrix that is the value of a*b.
func (a *Matrix) Mult(b *Matrix) *Matrix {
	// because we're row-major, this is straight from the mathematical
	// definition
	c := &Matrix{}
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
func (a *Matrix) MultPoint(v Vec) Vec {
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
func (a *Matrix) MultVec(v Vec) Vec {
	return Vec{
		a[0][0]*v[0] + a[0][1]*v[1] + a[0][2]*v[2],
		a[1][0]*v[0] + a[1][1]*v[1] + a[1][2]*v[2],
		a[2][0]*v[0] + a[2][1]*v[1] + a[2][2]*v[2],
	}
}

// MultUnit is a convenience method. Its precisely equilent to
//
//	MultVec(Vec(u))
func (a *Matrix) MultUnit(u Unit) Vec {
	return a.MultVec(Vec(u))
}

// T returns a new matrix that is the transpose of this matrix.
func (a *Matrix) T() *Matrix {
	t := &Matrix{}
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			t[i][j] = a[j][i]
		}
	}
	return t
}
