package render

import (
	"fmt"
	"image/png"
	"os"
	"testing"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/spectrum"
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

func TestSomeSpectra(t *testing.T) {
	redSpec := spectrum.Sample(spectrum.Peak(675, 0.2))
	greenSpec := spectrum.Sample(spectrum.Peak(540, 0.2))
	blueSpec := spectrum.Sample(spectrum.Peak(465, 0.2))

	whiteSpec := redSpec.Plus(greenSpec.Plus(blueSpec))

	redCol := colorspace.SRGB.Convert(redSpec)
	greenCol := colorspace.SRGB.Convert(greenSpec)
	blueCol := colorspace.SRGB.Convert(blueSpec)

	whiteCol := colorspace.SRGB.Convert(whiteSpec)

	fmt.Println(redCol, greenCol, blueCol, whiteCol)
}
