package geo

import "math"

// Shift returns a transform matrix that translates by the delta vector.
//
// Note that the inverse of Shift(v) is identical to the Shift of the complement
// of v:
//
//	Shift(v).Inv() == Shift(v.Scale(-1.0))
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Translations
func Shift(delta Vec) *Mtx {
	return &Mtx{
		{1, 0, 0, delta[0]},
		{0, 1, 0, delta[1]},
		{0, 0, 1, delta[2]},
		{0, 0, 0, 1},
	}
}

// Scale returns a scaling transform matrix.
//
// Note that the inverse of Scale(v) is equal to Scale(w) where the components
// w are the reciprocals of the components of v:
//
//	Scale(Vec{x, y, z}).Inv() == Scale(Vec{1.0/x, 1.0/y, 1.0/z})
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Scaling
func Scale(v Vec) *Mtx {
	return &Mtx{
		{v[0], 0, 0, 0},
		{0, v[1], 0, 0},
		{0, 0, v[2], 0},
		{0, 0, 0, 1},
	}
}

// Rotate returns a transform matrix representing rotation about the given axis.
//
// Note that the inverse of a rotation transform is equal to its transpose:
//
//	Rotate(axis).Inv() == Rotate(axis).T()
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#RotationaroundanArbitraryAxis
func Rotate(theta float64, axis Vec) *Mtx {
	mtx := Identity.Clone()
	axis, _ = axis.Normalize()

	sinTheta := math.Sin(theta)
	cosTheta := math.Cos(theta)
	// Rotation of first basis vector
	mtx[0][0] = axis[0]*axis[0] + (1-axis[0]*axis[0])*cosTheta
	mtx[0][1] = axis[0]*axis[1]*(1-cosTheta) - axis[2]*sinTheta
	mtx[0][2] = axis[0]*axis[2]*(1-cosTheta) + axis[1]*sinTheta
	mtx[0][3] = 0
	// Rotation of second basis vector
	mtx[1][0] = axis[0]*axis[1]*(1-cosTheta) + axis[2]*sinTheta
	mtx[1][1] = axis[1]*axis[1] + (1-axis[1]*axis[1])*cosTheta
	mtx[1][2] = axis[1]*axis[2]*(1-cosTheta) - axis[0]*sinTheta
	mtx[1][3] = 0
	// Rotation of third basis vector
	mtx[2][0] = axis[0]*axis[2]*(1-cosTheta) - axis[1]*sinTheta
	mtx[2][1] = axis[1]*axis[2]*(1-cosTheta) + axis[0]*sinTheta
	mtx[2][2] = axis[2]*axis[2] + (1-axis[2]*axis[2])*cosTheta
	mtx[2][3] = 0
	// Final row identical to identity matrix, so we're fine

	return mtx
}

// LookAt returns the view matrix that can translate from camera space to world
// space. All vectors given in world-view units. The from vector is the location
// of the camera, the to vector is the location where it's looking, and the up
// vector orients the camera along the viewing direction implied by from and to.
// Most of the time, up can just be the y-axis Vec{0, 1, 0}.
//
// See:
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#TheLook-AtTransformation
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#positionablecamera
//
// Note that PBRT uses a left-handed coordinate system, so their z axis is
// given by
//
//	to - from
//
// We use a right-handed system, so the arguments are reversed. Other than that,
// the PBRT and RTOW implementations are in agreement.
func LookAt(from, to, up Vec) *Mtx {
	zaxis, _ := from.Minus(to).Normalize()
	xaxis, _ := up.Cross(zaxis).Normalize()
	yaxis := zaxis.Cross(xaxis)

	return &Mtx{
		{xaxis[0], yaxis[0], zaxis[0], from[0]},
		{xaxis[1], yaxis[1], zaxis[1], from[1]},
		{xaxis[2], yaxis[2], zaxis[2], from[2]},
		{0, 0, 0, 1}}
}
