package render

import (
	"image/png"
	"os"
	"testing"
	"unsafe"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/stretchr/testify/assert"
)

func TestPixel(t *testing.T) {
	p := Pixel{}
	s := unsafe.Sizeof(p)
	a := unsafe.Alignof(p)

	t.Log("Pixel size: ", s)
	t.Log("Pixel align:", a)
}

func TestRender(t *testing.T) {
	film := camera.NewFilm(64, 32)
	Render(film, 16)

	file, err := os.Create("test.png")
	assert.NoError(t, err)
	defer file.Close()

	err = png.Encode(file, film.Image(colorspace.SRGB))
	assert.NoError(t, err)
}
