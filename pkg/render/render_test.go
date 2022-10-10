package render

import (
	"image/png"
	"os"
	"testing"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/stretchr/testify/assert"
)

func TestRender(t *testing.T) {
	film := camera.NewFilm(64, 32)
	Render(film, 16)

	file, err := os.Create("test.png")
	assert.NoError(t, err)
	defer file.Close()

	err = png.Encode(file, film.Image(colorspace.SRGB))
	assert.NoError(t, err)
}
