package camera

import (
	"image"
	"testing"

	"github.com/gmhorn/gremlin/pkg/colorspace"
)

var img *image.RGBA

func BenchmarkFilm_Image(b *testing.B) {
	film := NewFilm(360, 240)
	for idx := range film.Pixels {
		film.Pixels[idx].Samples++
	}

	for i := 0; i < b.N; i++ {
		img = film.Image(colorspace.SRGB)
	}
}
