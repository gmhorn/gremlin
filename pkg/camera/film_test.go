package camera

import (
	"fmt"
	"image"
	"testing"

	"github.com/gmhorn/gremlin/pkg/colorspace"
)

var img *image.RGBA

func TestPixel_AddColor(t *testing.T) {
	pixels := make([]Pixel, 1)
	c := colorspace.Point{1, 2, 3}
	pixels[0].AddColor(c)

	fmt.Println("lol")
}

func BenchmarkFilm_Image(b *testing.B) {
	film := NewFilm(360, 240)
	for idx := range film.Pixels {
		film.Pixels[idx].Samples++
	}

	for i := 0; i < b.N; i++ {
		img = film.Image(colorspace.SRGB)
	}
}
