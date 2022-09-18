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

// LookAt returns the view matrix. Conceptually, the "eye" position is given by
// the from Vector and the target position is given by the to Vector.
// https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function
// https://www.3dgep.com/understanding-the-view-matrix/#Look_At_Camera
// https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxmatrixlookatrh
func LookAt(from, to Vec) *Matrix {
	zaxis, _ := from.Minus(to).Unit()
	xaxis, _ := YAxis.Cross(zaxis)
	yaxis, _ := zaxis.Cross(xaxis)

	trans := Vec{xaxis.Dot(from), yaxis.Dot(from), zaxis.Dot(from)}.Scale(-1)

	return &Matrix{
		{xaxis[0], xaxis[1], xaxis[2], 0},
		{yaxis[0], yaxis[1], yaxis[2], 0},
		{zaxis[0], zaxis[1], xaxis[2], 0},
		{trans[2], trans[1], trans[0], 1}}
}

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
