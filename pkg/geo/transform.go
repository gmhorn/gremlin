package geo

import (
	"log"
	"math"
)

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
		{1, 0, 0, delta.X},
		{0, 1, 0, delta.Y},
		{0, 0, 1, delta.Z},
		{0, 0, 0, 1},
	}
}

// Scale returns a scaling transform matrix.
//
// Note that the inverse of Scale(v) is equal to Scale(w) where the components
// w are the reciprocals of the components of v:
//
//	Scale(V(x, y, z)).Inv() == Scale(V(1.0/x, 1.0/y, 1.0/z))
//
// https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Scaling
func Scale(v Vec) *Mtx {
	return &Mtx{
		{v.X, 0, 0, 0},
		{0, v.Y, 0, 0},
		{0, 0, v.Z, 0},
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
func Rotate(theta float64, axis Unit) *Mtx {
	mtx := Identity.Clone()

	sinTheta := math.Sin(theta)
	cosTheta := math.Cos(theta)
	// Rotation of first basis vector
	mtx[0][0] = axis.X*axis.X + (1-axis.X*axis.X)*cosTheta
	mtx[0][1] = axis.X*axis.Y*(1-cosTheta) - axis.Z*sinTheta
	mtx[0][2] = axis.X*axis.Z*(1-cosTheta) + axis.Y*sinTheta
	mtx[0][3] = 0
	// Rotation of second basis vector
	mtx[1][0] = axis.Y*axis.X*(1-cosTheta) + axis.Z*sinTheta
	mtx[1][1] = axis.Y*axis.Y + (1-axis.Y*axis.Y)*cosTheta
	mtx[1][2] = axis.Y*axis.Z*(1-cosTheta) - axis.X*sinTheta
	mtx[1][3] = 0
	// Rotation of third basis vector
	mtx[2][0] = axis.Z*axis.X*(1-cosTheta) - axis.Y*sinTheta
	mtx[2][1] = axis.Z*axis.Y*(1-cosTheta) + axis.X*sinTheta
	mtx[2][2] = axis.Z*axis.Z + (1-axis.Z*axis.Z)*cosTheta
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
func LookAt(from, to Vec, up Unit) *Mtx {
	zaxis, ok := from.Minus(to).Unit()
	if !ok {
		log.Fatalln("LookAt transform cannot have identical from and to vectors:", from, to)
	}

	xaxis, ok := up.Cross(zaxis).Unit()
	if !ok {
		log.Fatalln("LookAt transform up vector cannot be perpendicular to from or to vectors:", from, to, up)
	}

	yaxis, ok := zaxis.Cross(xaxis).Unit()
	if !ok {
		log.Fatalln("LookAt transform failed to construct orthonormal basis:", from, to, up)
	}

	return &Mtx{
		{xaxis.X, yaxis.X, zaxis.X, from.X},
		{xaxis.Y, yaxis.Y, zaxis.Y, from.Y},
		{xaxis.Z, yaxis.Z, zaxis.Z, from.Z},
		{0, 0, 0, 1}}
}
