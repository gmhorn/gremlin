package render

import (
	"image/png"
	"os"
	"testing"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/stretchr/testify/assert"
)

func TestFixed(t *testing.T) {
	film := camera.NewFilm(640, 320)
	cam := camera.NewPerspective(film.AspectRatio, 75.0)

	err := Fixed(film, cam, nil)
	assert.NoError(t, err)

	file, err := os.Create("test.png")
	assert.NoError(t, err)
	defer file.Close()

	err = png.Encode(file, film.Image(colorspace.SRGB))
	assert.NoError(t, err)
}
