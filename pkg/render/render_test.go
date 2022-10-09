package render

import (
	"testing"
	"unsafe"

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
	width, height := 64, 32
	frame := Render(width, height, 16)
	err := OutputImage(width, height, frame, "test.png")
	assert.NoError(t, err)
}
