package main

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/gmhorn/gremlin/pkg/geo"
	"github.com/gmhorn/gremlin/pkg/shape"
)

var Tris = initTris(64)
var BVHNodes = make([]BVHNode, 0)
var RootNodeIdx = 0
var NodesUsed = 1

type Shape interface {
	Bounds() geo.Bounds
	Centroid() geo.Vec
}

type BVH struct{}

func NewBVH(shapes []Shape) *BVH {
	// Initialize primInfo array for primitives
	primInfo := make([]BVHPrimInfo, len(shapes))
	for i, s := range shapes {
		primInfo[i].primIdx = i
		primInfo[i].bounds = s.Bounds()
		primInfo[i].centroid = s.Centroid()
	}

	// Build BVH tree for primitives using primInfo
	orderedPrims := make([]Shape, len(shapes))

	// Compute representation of depth-first traversal of BVH tree

	return nil
}

type BVHNode struct {
	aabbMin, aabbMax      geo.Vec
	leftChild, rightChild uint
	firstPrim, primCount  int
}

type BVHPrimInfo struct {
	primIdx  int
	bounds   geo.Bounds
	centroid geo.Vec
}

func initTris(count int) []*shape.Triangle {
	start := time.Now()

	tris := make([]*shape.Triangle, 0)

	for i := 0; i < count; i++ {
		r0 := geo.V(rand.Float64(), rand.Float64(), rand.Float64())
		r1 := geo.V(rand.Float64(), rand.Float64(), rand.Float64())
		r2 := geo.V(rand.Float64(), rand.Float64(), rand.Float64())

		p0 := r0.Scale(9).Minus(geo.V(5, 5, 5))
		p1 := p0.Plus(r1)
		p2 := p0.Plus(r2)

		tris = append(tris, shape.NewTriangle(p0, p1, p2))
	}

	fmt.Printf("Created %d triangles in %s\n", count, time.Since(start))
	return tris
}

func BuildBVH() {
	root := BVHNodes[RootNodeIdx]
	root.leftChild = 0
	root.rightChild = 0
	root.firstPrim = 0
	root.primCount = len(Tris)
	UpdateNodeBounds(RootNodeIdx)
	Subdivide(RootNodeIdx)
}

func UpdateNodeBounds(nodeIdx int) {
	node := BVHNodes[nodeIdx]
	node.aabbMin = geo.V(1e30, 1e30, 1e30)
	node.aabbMax = geo.V(-1e30, -1e30, -1e30)
	for first, i := node.firstPrim, 0; i < node.primCount; i++ {
		leaf := Tris[first+i]
		node.aabbMin = geo.VecMin(node.aabbMin, leaf.P1)
		node.aabbMin = geo.VecMin(node.aabbMin, leaf.P2)
		node.aabbMin = geo.VecMin(node.aabbMin, leaf.P3)
		node.aabbMax = geo.VecMax(node.aabbMax, leaf.P1)
		node.aabbMax = geo.VecMax(node.aabbMax, leaf.P2)
		node.aabbMax = geo.VecMax(node.aabbMax, leaf.P3)
	}
}
