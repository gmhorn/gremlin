package render

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math/rand"
	"os"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/spectrum"
	"github.com/gmhorn/gremlin/pkg/util"
)

type FrameSample struct {
	Pixels []Pixel
	Offset int
}

type filmTile struct {
	pixels []camera.Pixel
	offset int
}

func Render(film *camera.Film, binSize int) {
	routines := 0
	tiles := make(chan filmTile)

	fmt.Println("Starting goroutines")
	for offset := 0; offset < len(film.Pixels); {
		size := util.IntMin(binSize, len(film.Pixels)-offset)

		go func(offset, size int) {
			tiles <- filmTile{
				pixels: renderTile(offset, size),
				offset: offset,
			}
		}(offset, size)

		routines++
		offset += size
	}

	fmt.Println("Awaiting goroutines")
	for i := 0; i < routines; i++ {
		tile := <-tiles
		fmt.Println("Filling pixels", tile.offset, "to", tile.offset+len(tile.pixels))
		film.Merge(tile.offset, tile.pixels)
	}
}

func renderTile(offset, size int) []camera.Pixel {
	fmt.Printf("Rendering tile, offset: %d, size: %d\n", offset, size)

	// Just pick a random color
	randSpec := spectrum.Peak((780-380)*rand.Float64()+380.0, 1.0)
	randColor := colorspace.CIE1931.Convert(randSpec)
	randColor = randColor.Scale(rand.Float64())

	pixels := make([]camera.Pixel, size)
	for i := 0; i < size; i++ {
		// TODO we'd use the real position using the offset

		pixels[i].Color = randColor
		pixels[i].Samples++
	}

	return pixels
}

func renderSample(offset, size int, c chan FrameSample) {
	fmt.Printf("Rendering sample, offset: %d, size: %d\n", offset, size)
	randSpec := spectrum.Peak((780-380)*rand.Float64()+380.0, 1.0)
	randColor := colorspace.SRGB.Convert(randSpec)
	randColor = randColor.Scale(rand.Float64())
	// randColor := colorspace.Point{
	// 	rand.Float64(),
	// 	rand.Float64(),
	// 	rand.Float64(),
	// }

	pixels := make([]Pixel, size)
	for i := 0; i < size; i++ {
		// TODO we'd calculate real position using offset, plus parent Width and
		// Height here

		pixels[i].Color = randColor
		pixels[i].Samples = 1
	}

	c <- FrameSample{Pixels: pixels, Offset: offset}
}

func OutputImage(width, height int, pixels []Pixel, name string) error {
	fmt.Println("Creating image")
	img := image.NewRGBA(image.Rect(0, 0, width, height))
	for t := range pixels {
		x := t % width
		y := t / width
		c := &color.RGBA{
			R: uint8(255.999 * (pixels[t].Color[0] / float64(pixels[t].Samples))),
			G: uint8(255.999 * (pixels[t].Color[1] / float64(pixels[t].Samples))),
			B: uint8(255.999 * (pixels[t].Color[2] / float64(pixels[t].Samples))),
			A: 255,
		}
		img.Set(x, y, c)
	}

	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()

	return png.Encode(file, img)
}
