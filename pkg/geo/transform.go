package geo

// Shift returns a transform matrix that translates by the delta vector.
//
// Note that the inverse of Shift(v) is identical to the Shift of the complement
// of v:
//
//	Shift(v).Inv() == Shift(v.Scale(-1.0))
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Translations
func Shift(delta Vec) *Matrix {
	return &Matrix{
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
func Scale(v Vec) *Matrix {
	return &Matrix{
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
func Rotate(axis Vec) *Matrix {
	// TODO
	return Identity
}

// LookAt returns the view matrix that can translate from camera space to world
// space. All vectors given in world-view units. The from vector is the location
// of the camera, the to vector is the location where it's looking, and the up
// vector orients the camera along the viewing direction implied by from and to.
//
// See:
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#TheLook-AtTransformation
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#positionablecamera
//
// Note that PBRT uses a left-handed coordinate system, and thus their z axis is
// given by
//
//	to - from
//
// We use a right-handed system, so the arguments are reversed. Other than that,
// the PBRT and RTOW implementations are in agreement.
func LookAt(from, to Vec, up Unit) *Matrix {
	zaxis, _ := from.Minus(to).Unit()
	xaxis, _ := up.Cross(zaxis)
	yaxis, _ := zaxis.Cross(xaxis)

	return &Matrix{
		{xaxis[0], yaxis[0], zaxis[0], from[0]},
		{xaxis[1], yaxis[1], zaxis[1], from[1]},
		{xaxis[2], yaxis[2], zaxis[2], from[2]},
		{0, 0, 0, 1}}
}
