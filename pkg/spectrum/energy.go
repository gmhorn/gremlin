package spectrum

import (
	"image/color"
	"math"
)

const sRGB = 1.0 / 1.8

// Energy represents the sRGB red, green and blue radiance.
type Energy [3]float64

var White = Energy{1, 1, 1}
var Black = Energy{0, 0, 0}

func (e Energy) Scale(n float64) Energy {
	return Energy{e[0] * n, e[1] * n, e[2] * n}
}

func (e Energy) ToRGBA() color.RGBA {
	return color.RGBA{
		R: rgba(e[0]),
		G: rgba(e[1]),
		B: rgba(e[2]),
		A: 255,
	}
}
func rgba(val float64) uint8 {
	corrected := math.Pow(val/255, sRGB) * 255
	return uint8(math.Min(255, math.Max(0, corrected)))
}
